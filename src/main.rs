mod app;
mod handlers;
mod ui;
mod util;

use std::io::{self, Write};

use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::spotify::util::get_token;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;

use app::{ActiveBlock, App};
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

            // Now that spotify is ready, check if the user has already selected a device_id to
            // play music on, if not send them to the device selection view
            match app.get_cached_device_token() {
                Ok(device_id) => {
                    app.device_id = Some(device_id);
                }
                Err(e) => {
                    app.handle_get_devices();
                }
            }

            if let Some(spotify) = &app.spotify {
                let playlists = spotify.current_user_playlists(app.large_search_limit, None);

                match playlists {
                    Ok(p) => {
                        app.playlists = Some(p);
                        // Select the first playlist
                        app.selected_playlist_index = Some(0);
                    }
                    Err(e) => {
                        app.active_block = ActiveBlock::Error;
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
                        ActiveBlock::Error => {
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

                if app.active_block == ActiveBlock::Input {
                    match terminal.show_cursor() {
                        Ok(_r) => {}
                        Err(_e) => {}
                    };
                } else {
                    match terminal.hide_cursor() {
                        Ok(_r) => {}
                        Err(_e) => {}
                    };
                }

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
                        Key::Char('q') | Key::Ctrl('c') => break,
                        _ => handle_app(&mut app, key),
                    }
                }
            }
        }
        None => println!("\nSpotify auth failed"),
    }

    Ok(())
}

fn handle_app(app: &mut App, key: Key) {
    // Match events for different app states
    match app.active_block {
        ActiveBlock::Input => {
            handlers::input_handler(key, app);
        }
        ActiveBlock::MyPlaylists => {
            handlers::playlist_handler(key, app);
        }
        ActiveBlock::SongTable => {
            handlers::song_table_handler(key, app);
        }
        ActiveBlock::HelpMenu => {
            handlers::help_menu_handler(key, app);
        }
        ActiveBlock::Error => {
            handlers::api_error_menu_handler(key, app);
        }
        ActiveBlock::SelectDevice => {
            handlers::select_device_handler(key, app);
        }
        ActiveBlock::SearchResultBlock => {
            handlers::search_results_handler(key, app);
        }
        ActiveBlock::Home => {}
    }
}
