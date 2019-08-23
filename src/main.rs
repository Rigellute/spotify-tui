mod util;

use std::io::{self, Write};

use rspotify::spotify::client::Spotify;
use rspotify::spotify::model::artist::SimplifiedArtist;
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
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Row, SelectableList, Table, Text, Widget};
use tui::Terminal;

use util::{Event, Events};

const LIMIT: u32 = 20;

#[derive(PartialEq)]
enum ActiveBlock {
    Input,
    Playlist,
    SongTable,
    HelpMenu,
}

struct App {
    active_block: ActiveBlock,
    current_playing_song: Option<FullTrack>,
    input: String,
    playlists: Option<Page<SimplifiedPlaylist>>,
    playlist_tracks: Vec<PlaylistTrack>,
    searched_tracks: Option<SearchTracks>,
    songs_for_table: Vec<FullTrack>,
    selected_playlist_index: Option<usize>,
    select_song_index: usize,
}

impl App {
    fn new() -> App {
        App {
            active_block: ActiveBlock::Playlist,
            current_playing_song: None,
            input: String::new(),
            playlists: None,
            playlist_tracks: vec![],
            searched_tracks: None,
            songs_for_table: vec![],
            selected_playlist_index: None,
            select_song_index: 0,
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

            // TODO: Create a step for selecting which device to play
            let devices = spotify.device();
            let device_id = String::from("2577b0ea0b00e3d2c0d276d8f9629dde8645e3d8");

            let playlists = spotify.current_user_playlists(10, None);

            match playlists {
                Ok(p) => {
                    app.playlists = Some(p);
                }
                // TODO: do something when there is an error
                Err(_e) => (),
            };

            let context = spotify.current_playing(None);
            if let Ok(ctx) = context {
                if let Some(c) = ctx {
                    app.current_playing_song = c.item;
                }
            };

            loop {
                terminal.draw(|mut f| {
                    if app.active_block == ActiveBlock::HelpMenu {
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
                            vec!["General", "q", "Go back/quit"],
                            vec!["General", "<ctrl+c>", "Quit"],
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
                            .render(&mut f, chunks[0]);
                    } else {
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
                        {
                            let chunks = Layout::default()
                                .direction(Direction::Horizontal)
                                .constraints(
                                    [Constraint::Percentage(90), Constraint::Percentage(10)]
                                        .as_ref(),
                                )
                                .split(parent_layout[0]);

                            Paragraph::new([Text::raw(&app.input)].iter())
                                .style(Style::default().fg(Color::Yellow))
                                .block(
                                    Block::default()
                                        .borders(Borders::ALL)
                                        .title("Input")
                                        .title_style(get_color(
                                            &app.active_block,
                                            ActiveBlock::Input,
                                        ))
                                        .border_style(get_color(
                                            &app.active_block,
                                            ActiveBlock::Input,
                                        )),
                                )
                                .render(&mut f, chunks[0]);

                            let block = Block::default()
                                .title("Help")
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(Color::Gray))
                                .title_style(Style::default().fg(Color::Gray));

                            Paragraph::new([Text::raw("Type ?")].iter())
                                .block(block)
                                .style(Style::default().fg(Color::Gray))
                                .render(&mut f, chunks[1]);
                        }

                        // Playlist and song block
                        {
                            let chunks = Layout::default()
                                .direction(Direction::Horizontal)
                                .constraints(
                                    [Constraint::Percentage(25), Constraint::Percentage(75)]
                                        .as_ref(),
                                )
                                .split(parent_layout[1]);

                            let playlist_items = match &app.playlists {
                                Some(p) => {
                                    p.items.iter().map(|item| item.name.to_owned()).collect()
                                }
                                None => vec![],
                            };

                            SelectableList::default()
                                .block(
                                    Block::default()
                                        .title("Playlists")
                                        .borders(Borders::ALL)
                                        .title_style(get_color(
                                            &app.active_block,
                                            ActiveBlock::Playlist,
                                        ))
                                        .border_style(get_color(
                                            &app.active_block,
                                            ActiveBlock::Playlist,
                                        )),
                                )
                                .items(&playlist_items)
                                .style(Style::default().fg(Color::White))
                                .select(app.selected_playlist_index)
                                .highlight_style(
                                    Style::default()
                                        .fg(Color::LightCyan)
                                        .modifier(Modifier::BOLD),
                                )
                                .render(&mut f, chunks[0]);

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
                                        .title_style(get_color(
                                            &app.active_block,
                                            ActiveBlock::SongTable,
                                        ))
                                        .border_style(get_color(
                                            &app.active_block,
                                            ActiveBlock::SongTable,
                                        )),
                                )
                                .style(Style::default().fg(Color::White))
                                .widths(&[40, 40, 40])
                                .render(&mut f, chunks[1]);
                        }

                        {
                            let chunks = Layout::default()
                                .direction(Direction::Horizontal)
                                .constraints([Constraint::Percentage(100)].as_ref())
                                .split(parent_layout[2]);

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
                                .render(&mut f, chunks[0]);
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
                                            };
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
                                    spotify
                                        .start_playback(
                                            Some(device_id.to_owned()),
                                            None,
                                            Some(vec![track.uri.to_owned()]),
                                            for_position(0),
                                        )
                                        // TODO: handle playback errors
                                        .unwrap();
                                };
                            }
                            _ => {}
                        },
                        ActiveBlock::HelpMenu => match key {
                            Key::Ctrl('c') => {
                                break;
                            }
                            Key::Char('q') | Key::Esc => {
                                app.active_block = ActiveBlock::Playlist;
                            }
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
        .fold("".to_string(), |artist_string, artist| {
            artist_string + &artist.name
        })
}

fn display_songs(track_search_results: &Vec<FullTrack>) -> Vec<Vec<String>> {
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
