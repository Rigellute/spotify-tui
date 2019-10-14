mod app;
mod banner;
mod config;
mod handlers;
mod redirect_uri;
mod ui;
mod util;

use clap::App as ClapApp;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth, TokenInfo};
use rspotify::spotify::util::get_token;
use std::cmp::min;
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::{Backend, TermionBackend};
use tui::Terminal;

use app::{ActiveBlock, App};
use banner::BANNER;
use config::{ClientConfig, LOCALHOST};
use redirect_uri::redirect_uri_web_server;
use util::{Event, Events};

const SCOPES: [&str; 8] = [
    "playlist-read-private",
    "user-library-modify",
    "user-library-read",
    "user-modify-playback-state",
    "user-read-currently-playing",
    "user-read-playback-state",
    "user-read-private",
    "user-read-recently-played",
];

fn get_spotify(token_info: TokenInfo) -> (Spotify, Instant) {
    let token_expiry = Instant::now()
        + Duration::from_secs(token_info.expires_in.into())
        // Set 10 seconds early
        - Duration::from_secs(10);

    let client_credential = SpotifyClientCredentials::default()
        .token_info(token_info)
        .build();

    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    (spotify, token_expiry)
}

fn main() -> Result<(), failure::Error> {
    ClapApp::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .usage("Press `?` while running the app to see keybindings")
        .before_help(BANNER)
        .after_help("Your spotify Client ID and Client Secret are stored in $HOME/.config/spotify-tui/client.yml")
        .get_matches();

    let (tx, rx) = mpsc::channel();

    // Start the web server in case we need to use the redirect uri, this will get closed below
    // after auth is successful
    thread::spawn(|| {
        redirect_uri_web_server(rx);
    });

    let mut client_config = ClientConfig::new();
    client_config.load_config()?;

    let config_paths = client_config.get_or_build_paths()?;

    // Start authorization with spotify
    let mut oauth = SpotifyOAuth::default()
        .client_id(&client_config.client_id)
        .client_secret(&client_config.client_secret)
        .redirect_uri(LOCALHOST)
        .cache_path(config_paths.token_cache_path)
        .scope(&SCOPES.join(" "))
        .build();

    match get_token(&mut oauth) {
        Some(token_info) => {
            // Terminate the web server running for the Redirect URI (fire and forget)
            if tx.send(()).is_ok() {};

            // Terminal initialization
            let stdout = io::stdout().into_raw_mode()?;
            let stdout = MouseTerminal::from(stdout);
            let stdout = AlternateScreen::from(stdout);
            let backend = TermionBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;
            terminal.hide_cursor()?;

            let events = Events::new();

            // Initialise app state
            let mut app = App::new();

            let (mut spotify, mut token_expiry) = get_spotify(token_info);

            app.client_config = client_config;

            app.spotify = Some(spotify);

            // Now that spotify is ready, check if the user has already selected a device_id to
            // play music on, if not send them to the device selection view
            if app.client_config.device_id.is_none() {
                app.handle_get_devices();
            }

            let mut is_first_render = true;

            loop {
                // Get the size of the screen on each loop to account for resize events
                if let Ok(size) = terminal.backend().size() {
                    app.size = size;

                    // Based on the size of the terminal, adjust the search limit.
                    let max_limit = 50;
                    app.large_search_limit = min((f32::from(size.height) / 1.5) as u32, max_limit);
                };

                let current_route = app.get_current_route();
                terminal.draw(|mut f| match current_route.active_block {
                    ActiveBlock::HelpMenu => {
                        ui::draw_help_menu(&mut f);
                    }
                    ActiveBlock::Error => {
                        ui::draw_error_screen(&mut f, &app);
                    }
                    ActiveBlock::SelectDevice => {
                        ui::draw_device_list(&mut f, &app);
                    }
                    _ => {
                        ui::draw_main_layout(&mut f, &app);
                    }
                })?;

                if current_route.active_block == ActiveBlock::Input {
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
                    Goto(4 + app.input_cursor_position, 4)
                )?;

                // stdout is buffered, flush it to see the effect immediately when hitting backspace
                io::stdout().flush().ok();

                if Instant::now() > token_expiry {
                    // refresh token
                    if let Some(new_token_info) = get_token(&mut oauth) {
                        let (new_spotify, new_token_expiry) = get_spotify(new_token_info);
                        spotify = new_spotify;
                        token_expiry = new_token_expiry;
                        app.spotify = Some(spotify);
                    } else {
                        println!("\nFailed to refresh authentication token");
                        break;
                    }
                }

                match events.next()? {
                    Event::Input(key) => {
                        if key == Key::Ctrl('c') {
                            break;
                        }
                        let current_active_block = app.get_current_route().active_block;

                        // To avoid swallowing the global key presses `q` and `-` make a special
                        // case for the input handler
                        if current_active_block == ActiveBlock::Input {
                            handlers::input_handler(key, &mut app);
                        } else {
                            match key {
                                // Handle global navigation" back" event
                                Key::Char('q') => {
                                    if app.get_current_route().active_block != ActiveBlock::Input {
                                        // Go back through navigation stack when not in search input mode and exit the app if there are no more places to back to
                                        let pop_result = app.pop_navigation_stack();

                                        if pop_result.is_none() {
                                            break;
                                        }
                                    }
                                }
                                _ => {
                                    handlers::handle_app(key, &mut app);
                                }
                            };
                        }
                    }
                    Event::Tick => {
                        app.update_on_tick();
                    }
                }

                // Delay spotify request until first render, will have the effect of improving
                // startup speed
                if is_first_render {
                    if let Some(spotify) = &app.spotify {
                        let playlists =
                            spotify.current_user_playlists(app.large_search_limit, None);

                        match playlists {
                            Ok(p) => {
                                app.playlists = Some(p);
                                // Select the first playlist
                                app.selected_playlist_index = Some(0);
                            }
                            Err(e) => {
                                app.handle_error(e);
                            }
                        };
                    }

                    app.get_current_playback();
                    is_first_render = false;
                }
            }
        }
        None => println!("\nSpotify auth failed"),
    }

    Ok(())
}
