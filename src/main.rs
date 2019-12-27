mod app;
mod banner;
mod config;
mod event;
mod handlers;
mod redirect_uri;
mod ui;
mod user_config;

use crate::app::RouteId;
use crate::event::Key;
use app::{ActiveBlock, App};
use backtrace::Backtrace;
use banner::BANNER;
use clap::App as ClapApp;
use config::{ClientConfig, LOCALHOST};
use crossterm::{
    cursor::MoveTo,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use failure::format_err;
use redirect_uri::redirect_uri_web_server;
use rspotify::spotify::{
    client::Spotify,
    oauth2::{SpotifyClientCredentials, SpotifyOAuth, TokenInfo},
    util::{get_token, process_token, request_token},
};
use std::{
    cmp::min,
    io::{self, stdout, Write},
    panic::{self, PanicInfo},
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use user_config::UserConfig;

const SCOPES: [&str; 13] = [
    "playlist-read-collaborative",
    "playlist-read-private",
    "playlist-modify-private",
    "playlist-modify-public",
    "user-follow-read",
    "user-follow-modify",
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
/// get token automatically with local webserver
pub fn get_token_auto(spotify_oauth: &mut SpotifyOAuth) -> Option<TokenInfo> {
    match spotify_oauth.get_cached_token() {
        Some(token_info) => Some(token_info),
        None => match redirect_uri_web_server(spotify_oauth) {
            Ok(mut url) => process_token(spotify_oauth, &mut url),
            Err(()) => {
                println!("Starting webserver failed. Continuing with manual authentication");
                request_token(spotify_oauth);
                println!("Enter the URL you were redirected to: ");
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => process_token(spotify_oauth, &mut input),
                    Err(_) => None,
                }
            }
        },
    }
}

fn panic_hook(info: &PanicInfo<'_>) {
    if cfg!(debug_assertions) {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let stacktrace: String = format!("{:?}", Backtrace::new()).replace('\n', "\n\r");

        execute!(
            io::stdout(),
            LeaveAlternateScreen,
            Print(format!(
                "thread '<unnamed>' panicked at '{}', {}\n\r{}",
                msg, location, stacktrace
            ))
        )
        .unwrap();
    }
}

fn main() -> Result<(), failure::Error> {
    panic::set_hook(Box::new(|info| {
        panic_hook(info);
    }));

    ClapApp::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .usage("Press `?` while running the app to see keybindings")
        .before_help(BANNER)
        .after_help("Your spotify Client ID and Client Secret are stored in $HOME/.config/spotify-tui/client.yml")
        .get_matches();

    let mut user_config = UserConfig::new();
    user_config.load_config()?;

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

    match get_token_auto(&mut oauth) {
        Some(token_info) => {
            // Terminal initialization
            let mut stdout = stdout();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
            enable_raw_mode()?;

            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;
            terminal.hide_cursor()?;

            let events = event::Events::new();

            // Initialise app state
            let mut app = App::new();

            let (mut spotify, mut token_expiry) = get_spotify(token_info);

            app.client_config = client_config;
            app.user_config = user_config;

            app.spotify = Some(spotify);

            app.clipboard_context =
                Some(clipboard::ClipboardProvider::new().map_err(|err| {
                    format_err!("failed to intialize clipboard context: {}", err)
                })?);

            // Now that spotify is ready, check if the user has already selected a device_id to
            // play music on, if not send them to the device selection view
            if app.client_config.device_id.is_none() {
                app.handle_get_devices();
            }

            let mut is_first_render = true;

            loop {
                // Get the size of the screen on each loop to account for resize event
                if let Ok(size) = terminal.backend().size() {
                    app.size = size;

                    // Based on the size of the terminal, adjust the search limit.
                    let max_limit = min((app.size.height as u32) - 13, 50);
                    app.large_search_limit = min((f32::from(size.height) / 1.4) as u32, max_limit);
                    app.small_search_limit =
                        min((f32::from(size.height) / 2.85) as u32, max_limit / 2);
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

                let cursor_offset = if app.size.height > ui::SMALL_TERMINAL_HEIGHT {
                    3
                } else {
                    2
                };

                // Put the cursor back inside the input box
                terminal.backend_mut().execute(MoveTo(
                    cursor_offset + app.input_cursor_position,
                    cursor_offset,
                ))?;

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
                    event::Event::Input(key) => {
                        if key == Key::Ctrl('c') {
                            disable_raw_mode()?;
                            let mut stdout = io::stdout();
                            execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
                            break;
                        }

                        let current_active_block = app.get_current_route().active_block;

                        // To avoid swallowing the global key presses `q` and `-` make a special
                        // case for the input handler
                        if current_active_block == ActiveBlock::Input {
                            handlers::input_handler(key, &mut app);
                        } else if key == app.user_config.keys.back {
                            if app.get_current_route().active_block != ActiveBlock::Input {
                                // Go back through navigation stack when not in search input mode and exit the app if there are no more places to back to

                                let pop_result = match app.pop_navigation_stack() {
                                    Some(ref x) if x.id == RouteId::Search => {
                                        app.pop_navigation_stack()
                                    }
                                    Some(x) => Some(x),
                                    None => None,
                                };
                                if pop_result.is_none() {
                                    break; // Exit application
                                }
                            }
                        } else {
                            handlers::handle_app(key, &mut app);
                        }
                    }
                    event::Event::Tick => {
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

                        app.get_user();
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
