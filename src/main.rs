mod util;

use std::io::{self, Write};

use rspotify::spotify::client::Spotify;
use rspotify::spotify::model::artist::SimplifiedArtist;
use rspotify::spotify::model::device::DevicePayload;
use rspotify::spotify::model::offset::for_position;
use rspotify::spotify::model::page::Page;
use rspotify::spotify::model::playlist::{PlaylistTrack, SimplifiedPlaylist};
use rspotify::spotify::model::search::SearchTracks;
use rspotify::spotify::model::track::FullTrack;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::spotify::senum::Country;
use rspotify::spotify::util::get_token;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Row, SelectableList, Table, Text, Widget};
use tui::{Frame, Terminal};

use util::{Event, Events};

const LIMIT: u32 = 20;

#[derive(PartialEq)]
enum ActiveBlock {
    Input,
    Playlist,
    SongTable,
    HelpMenu,
    ApiError,
    SelectDevice,
}

struct App {
    active_block: ActiveBlock,
    devices: Option<DevicePayload>,
    device_id: Option<String>,
    current_playing_song: Option<FullTrack>,
    input: String,
    playlists: Option<Page<SimplifiedPlaylist>>,
    playlist_tracks: Vec<PlaylistTrack>,
    searched_tracks: Option<SearchTracks>,
    spotify: Option<Spotify>,
    songs_for_table: Vec<FullTrack>,
    selected_playlist_index: Option<usize>,
    select_song_index: usize,
    api_error: String,
    selected_device_index: Option<usize>,
}

impl App {
    fn new() -> App {
        App {
            active_block: ActiveBlock::Playlist,
            devices: None,
            device_id: None,
            api_error: String::new(),
            current_playing_song: None,
            input: String::new(),
            playlists: None,
            playlist_tracks: vec![],
            searched_tracks: None,
            spotify: None,
            songs_for_table: vec![],
            selected_playlist_index: None,
            select_song_index: 0,
            selected_device_index: None,
        }
    }
}

fn main() -> Result<(), failure::Error> {
    // Start authorization with spotify
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state user-read-playback-state user-read-private user-read-currently-playing playlist-read-private")
        .build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            // Terminal initialization
            let stdout = io::stdout().into_raw_mode()?;
            let stdout = MouseTerminal::from(stdout);
            let stdout = AlternateScreen::from(stdout);
            let backend = TermionBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;
            terminal.hide_cursor()?;

            let events = Events::new();

            // App
            let mut app = App::new();

            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();

            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();

            app.spotify = Some(spotify);

            if let Some(spotify) = &app.spotify {
                let playlists = spotify.current_user_playlists(LIMIT, None);

                match playlists {
                    Ok(p) => {
                        app.playlists = Some(p);
                    }
                    Err(e) => {
                        app.active_block = ActiveBlock::ApiError;
                        app.api_error = e.to_string();
                    }
                };

                let context = spotify.current_playing(None);
                if let Ok(ctx) = context {
                    if let Some(c) = ctx {
                        app.current_playing_song = c.item;
                    }
                };
            }

            loop {
                terminal.draw(|mut f| {
                    match app.active_block {
                        ActiveBlock::HelpMenu => {
                            draw_help_menu(&mut f);
                        }
                        ActiveBlock::ApiError => {
                            draw_api_error(&mut f, &app);
                        }
                        ActiveBlock::SelectDevice => {
                            draw_device_list(&mut f, &app);
                        }
                        _ => {
                            let parent_layout = Layout::default()
                                .direction(Direction::Vertical)
                                .constraints(
                                    [
                                        Constraint::Length(3),
                                        Constraint::Min(1),
                                        Constraint::Length(3),
                                    ]
                                    .as_ref(),
                                )
                                .margin(2)
                                .split(f.size());

                            // Search input and help
                            draw_input_and_help_box(&mut f, &app, parent_layout[0]);

                            // Playlist and song block
                            draw_main_layout(&mut f, &app, parent_layout[1]);

                            // Currently playing
                            draw_playing_block(&mut f, &app, parent_layout[2]);
                        }
                    }
                })?;

                // Put the cursor back inside the input box
                write!(
                    terminal.backend_mut(),
                    "{}",
                    Goto(4 + app.input.len() as u16, 4)
                )?;
                // stdout is buffered, flush it to see the effect immediately when hitting backspace
                io::stdout().flush().ok();

                if let Event::Input(key) = events.next()? {
                    // Match events for different app states
                    match app.active_block {
                        ActiveBlock::Input => match key {
                            Key::Char('q') | Key::Ctrl('c') => {
                                break;
                            }
                            Key::Ctrl('u') => {
                                if app.active_block == ActiveBlock::Input {
                                    app.input = String::new();
                                }
                            }
                            Key::Esc => {
                                if app.active_block == ActiveBlock::Input {
                                    app.active_block = ActiveBlock::Playlist;
                                }
                            }
                            Key::Char('\n') => {
                                if let Some(spotify) = &app.spotify {
                                    let result = spotify
                                        .search_track(
                                            &app.input,
                                            LIMIT,
                                            0,
                                            Some(Country::UnitedKingdom),
                                        )
                                        .expect("Failed to fetch spotify tracks");

                                    app.songs_for_table = result.tracks.items.clone();
                                    app.searched_tracks = Some(result);

                                    app.active_block = ActiveBlock::SongTable;
                                }
                            }
                            Key::Char(c) => {
                                if app.active_block == ActiveBlock::Input {
                                    app.input.push(c);
                                }
                            }
                            Key::Backspace => {
                                if app.active_block == ActiveBlock::Input {
                                    app.input.pop();
                                }
                            }
                            _ => {}
                        },
                        ActiveBlock::Playlist => match key {
                            Key::Char('q') | Key::Ctrl('c') => {
                                break;
                            }
                            Key::Char('d') => {
                                if let Some(spotify) = &app.spotify {
                                    match spotify.device() {
                                        Ok(devices) => {
                                            app.active_block = ActiveBlock::SelectDevice;
                                            app.devices = Some(devices);
                                        }

                                        Err(e) => {
                                            app.active_block = ActiveBlock::ApiError;
                                            app.api_error = e.to_string();
                                        }
                                    }
                                }
                            }
                            Key::Char('?') => {
                                app.active_block = ActiveBlock::HelpMenu;
                            }
                            Key::Right | Key::Char('l') => {
                                app.active_block = ActiveBlock::SongTable;
                            }
                            Key::Down | Key::Char('j') => match &app.playlists {
                                Some(p) => {
                                    if !p.items.is_empty() {
                                        app.selected_playlist_index =
                                            if let Some(selected_playlist_index) =
                                                app.selected_playlist_index
                                            {
                                                if selected_playlist_index >= p.items.len() - 1 {
                                                    Some(0)
                                                } else {
                                                    Some(selected_playlist_index + 1)
                                                }
                                            } else {
                                                Some(0)
                                            }
                                    }
                                }
                                None => (),
                            },
                            Key::Up | Key::Char('k') => match &app.playlists {
                                Some(p) => {
                                    if !p.items.is_empty() {
                                        app.selected_playlist_index =
                                            if let Some(selected_playlist_index) =
                                                app.selected_playlist_index
                                            {
                                                if selected_playlist_index > 0 {
                                                    Some(selected_playlist_index - 1)
                                                } else {
                                                    Some(p.items.len() - 1)
                                                }
                                            } else {
                                                Some(0)
                                            }
                                    }
                                }
                                None => (),
                            },
                            Key::Char('/') => {
                                app.active_block = ActiveBlock::Input;
                            }
                            Key::Char('\n') => {
                                if let Some(spotify) = &app.spotify {
                                    if let Some(playlists) = &app.playlists {
                                        if let Some(selected_playlist_index) =
                                            app.selected_playlist_index
                                        {
                                            if let Some(selected_playlist) =
                                                playlists.items.get(selected_playlist_index)
                                            {
                                                let playlist_id = &selected_playlist.id;
                                                if let Ok(playlist_tracks) = spotify
                                                    .user_playlist_tracks(
                                                        "spotify",
                                                        &playlist_id,
                                                        None,
                                                        Some(LIMIT),
                                                        None,
                                                        None,
                                                    )
                                                {
                                                    app.songs_for_table = playlist_tracks
                                                        .items
                                                        .clone()
                                                        .into_iter()
                                                        .map(|item| item.track)
                                                        .collect::<Vec<FullTrack>>();

                                                    app.playlist_tracks = playlist_tracks.items;
                                                    app.active_block = ActiveBlock::SongTable;
                                                };
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        },
                        ActiveBlock::SongTable => match key {
                            Key::Char('q') | Key::Ctrl('c') => {
                                break;
                            }
                            Key::Char('d') => {
                                if let Some(spotify) = &app.spotify {
                                    if let Ok(devices) = spotify.device() {
                                        app.active_block = ActiveBlock::SelectDevice;
                                        app.devices = Some(devices);
                                    }
                                }
                            }
                            Key::Left | Key::Char('h') => {
                                app.active_block = ActiveBlock::Playlist;
                            }
                            Key::Down | Key::Char('j') => {
                                if !app.songs_for_table.is_empty() {
                                    app.select_song_index += 1;
                                    if app.select_song_index > app.songs_for_table.len() - 1 {
                                        app.select_song_index = 0;
                                    }
                                }
                            }
                            Key::Char('?') => {
                                app.active_block = ActiveBlock::HelpMenu;
                            }
                            Key::Up | Key::Char('k') => {
                                if !app.songs_for_table.is_empty() {
                                    if app.select_song_index > 0 {
                                        app.select_song_index -= 1;
                                    } else {
                                        app.select_song_index = app.songs_for_table.len() - 1;
                                    }
                                }
                            }
                            Key::Char('/') => {
                                app.active_block = ActiveBlock::Input;
                            }
                            Key::Char('\n') => {
                                if let Some(track) = app.songs_for_table.get(app.select_song_index)
                                {
                                    let context_uri =
                                        match (&app.selected_playlist_index, &app.playlists) {
                                            (Some(selected_playlist_index), Some(playlists)) => {
                                                if let Some(selected_playlist) = playlists
                                                    .items
                                                    .get(selected_playlist_index.to_owned())
                                                {
                                                    Some(selected_playlist.uri.to_owned())
                                                } else {
                                                    None
                                                }
                                            }
                                            _ => None,
                                        };

                                    let device_id = app.device_id.take();

                                    // I need to pass in different arguments here, how can this be
                                    // nicer?
                                    if let Some(spotify) = &app.spotify {
                                        if let Some(context_uri) = context_uri {
                                            match spotify.start_playback(
                                                device_id,
                                                Some(context_uri),
                                                None,
                                                for_position(app.select_song_index as u32),
                                            ) {
                                                Ok(_r) => {
                                                    app.current_playing_song =
                                                        Some(track.to_owned());
                                                }
                                                Err(e) => {
                                                    app.active_block = ActiveBlock::ApiError;
                                                    app.api_error = e.to_string();
                                                }
                                            }
                                        } else {
                                            match spotify.start_playback(
                                                device_id,
                                                None,
                                                Some(vec![track.uri.to_owned()]),
                                                for_position(0),
                                            ) {
                                                Ok(_r) => {
                                                    app.current_playing_song =
                                                        Some(track.to_owned());
                                                }
                                                Err(e) => {
                                                    app.active_block = ActiveBlock::ApiError;
                                                    app.api_error = e.to_string();
                                                }
                                            }
                                        }
                                    }
                                };
                            }
                            _ => {}
                        },
                        ActiveBlock::HelpMenu => match key {
                            Key::Char('q') | Key::Ctrl('c') => {
                                break;
                            }
                            Key::Esc => {
                                app.active_block = ActiveBlock::Playlist;
                            }
                            _ => {}
                        },
                        ActiveBlock::ApiError => match key {
                            Key::Char('q') | Key::Ctrl('c') => {
                                break;
                            }
                            Key::Esc => {
                                app.active_block = ActiveBlock::Playlist;
                            }
                            _ => {}
                        },
                        ActiveBlock::SelectDevice => match key {
                            Key::Char('q') | Key::Ctrl('c') => {
                                break;
                            }
                            Key::Esc => {
                                app.active_block = ActiveBlock::Playlist;
                            }
                            Key::Down | Key::Char('j') => match &app.devices {
                                Some(p) => {
                                    if !p.devices.is_empty() {
                                        app.selected_device_index =
                                            if let Some(selected_device_index) =
                                                app.selected_device_index
                                            {
                                                if selected_device_index >= p.devices.len() - 1 {
                                                    Some(0)
                                                } else {
                                                    Some(selected_device_index + 1)
                                                }
                                            } else {
                                                Some(0)
                                            }
                                    }
                                }
                                None => (),
                            },
                            Key::Up | Key::Char('k') => match &app.devices {
                                Some(p) => {
                                    if !p.devices.is_empty() {
                                        app.selected_device_index =
                                            if let Some(selected_device_index) =
                                                app.selected_device_index
                                            {
                                                if selected_device_index > 0 {
                                                    Some(selected_device_index - 1)
                                                } else {
                                                    Some(p.devices.len() - 1)
                                                }
                                            } else {
                                                Some(0)
                                            }
                                    }
                                }
                                None => (),
                            },
                            Key::Char('\n') => match (&app.devices, app.selected_device_index) {
                                (Some(devices), Some(index)) => {
                                    if let Some(device) = devices.devices.get(index) {
                                        app.device_id = Some(device.id.to_owned());
                                        app.active_block = ActiveBlock::Playlist;
                                    }
                                }
                                _ => (),
                            },
                            _ => {}
                        },
                    }
                }
            }
        }
        None => println!("Spotify auth failed"),
    }

    Ok(())
}

fn get_color(active_block: &ActiveBlock, block_to_match: ActiveBlock) -> Style {
    if *active_block == block_to_match {
        Style::default().fg(Color::LightCyan)
    } else {
        Style::default().fg(Color::Gray)
    }
}

fn create_artist_string(artists: &[SimplifiedArtist]) -> String {
    artists
        .iter()
        .map(|artist| artist.name.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

fn display_songs(track_search_results: &[FullTrack]) -> Vec<Vec<String>> {
    track_search_results
        .iter()
        .map(|item| {
            vec![
                item.name.to_owned(),
                create_artist_string(&item.artists),
                item.album.name.to_owned(),
            ]
        })
        .collect()
}

fn draw_help_menu<B>(f: &mut Frame<B>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(2)
        .split(f.size());

    let white = Style::default().fg(Color::White);
    let gray = Style::default().fg(Color::White);
    let header = ["Active block", "Event", "Description"];

    // Would be nice to share the same source of truth as the event match below
    let help_rows = vec![
        vec!["Playlist/Song block", "j", "Move selection down"],
        vec!["Playlist/Song blocks", "k", "Move selection up"],
        vec!["General", "/", "Enter input for search"],
        vec!["General", "q", "Quit"],
        vec!["General", "<ctrl+c>", "Quit"],
        vec!["General", "<Esc>", "Go back"],
        vec!["General", "d", "Select device to play music on"],
        vec!["Input", "<ctrl+u>", "Delete input"],
        vec!["Input", "Enter", "Search with input text"],
    ];

    let rows = help_rows
        .into_iter()
        .map(|item| Row::StyledData(item.into_iter(), gray));

    Table::new(header.iter(), rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(white)
                .title("Help")
                .title_style(gray)
                .border_style(gray),
        )
        .style(Style::default().fg(Color::White))
        .widths(&[30, 30, 30])
        .render(f, chunks[0]);
}

fn draw_input_and_help_box<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(layout_chunk);

    Paragraph::new([Text::raw(&app.input)].iter())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Input")
                .title_style(get_color(&app.active_block, ActiveBlock::Input))
                .border_style(get_color(&app.active_block, ActiveBlock::Input)),
        )
        .render(f, chunks[0]);

    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray))
        .title_style(Style::default().fg(Color::Gray));

    Paragraph::new([Text::raw("Type ?")].iter())
        .block(block)
        .style(Style::default().fg(Color::Gray))
        .render(f, chunks[1]);
}

fn draw_main_layout<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    let playlist_items = match &app.playlists {
        Some(p) => p.items.iter().map(|item| item.name.to_owned()).collect(),
        None => vec![],
    };

    SelectableList::default()
        .block(
            Block::default()
                .title("Playlists")
                .borders(Borders::ALL)
                .title_style(get_color(&app.active_block, ActiveBlock::Playlist))
                .border_style(get_color(&app.active_block, ActiveBlock::Playlist)),
        )
        .items(&playlist_items)
        .style(Style::default().fg(Color::White))
        .select(app.selected_playlist_index)
        .highlight_style(
            Style::default()
                .fg(Color::LightCyan)
                .modifier(Modifier::BOLD),
        )
        .render(f, chunks[0]);

    let normal_style = Style::default().fg(Color::White);
    let header = ["Title", "Artist", "Album"];

    let formatted_songs = display_songs(&app.songs_for_table);

    let selected_style = Style::default()
        .fg(Color::LightCyan)
        .modifier(Modifier::BOLD);

    let selected_song_index = app.select_song_index;
    let rows = formatted_songs.into_iter().enumerate().map(|(i, item)| {
        if i == selected_song_index {
            Row::StyledData(item.into_iter(), selected_style)
        } else {
            Row::StyledData(item.into_iter(), normal_style)
        }
    });

    Table::new(header.iter(), rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Songs")
                .title_style(get_color(&app.active_block, ActiveBlock::SongTable))
                .border_style(get_color(&app.active_block, ActiveBlock::SongTable)),
        )
        .style(Style::default().fg(Color::White))
        .widths(&[40, 40, 40])
        .render(f, chunks[1]);
}

fn draw_playing_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(layout_chunk);

    let playing_text = match &app.current_playing_song {
        Some(s) => [
            Text::styled(&s.name, Style::default().fg(Color::Magenta)),
            Text::raw(" - "),
            Text::styled(
                create_artist_string(&s.artists),
                Style::default().fg(Color::White),
            ),
        ],
        None => [Text::raw(""), Text::raw(""), Text::raw("")],
    };

    Paragraph::new(playing_text.iter())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Playing")
                .title_style(Style::default().fg(Color::Gray))
                .border_style(Style::default().fg(Color::Gray)),
        )
        .render(f, chunks[0]);
}

fn draw_api_error<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(10)
        .split(f.size());

    let playing_text = [
        Text::raw("Api response: "),
        Text::styled(&app.api_error, Style::default().fg(Color::LightRed)),
        Text::styled("\nPress <Esc> to return", Style::default().fg(Color::White)),
    ];

    Paragraph::new(playing_text.iter())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Error")
                .title_style(Style::default().fg(Color::Red))
                .border_style(Style::default().fg(Color::Red)),
        )
        .render(f, chunks[0]);
}

fn draw_device_list<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(10)
        .split(f.size());

    let no_device_message = vec!["No devices found: Make sure a device has is active".to_string()];
    let items = match &app.devices {
        Some(items) => {
            if items.devices.is_empty() {
                no_device_message
            } else {
                items
                    .devices
                    .iter()
                    .map(|device| device.name.to_owned())
                    .collect()
            }
        }
        None => no_device_message,
    };

    SelectableList::default()
        .block(
            Block::default()
                .title("Devices")
                .borders(Borders::ALL)
                .title_style(Style::default().fg(Color::LightCyan))
                .border_style(Style::default().fg(Color::Gray)),
        )
        .items(&items)
        .style(Style::default().fg(Color::White))
        .select(app.selected_device_index)
        .highlight_style(
            Style::default()
                .fg(Color::LightCyan)
                .modifier(Modifier::BOLD),
        )
        .render(f, chunks[0]);
}
