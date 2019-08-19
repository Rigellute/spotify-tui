mod util;

use std::io::{self, Write};

use rspotify::spotify::client::Spotify;
use rspotify::spotify::model::artist::SimplifiedArtist;
use rspotify::spotify::model::offset::for_position;
use rspotify::spotify::model::search::SearchTracks;
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

#[derive(PartialEq)]
enum ActiveBlock {
    Input,
    Playlist,
    SongTable,
}

struct App {
    // TODO: figure out how to store the actual response `SearchTracks`
    songs: Vec<Vec<String>>,
    song_ids: Vec<String>,
    input: String,
    selected_song: usize,
    selected_playlist: Option<usize>,
    active_block: ActiveBlock,
    playlists: Vec<String>,
}

impl App {
    fn new() -> App {
        App {
            input: String::new(),
            playlists: vec![],
            songs: vec![],
            song_ids: vec![],
            selected_song: 0,
            selected_playlist: None,
            active_block: ActiveBlock::Playlist,
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

            app.playlists = playlists
                .unwrap()
                .items
                .iter()
                .map(|playlist| playlist.name.to_owned())
                .collect();

            loop {
                terminal.draw(|mut f| {
                    let selected_style = Style::default().fg(Color::Cyan).modifier(Modifier::BOLD);
                    let normal_style = Style::default().fg(Color::White);
                    let header = ["Title", "Artist", "Album"];
                    let rows = app.songs.iter().enumerate().map(|(i, item)| {
                        if i == app.selected_song {
                            Row::StyledData(item.iter(), selected_style)
                        } else {
                            Row::StyledData(item.iter(), normal_style)
                        }
                    });

                    let size = f.size();

                    let parent_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                        .margin(2)
                        .split(size);

                    {
                        let chunks = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([Constraint::Percentage(100)].as_ref())
                            .split(parent_layout[0]);

                        Paragraph::new([Text::raw(&app.input)].iter())
                            .style(Style::default().fg(Color::Yellow))
                            .block(
                                Block::default()
                                    .borders(Borders::ALL)
                                    .title("Input")
                                    .border_style(Style::default().fg(get_border_color(
                                        &app.active_block,
                                        ActiveBlock::Input,
                                    ))),
                            )
                            .render(&mut f, chunks[0]);
                    }

                    // Playlist and song block
                    {
                        let chunks = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints(
                                [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                            )
                            .split(parent_layout[1]);

                        SelectableList::default()
                            .block(
                                Block::default()
                                    .title("Playlists")
                                    .borders(Borders::ALL)
                                    .border_style(Style::default().fg(get_border_color(
                                        &app.active_block,
                                        ActiveBlock::Playlist,
                                    ))),
                            )
                            .items(&app.playlists)
                            .select(app.selected_playlist)
                            .highlight_style(
                                Style::default()
                                    .fg(Color::LightGreen)
                                    .modifier(Modifier::BOLD),
                            )
                            .highlight_symbol(">")
                            .render(&mut f, chunks[0]);

                        Table::new(header.iter(), rows)
                            .block(
                                Block::default()
                                    .borders(Borders::ALL)
                                    .title("Songs")
                                    .border_style(Style::default().fg(get_border_color(
                                        &app.active_block,
                                        ActiveBlock::SongTable,
                                    ))),
                            )
                            .widths(&[40, 40, 40])
                            .render(&mut f, chunks[1]);
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
                    match key {
                        Key::Char('q') | Key::Ctrl('c') => {
                            break;
                        }
                        Key::Char('/') => {
                            app.active_block = ActiveBlock::Input;
                        }
                        Key::Ctrl('u') => {
                            if app.active_block == ActiveBlock::Input {
                                app.input = String::new();
                            }
                        }
                        Key::Left | Key::Char('h') => match app.active_block {
                            ActiveBlock::Playlist => {} // Could wrap around and go to SongTable, like tmux?
                            ActiveBlock::SongTable => {
                                app.active_block = ActiveBlock::Playlist;
                            }
                            ActiveBlock::Input => app.input.push('h'),
                        },
                        Key::Right | Key::Char('l') => match app.active_block {
                            ActiveBlock::Playlist => {
                                app.active_block = ActiveBlock::SongTable;
                            }
                            ActiveBlock::SongTable => {}
                            ActiveBlock::Input => app.input.push('l'),
                        },
                        Key::Down | Key::Char('j') => match app.active_block {
                            ActiveBlock::Playlist => {
                                app.selected_playlist =
                                    if let Some(selected_playlist) = app.selected_playlist {
                                        if selected_playlist >= app.playlists.len() - 1 {
                                            Some(0)
                                        } else {
                                            Some(selected_playlist + 1)
                                        }
                                    } else {
                                        Some(0)
                                    }
                            }
                            ActiveBlock::SongTable => {
                                app.selected_song += 1;
                                if app.selected_song > app.songs.len() - 1 {
                                    app.selected_song = 0;
                                }
                            }
                            ActiveBlock::Input => {
                                // NOTE: this will also type `j` if the user presses the down arrow!
                                app.input.push('j');
                            }
                        },
                        Key::Up | Key::Char('k') => match app.active_block {
                            ActiveBlock::Playlist => {
                                app.selected_playlist =
                                    if let Some(selected_playlist) = app.selected_playlist {
                                        if selected_playlist > 0 {
                                            Some(selected_playlist - 1)
                                        } else {
                                            Some(app.songs.len() - 1)
                                        }
                                    } else {
                                        Some(0)
                                    }
                            }
                            ActiveBlock::SongTable => {
                                if app.selected_song > 0 {
                                    app.selected_song -= 1;
                                } else {
                                    app.selected_song = app.songs.len() - 1;
                                }
                            }
                            ActiveBlock::Input => {
                                // NOTE: this will also type `k` if the user presses the down arrow!
                                app.input.push('k')
                            }
                        },
                        Key::Char('\n') => match app.active_block {
                            ActiveBlock::Input => {
                                let result = spotify
                                    .search_track(&app.input, 20, 0, Some(Country::UnitedKingdom))
                                    .expect("Failed to fetch spotify tracks");

                                app.songs = display_songs(&result);
                                app.song_ids = result
                                    .tracks
                                    .items
                                    .iter()
                                    .map(|item| item.uri.to_owned())
                                    .collect();
                                app.active_block = ActiveBlock::SongTable;
                            }
                            ActiveBlock::Playlist => {}
                            ActiveBlock::SongTable => {
                                if let Some(uri) = app.song_ids.get(app.selected_song) {
                                    spotify
                                        .start_playback(
                                            Some(device_id.to_owned()),
                                            None,
                                            Some(vec![uri.to_owned()]),
                                            for_position(0),
                                        )
                                        // TODO: handle playback errors
                                        .unwrap();
                                };

;                            }
                        },
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
                    }
                }
            }
        }
        None => println!("Auth failed"),
    }

    Ok(())
}

fn get_border_color(active_block: &ActiveBlock, block_to_match: ActiveBlock) -> Color {
    if *active_block == block_to_match {
        Color::Green
    } else {
        Color::White
    }
}

fn create_artist_string(artists: &[SimplifiedArtist]) -> String {
    artists
        .iter()
        .fold("".to_string(), |artist_string, artist| {
            artist_string + &artist.name
        })
}

fn display_songs(track_search_results: &SearchTracks) -> Vec<Vec<String>> {
    track_search_results
        .tracks
        .items
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
