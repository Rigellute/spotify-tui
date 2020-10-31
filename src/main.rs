mod app;
mod banner;
mod cli;
mod config;
mod event;
mod handlers;
mod network;
mod redirect_uri;
mod ui;
mod user_config;

use crate::app::RouteId;
use crate::event::Key;
use anyhow::Result;
use app::{ActiveBlock, App};
use backtrace::Backtrace;
use banner::BANNER;
use clap::{App as ClapApp, Arg, ArgGroup, SubCommand};
use cli::handle_matches;
use config::ClientConfig;
use crossterm::{
  cursor::MoveTo,
  event::{DisableMouseCapture, EnableMouseCapture},
  execute,
  style::Print,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};
use network::{get_spotify, IoEvent, Network};
use redirect_uri::redirect_uri_web_server;
use rspotify::{
  oauth2::{SpotifyOAuth, TokenInfo},
  util::{process_token, request_token},
};
use std::{
  cmp::{max, min},
  io::{self, stdout, Write},
  panic::{self, PanicInfo},
  path::PathBuf,
  sync::Arc,
  time::SystemTime,
};
use tokio::sync::Mutex;
use tui::{
  backend::{Backend, CrosstermBackend},
  Terminal,
};
use user_config::{UserConfig, UserConfigPaths};

const SCOPES: [&str; 14] = [
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
  "user-read-playback-position",
  "user-read-private",
  "user-read-recently-played",
];

/// get token automatically with local webserver
pub async fn get_token_auto(spotify_oauth: &mut SpotifyOAuth, port: u16) -> Option<TokenInfo> {
  match spotify_oauth.get_cached_token().await {
    Some(token_info) => Some(token_info),
    None => match redirect_uri_web_server(spotify_oauth, port) {
      Ok(mut url) => process_token(spotify_oauth, &mut url).await,
      Err(()) => {
        println!("Starting webserver failed. Continuing with manual authentication");
        request_token(spotify_oauth);
        println!("Enter the URL you were redirected to: ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
          Ok(_) => process_token(spotify_oauth, &mut input).await,
          Err(_) => None,
        }
      }
    },
  }
}

fn close_application() -> Result<()> {
  disable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
  Ok(())
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

    disable_raw_mode().unwrap();
    execute!(
      io::stdout(),
      LeaveAlternateScreen,
      Print(format!(
        "thread '<unnamed>' panicked at '{}', {}\n\r{}",
        msg, location, stacktrace
      )),
      DisableMouseCapture
    )
    .unwrap();
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  panic::set_hook(Box::new(|info| {
    panic_hook(info);
  }));

  let device_arg = Arg::with_name("device")
    .short("d")
    .long("device")
    .takes_value(true)
    .value_name("DEVICE")
    .help("Specify device to use");
  let format_arg = Arg::with_name("format")
    .short("f")
    .long("format")
    .takes_value(true)
    .value_name("FORMAT")
    .help("Specify output format");
  let matches = ClapApp::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .usage("Press `?` while running the app to see keybindings")
        .before_help(BANNER)
        .after_help("Your spotify Client ID and Client Secret are stored in $HOME/.config/spotify-tui/client.yml")
         .arg(Arg::with_name("tick-rate")
                               .short("t")
                               .long("tick-rate")
                               .help("Set the tick rate (milliseconds): the lower the number the higher the FPS. It can be nicer to have a lower value when you want to use the audio analysis view of the app. Beware that this comes at a CPU cost!")
                               .takes_value(true))
         .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .help("Specify configuration file path.")
                               .takes_value(true))
         // Control spotify from the command line
         .subcommand(SubCommand::with_name("playback")
                               .version(env!("CARGO_PKG_VERSION"))
                               .author(env!("CARGO_PKG_AUTHORS"))
                               .about("Interact with playback of a device")
                               .visible_alias("pb")
                               .arg(&device_arg)
                               .arg(format_arg.clone()
                                    .default_value("%s %t - %a")
                                    .default_value_ifs(&[
                                      ("like",      None, "%f %s %t - %a"   ),
                                      ("shuffle",   None, "%f %s %t - %a"   ),
                                      ("repeat",    None, "%f %s %t - %a"   ),
                                      ("transfer",  None, "%s %t - %a on %d" )
                                    ]))
                               .arg(Arg::with_name("toggle")
                                    .short("t")
                                    .long("toggle")
                                    .help("Pause/resume playback of a device"))
                               .arg(Arg::with_name("status")
                                    .short("s")
                                    .long("status")
                                    .help("Print out status of a device"))
                               .arg(Arg::with_name("transfer")
                                    .short("r")
                                    .long("transfer")
                                    .conflicts_with("device")
                                    .takes_value(true)
                                    .value_name("DEVICE")
                                    .help("Transfer playback to device"))
                               .arg(Arg::with_name("like")
                                    .long("like")
                                    .help("Like the current song"))
                               .arg(Arg::with_name("shuffle")
                                    .long("shuffle")
                                    .help("Toggle shuffle mode"))
                               .arg(Arg::with_name("repeat")
                                    .long("repeat")
                                    .help("Toggle repeat mode"))
                               .group(ArgGroup::with_name("flags")
                                    .args(&["like", "shuffle", "repeat"]))
                               .group(ArgGroup::with_name("actions")
                                    .args(&["toggle", "status", "transfer", "like", "shuffle", "repeat"])
                                    .multiple(false)
                                    .required(true)))
        .subcommand(SubCommand::with_name("play")
                               .version(env!("CARGO_PKG_VERSION"))
                               .author(env!("CARGO_PKG_AUTHORS"))
                               .about("Play a uri (track, playlist, artist, album)")
                               .visible_alias("p")
                               .arg(&device_arg)
                               .arg(Arg::with_name("URI")
                                    .required(true)
                                    .help("Specify URI to play"))
                               .arg(Arg::with_name("track")
                                    .short("t")
                                    .long("track")
                                    .help("Specify track to play"))
                               .arg(Arg::with_name("context")
                                    .short("c")
                                    .long("context")
                                    .help("Specify a context (artist, playlist, album) to play"))
                               .group(ArgGroup::with_name("types")
                                    .args(&["track", "context"])
                                    .multiple(false)
                                    .required(true)))
        .subcommand(SubCommand::with_name("query")
                               .version(env!("CARGO_PKG_VERSION"))
                               .author(env!("CARGO_PKG_AUTHORS"))
                               .about("Search for tracks, playlists and more")
                               .visible_alias("q")
                               .arg(format_arg
                                    .default_value_ifs(&[
                                        ("devices",     None, "%d"          ),
                                        ("liked",       None, "%t - %a (%u)"),
                                        ("tracks",      None, "%t - %a (%u)"),
                                        ("playlists",   None, "%p (%u)"     ),
                                        ("artists",     None, "%a (%u)"     ),
                                        ("albums",      None, "%l - %(%u)"  ),
                                        ("shows",       None, "%h - %(%u)"  )
                                    ]))
                               // Listing
                               .arg(Arg::with_name("list")
                                    .short("l")
                                    .long("list")
                                    .help("List devices and playlists"))
                               .arg(Arg::with_name("devices")
                                    .short("d")
                                    .long("devices")
                                    .help("List the category 'devices'"))
                               .arg(Arg::with_name("playlists")
                                    .short("p")
                                    .long("playlists")
                                    .help("List / search the category 'playlists'"))
                               .arg(Arg::with_name("liked")
                                    .long("liked")
                                    .help("List the category 'liked songs'"))
                               .group(ArgGroup::with_name("listable")
                                    .args(&["devices", "playlists", "liked"])
                                    .multiple(false))
                               // Searching
                               .arg(Arg::with_name("search")
                                    .short("s")
                                    .long("search")
                                    .takes_value(true)
                                    .value_name("SEARCH")
                                    .requires("searchable")
                                    .help("Search for tracks etc."))
                               .arg(Arg::with_name("albums")
                                    .short("b")
                                    .long("albums")
                                    .help("Search the category 'albums'"))
                               .arg(Arg::with_name("artists")
                                    .short("a")
                                    .long("artists")
                                    .help("Search the category 'artists'"))
                               .arg(Arg::with_name("tracks")
                                    .short("t")
                                    .long("tracks")
                                    .help("Search the category 'tracks'"))
                               .arg(Arg::with_name("shows")
                                    .short("h")
                                    .long("shows")
                                    .help("Search the category 'shows'"))
                               .group(ArgGroup::with_name("searchable")
                                    .args(&["playlists", "tracks", "albums", "artists", "shows"])
                                    .multiple(false))
                               // Actions
                               .group(ArgGroup::with_name("actions")
                                    .args(&["list", "search"])
                                    .multiple(false)
                                    .required(true)))
        .get_matches();

  let mut user_config = UserConfig::new();
  if let Some(config_file_path) = matches.value_of("config") {
    let config_file_path = PathBuf::from(config_file_path);
    let path = UserConfigPaths { config_file_path };
    user_config.path_to_config.replace(path);
  }
  user_config.load_config()?;

  if let Some(tick_rate) = matches
    .value_of("tick-rate")
    .and_then(|tick_rate| tick_rate.parse().ok())
  {
    if tick_rate >= 1000 {
      panic!("Tick rate must be below 1000");
    } else {
      user_config.behavior.tick_rate_milliseconds = tick_rate;
    }
  }

  let mut client_config = ClientConfig::new();
  client_config.load_config()?;

  let config_paths = client_config.get_or_build_paths()?;

  // Start authorization with spotify
  let mut oauth = SpotifyOAuth::default()
    .client_id(&client_config.client_id)
    .client_secret(&client_config.client_secret)
    .redirect_uri(&client_config.get_redirect_uri())
    .cache_path(config_paths.token_cache_path)
    .scope(&SCOPES.join(" "))
    .build();

  let config_port = client_config.get_port();
  match get_token_auto(&mut oauth, config_port).await {
    Some(token_info) => {
      let (sync_io_tx, sync_io_rx) = std::sync::mpsc::channel::<IoEvent>();

      let (spotify, token_expiry) = get_spotify(token_info);

      // Initialise app state
      let app = Arc::new(Mutex::new(App::new(
        sync_io_tx,
        user_config.clone(),
        token_expiry,
      )));

      // Check if user asked to execute command
      let mut sub_matches = None;
      let possible_cmds = ["play", "query", "playback"];
      for cmd in &possible_cmds {
        if let Some(m) = matches.subcommand_matches(cmd) {
          sub_matches = Some((m, cmd));
        }
      }

      // Work with the cli (not really async)
      if let Some((m, cmd)) = sub_matches {
        let mut network = Network::new(oauth, spotify, client_config, &app);
        println!("{}", handle_matches(m, cmd.to_string(), &mut network).await);
      // Launch the tui (async)
      } else {
        let cloned_app = Arc::clone(&app);
        std::thread::spawn(move || {
          let mut network = Network::new(oauth, spotify, client_config, &app);
          start_tokio(sync_io_rx, &mut network);
        });
        // The UI must run in the "main" thread
        start_ui(user_config, &cloned_app).await?;
      }
    }
    None => println!("\nSpotify auth failed"),
  }

  Ok(())
}

#[tokio::main]
async fn start_tokio<'a>(io_rx: std::sync::mpsc::Receiver<IoEvent>, network: &mut Network) {
  while let Ok(io_event) = io_rx.recv() {
    network.handle_network_event(io_event).await;
  }
}

async fn start_ui(user_config: UserConfig, app: &Arc<Mutex<App>>) -> Result<()> {
  // Terminal initialization
  let mut stdout = stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  enable_raw_mode()?;

  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  terminal.hide_cursor()?;

  let events = event::Events::new(user_config.behavior.tick_rate_milliseconds);

  // play music on, if not send them to the device selection view

  let mut is_first_render = true;

  loop {
    let mut app = app.lock().await;
    // Get the size of the screen on each loop to account for resize event
    if let Ok(size) = terminal.backend().size() {
      // Reset the help menu is the terminal was resized
      if is_first_render || app.size != size {
        app.help_menu_max_lines = 0;
        app.help_menu_offset = 0;
        app.help_menu_page = 0;

        app.size = size;

        // Based on the size of the terminal, adjust the search limit.
        let potential_limit = max((app.size.height as i32) - 13, 0) as u32;
        let max_limit = min(potential_limit, 50);
        let large_search_limit = min((f32::from(size.height) / 1.4) as u32, max_limit);
        let small_search_limit = min((f32::from(size.height) / 2.85) as u32, max_limit / 2);

        app.dispatch(IoEvent::UpdateSearchLimits(
          large_search_limit,
          small_search_limit,
        ));

        // Based on the size of the terminal, adjust how many lines are
        // dislayed in the help menu
        if app.size.height > 8 {
          app.help_menu_max_lines = (app.size.height as u32) - 8;
        } else {
          app.help_menu_max_lines = 0;
        }
      }
    };

    let current_route = app.get_current_route();
    terminal.draw(|mut f| match current_route.active_block {
      ActiveBlock::HelpMenu => {
        ui::draw_help_menu(&mut f, &app);
      }
      ActiveBlock::Error => {
        ui::draw_error_screen(&mut f, &app);
      }
      ActiveBlock::SelectDevice => {
        ui::draw_device_list(&mut f, &app);
      }
      ActiveBlock::Analysis => {
        ui::audio_analysis::draw(&mut f, &app);
      }
      ActiveBlock::BasicView => {
        ui::draw_basic_view(&mut f, &app);
      }
      _ => {
        ui::draw_main_layout(&mut f, &app);
      }
    })?;

    if current_route.active_block == ActiveBlock::Input {
      terminal.show_cursor()?;
    } else {
      terminal.hide_cursor()?;
    }

    let cursor_offset = if app.size.height > ui::util::SMALL_TERMINAL_HEIGHT {
      2
    } else {
      1
    };

    // Put the cursor back inside the input box
    terminal.backend_mut().execute(MoveTo(
      cursor_offset + app.input_cursor_position,
      cursor_offset,
    ))?;

    // Handle authentication refresh
    if SystemTime::now() > app.spotify_token_expiry {
      app.dispatch(IoEvent::RefreshAuthentication);
    }

    match events.next()? {
      event::Event::Input(key) => {
        if key == Key::Ctrl('c') {
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
              Some(ref x) if x.id == RouteId::Search => app.pop_navigation_stack(),
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
      app.dispatch(IoEvent::GetPlaylists);
      app.dispatch(IoEvent::GetUser);
      app.dispatch(IoEvent::GetCurrentPlayback);
      app.help_docs_size = ui::help::get_help_docs(&app.user_config.keys).len() as u32;

      is_first_render = false;
    }
  }

  terminal.show_cursor()?;
  close_application()?;

  Ok(())
}
