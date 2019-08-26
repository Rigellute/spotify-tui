mod app;
mod handlers;
mod ui;
mod util;

use std::io::{self, Write};

use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::spotify::util::get_token;
use termion::cursor::Goto;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;

use app::{ActiveBlock, App, EventLoop, LIMIT};
use util::{Event, Events};

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
                            ui::draw_help_menu(&mut f);
                        }
                        ActiveBlock::ApiError => {
                            ui::draw_api_error(&mut f, &app);
                        }
                        ActiveBlock::SelectDevice => {
                            ui::draw_device_list(&mut f, &app);
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
                            ui::draw_input_and_help_box(&mut f, &app, parent_layout[0]);

                            // Playlist and song block
                            ui::draw_main_layout(&mut f, &app, parent_layout[1]);

                            // Currently playing
                            ui::draw_playing_block(&mut f, &app, parent_layout[2]);
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
                        ActiveBlock::Input => {
                            if let Some(event) = handlers::input_handler(key, &mut app) {
                                if event == EventLoop::Exit {
                                    break;
                                }
                            }
                        }
                        ActiveBlock::Playlist => {
                            if let Some(event) = handlers::playlist_handler(key, &mut app) {
                                if event == EventLoop::Exit {
                                    break;
                                }
                            }
                        }
                        ActiveBlock::SongTable => {
                            if let Some(event) = handlers::song_table_handler(key, &mut app) {
                                if event == EventLoop::Exit {
                                    break;
                                }
                            }
                        }
                        ActiveBlock::HelpMenu => {
                            if let Some(event) = handlers::help_menu_handler(key, &mut app) {
                                if event == EventLoop::Exit {
                                    break;
                                }
                            }
                        }
                        ActiveBlock::ApiError => {
                            if let Some(event) = handlers::api_error_menu_handler(key, &mut app) {
                                if event == EventLoop::Exit {
                                    break;
                                }
                            }
                        }
                        ActiveBlock::SelectDevice => {
                            if let Some(event) = handlers::select_device_handler(key, &mut app) {
                                if event == EventLoop::Exit {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        None => println!("Spotify auth failed"),
    }

    Ok(())
}
