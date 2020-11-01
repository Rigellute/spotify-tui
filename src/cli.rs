use crate::network::IoEvent;
use crate::network::Network;

use clap::ArgMatches;
use rspotify::{
  model::{
    album::SimplifiedAlbum, artist::FullArtist, artist::SimplifiedArtist,
    playlist::SimplifiedPlaylist, show::FullEpisode, show::SimplifiedShow, track::FullTrack,
    PlayingItem,
  },
  senum::RepeatState,
};

const LIKED_EMOJI: &'static str = " ";
const SHUFFLE_EMOJI: &'static str = "咽";
const REPEAT_T_EMOJI: &'static str = "綾";
const REPEAT_C_EMOJI: &'static str = "凌";

enum Type {
  Playlist,
  Track,
  Artist,
  Album,
  Show,
  Device,
  Liked,
}

impl Type {
  fn search_from_matches(m: &ArgMatches<'_>) -> Self {
    if m.is_present("playlists") {
      Self::Playlist
    } else if m.is_present("tracks") {
      Self::Track
    } else if m.is_present("artists") {
      Self::Artist
    } else if m.is_present("albums") {
      Self::Album
    } else if m.is_present("shows") {
      Self::Show
    }
    // Default: track
    else {
      Self::Track
    }
  }
  fn list_from_matches(m: &ArgMatches<'_>) -> Self {
    if m.is_present("playlists") {
      Self::Playlist
    } else if m.is_present("devices") {
      Self::Device
    } else if m.is_present("liked") {
      Self::Liked
    }
    // Default: device
    else {
      Self::Device
    }
  }
}

enum Flag {
  Like,
  Shuffle,
  Repeat,
}

impl Flag {
  fn from_matches(m: &ArgMatches<'_>) -> Self {
    if m.is_present("like") {
      Self::Like
    } else if m.is_present("shuffle") {
      Self::Shuffle
    } else if m.is_present("repeat") {
      Self::Repeat
    }
    // No default, just placeholder (clap requires user to specify something)
    else {
      Self::Like
    }
  }
}

enum FormatType {
  Album(SimplifiedAlbum),
  Artist(FullArtist),
  Playlist(SimplifiedPlaylist),
  Track(FullTrack),
  Episode(FullEpisode),
  Show(SimplifiedShow),
}

#[derive(Clone)]
enum Format {
  Album(String),
  Artist(String),
  Playlist(String),
  Track(String),
  Show(String),
  Uri(String),
  Device(String),
}

fn join_artists(a: Vec<SimplifiedArtist>) -> String {
  a.iter()
    .map(|l| l.name.clone())
    .collect::<Vec<String>>()
    .join(", ")
}

impl Format {
  // Extract important information from types
  fn from_type(t: FormatType) -> Vec<Self> {
    match t {
      FormatType::Album(a) => {
        let joined_artists = join_artists(a.artists.clone());

        let mut vec = vec![Self::Album(a.name), Self::Artist(joined_artists)];
        if let Some(uri) = a.uri {
          vec.push(Self::Uri(uri));
        }

        vec
      }
      FormatType::Artist(a) => vec![Self::Artist(a.name), Self::Uri(a.uri)],
      FormatType::Playlist(p) => vec![Self::Playlist(p.name), Self::Uri(p.uri)],
      FormatType::Track(t) => {
        let joined_artists = join_artists(t.artists.clone());
        vec![
          Self::Album(t.album.name),
          Self::Artist(joined_artists),
          Self::Track(t.name),
          Self::Uri(t.uri),
        ]
      }
      FormatType::Show(r) => vec![
        Self::Artist(r.publisher),
        Self::Show(r.name),
        Self::Uri(r.uri),
      ],
      FormatType::Episode(e) => vec![
        Self::Show(e.show.name),
        Self::Artist(e.show.publisher),
        Self::Track(e.name),
        Self::Uri(e.uri),
      ],
    }
  }
  // Is there a better way?
  fn inner(&self) -> String {
    match self {
      Self::Album(s) => s,
      Self::Artist(s) => s,
      Self::Playlist(s) => s,
      Self::Track(s) => s,
      Self::Show(s) => s,
      Self::Uri(s) => s,
      Self::Device(s) => s,
    }
    .to_string()
  }
  fn get_placeholder(&self) -> &str {
    match self {
      Self::Album(_) => "%b",
      Self::Artist(_) => "%a",
      Self::Playlist(_) => "%p",
      Self::Track(_) => "%t",
      Self::Show(_) => "%h",
      Self::Uri(_) => "%u",
      Self::Device(_) => "%d",
    }
  }
}

fn format_output(
  format: String,
  values: Vec<Format>,
  // (repeat, shuffle, like)
  flags: Option<(RepeatState, bool, bool)>,
  playing: bool,
) -> String {
  // Create a mutable 'clone'
  let mut f = format;

  let flags_string = if let Some((r, s, l)) = flags {
    let shuffle = if s { SHUFFLE_EMOJI } else { "" };
    let repeat = match r {
      RepeatState::Off => "",
      RepeatState::Track => REPEAT_T_EMOJI,
      RepeatState::Context => REPEAT_C_EMOJI,
    };
    let like = if l { LIKED_EMOJI } else { "" };
    [shuffle, repeat, like]
      .iter()
      .filter(|a| !a.is_empty())
      // Reduce the &&str to &str
      .map(|a| *a)
      .collect::<Vec<&str>>()
      .join(" ")
  } else {
    "".to_string()
  };
  let playing_string = if playing { "契" } else { "" };

  // Replace set values
  for val in values {
    f = f.replace(val.get_placeholder(), &val.inner());
  }

  // Replace the rest with 'None'
  for p in &["%a", "%b", "%t", "%p", "%h", "%u", "%d"] {
    f = f.replace(p, "None");
  }

  // Add the last two
  f.replace("%f", &flags_string)
    .replace("%s", &playing_string)
}

//
// Commands
//

//
struct CliApp<'a>(Network<'a>);

// Non-concurrent functions
// I feel that async in a cli is not working
// I just .await all processes and directly interact
// by calling network.handle_network_event
impl<'a> CliApp<'a> {
  // spt playback -t
  async fn toggle_playback(&mut self) {
    let context = self.0.app.lock().await.current_playback_context.clone();
    if let Some(c) = context {
      if c.is_playing {
        self.0.handle_network_event(IoEvent::PausePlayback).await;
        return;
      }
    }
    self
      .0
      .handle_network_event(IoEvent::StartPlayback(None, None, None))
      .await;
  }

  // spt query -l ...
  async fn list(&mut self, item: Type, format: &str) -> String {
    match item {
      Type::Device => {
        if let Some(devices) = &self.0.app.lock().await.devices {
          devices
            .devices
            .iter()
            .map(|d| {
              format_output(
                format.to_string(),
                vec![Format::Device(d.name.clone())],
                None,
                false,
              )
            })
            .collect::<Vec<String>>()
            .join("\n")
        } else {
          "No devices avaible".to_string()
        }
      }
      Type::Playlist => {
        self.0.handle_network_event(IoEvent::GetPlaylists).await;
        if let Some(playlists) = &self.0.app.lock().await.playlists {
          playlists
            .items
            .iter()
            .map(|p| {
              format_output(
                format.to_string(),
                Format::from_type(FormatType::Playlist(p.clone())),
                None,
                false,
              )
            })
            .collect::<Vec<String>>()
            .join("\n")
        } else {
          "No playlists".to_string()
        }
      }
      Type::Liked => {
        self
          .0
          .handle_network_event(IoEvent::GetCurrentSavedTracks(None))
          .await;
        self
          .0
          .app
          .lock()
          .await
          .track_table
          .tracks
          .iter()
          .map(|t| {
            format_output(
              format.to_string(),
              Format::from_type(FormatType::Track(t.clone())),
              None,
              false,
            )
          })
          .collect::<Vec<String>>()
          .join("\n")
      }
      _ => String::new(),
    }
  }

  // spt playback -t DEVICE
  async fn transfer_playback(&mut self, device: &str) -> String {
    // Get the device id by name
    let mut id = String::new();
    if let Some(devices) = &self.0.app.lock().await.devices {
      for d in &devices.devices {
        if d.name == device {
          id.push_str(d.id.as_str());
          break;
        }
      }
    };

    if id.is_empty() {
      format!("Err: no device with name {}", device)
    } else {
      self
        .0
        .handle_network_event(IoEvent::TransferPlaybackToDevice(id.to_string()))
        .await;
      String::new()
    }
  }

  // spt playback --like / --shuffle / --repeat
  async fn mark(&mut self, flag: Flag) -> Result<(), String> {
    let c = {
      let app = self.0.app.lock().await;
      match app.current_playback_context.clone() {
        Some(c) => c,
        None => return Err("Err: no context avaible".to_string()),
      }
    };

    match flag {
      Flag::Like => {
        // Get the id of the current song
        let id = match c.item {
          Some(i) => match i {
            PlayingItem::Track(t) => match t.id {
              Some(id) => id,
              None => return Err("Err: item has no id".to_string()),
            },
            PlayingItem::Episode(_) => {
              return Err("Err: saving episodes not yet implemented".to_string())
            }
          },
          None => return Err("Err: no item playing".to_string()),
        };
        self
          .0
          .handle_network_event(IoEvent::ToggleSaveTrack(id))
          .await;
      }
      Flag::Shuffle => {
        self
          .0
          .handle_network_event(IoEvent::Shuffle(c.shuffle_state))
          .await
      }
      // Very weird behavior
      // For some reason you can't set RepeatState::Track
      // This just toggles between RepeatState::Off and RepeatState::Context
      Flag::Repeat => {
        let r = match c.repeat_state {
          RepeatState::Off => RepeatState::Off,
          RepeatState::Track => RepeatState::Track,
          RepeatState::Context => RepeatState::Track,
        };
        self.0.handle_network_event(IoEvent::Repeat(r)).await;
      }
    }

    Ok(())
  }

  // spt ... -d ... (specify device to control)
  async fn set_device(&mut self, name: String) -> Result<(), String> {
    // Change the device if specified by user
    let mut app = self.0.app.lock().await;
    let mut selected_device_index = Some(0);
    if let Some(dp) = &app.devices {
      for (i, d) in dp.devices.iter().enumerate() {
        if d.name == name {
          selected_device_index = Some(i);
        }
      }
    } else {
      // Error out if no device is avaible
      return Err("Err: no device avaible".to_string());
    }
    app.selected_device_index = selected_device_index;
    Ok(())
  }

  // spt playback -s
  async fn get_status(&mut self, format: String) -> String {
    // Update info on current playback
    self
      .0
      .handle_network_event(IoEvent::GetCurrentPlayback)
      .await;
    self
      .0
      .handle_network_event(IoEvent::GetCurrentSavedTracks(None))
      .await;

    let context = match self.0.app.lock().await.current_playback_context.clone() {
      Some(c) => c,
      None => return "Err: no context avaible".to_string(),
    };

    let playing_item = match context.item {
      Some(item) => item,
      None => return "No track playing".to_string(),
    };

    match playing_item {
      PlayingItem::Track(track) => {
        let id = track.id.clone().unwrap_or(String::new());
        let mut hs = Format::from_type(FormatType::Track(track));
        hs.push(Format::Device(context.device.name));
        format_output(
          format,
          hs,
          Some((
            context.repeat_state,
            context.shuffle_state,
            self.0.app.lock().await.liked_song_ids_set.contains(&id),
          )),
          context.is_playing,
        )
      }
      PlayingItem::Episode(episode) => {
        let mut hs = Format::from_type(FormatType::Episode(episode));
        hs.push(Format::Device(context.device.name));
        format_output(
          format,
          hs,
          Some((context.repeat_state, context.shuffle_state, false)),
          context.is_playing,
        )
      }
    }
  }

  // spt play -t / -p URI
  async fn play_uri(&mut self, uri: String, track: bool) {
    // Track was requested
    if track {
      self
        .0
        .handle_network_event(IoEvent::StartPlayback(
          None,
          Some(vec![uri.clone()]),
          Some(0),
        ))
        .await;
    } else {
      self
        .0
        .handle_network_event(IoEvent::StartPlayback(Some(uri.clone()), None, None))
        .await;
    }
  }

  // Query for a playlist, track, artist, shows and albums
  // Returns result and their respective uris (to play them)
  //
  // spt query -s SEARCH ...
  async fn query(&mut self, search: String, format: String, item: Type) -> String {
    self
      .0
      .handle_network_event(IoEvent::GetSearchResults(search, None))
      .await;

    let app = self.0.app.lock().await;
    match item {
      Type::Playlist => {
        if let Some(results) = &app.search_results.playlists {
          results
            .items
            .iter()
            .map(|r| {
              format_output(
                format.clone(),
                Format::from_type(FormatType::Playlist(r.clone())),
                None,
                false,
              )
            })
            .collect::<Vec<String>>()
            .join("\n")
        } else {
          "No playlists found".to_string()
        }
      }
      Type::Track => {
        if let Some(results) = &app.search_results.tracks {
          results
            .items
            .iter()
            .map(|r| {
              format_output(
                format.clone(),
                Format::from_type(FormatType::Track(r.clone())),
                None,
                false,
              )
            })
            .collect::<Vec<String>>()
            .join("\n")
        } else {
          "No tracks found".to_string()
        }
      }
      Type::Artist => {
        if let Some(results) = &app.search_results.artists {
          results
            .items
            .iter()
            .map(|r| {
              format_output(
                format.clone(),
                Format::from_type(FormatType::Artist(r.clone())),
                None,
                false,
              )
            })
            .collect::<Vec<String>>()
            .join("\n")
        } else {
          "No artists found".to_string()
        }
      }
      Type::Show => {
        if let Some(results) = &app.search_results.shows {
          results
            .items
            .iter()
            .map(|r| {
              format_output(
                format.clone(),
                Format::from_type(FormatType::Show(r.clone())),
                None,
                false,
              )
            })
            .collect::<Vec<String>>()
            .join("\n")
        } else {
          "No shows found".to_string()
        }
      }
      Type::Album => {
        if let Some(results) = &app.search_results.albums {
          results
            .items
            .iter()
            .map(|r| {
              format_output(
                format.clone(),
                Format::from_type(FormatType::Album(r.clone())),
                None,
                false,
              )
            })
            .collect::<Vec<String>>()
            .join("\n")
        } else {
          "No albums found".to_string()
        }
      }
      // Never called, just here for compiler
      _ => String::new(),
    }
  }
}

pub async fn handle_matches(matches: &ArgMatches<'_>, cmd: String, net: Network<'_>) -> String {
  // Tuple struct
  let mut cli = CliApp(net);

  cli.0.handle_network_event(IoEvent::GetDevices).await;
  cli
    .0
    .handle_network_event(IoEvent::GetCurrentPlayback)
    .await;

  if let Some(d) = matches.value_of("device") {
    if cli.set_device(d.to_string()).await.is_err() {
      return "Err: failed to set device".to_string();
    }
  }

  // Evalute the subcommand
  let output = match cmd.as_str() {
    "playback" => {
      let format = matches.value_of("format").unwrap();
      if matches.is_present("toggle") {
        cli.toggle_playback().await;
      } else if let Some(d) = matches.value_of("transfer") {
        let output = cli.transfer_playback(d).await;
        if !output.is_empty() {
          return output;
        }
      }

      if matches.is_present("flags") {
        let flag = Flag::from_matches(matches);
        if let Err(e) = cli.mark(flag).await {
          return e;
        }
      }

      cli.get_status(format.to_string()).await
    }
    "play" => {
      if let Some(uri) = matches.value_of("URI") {
        if matches.is_present("context") {
          cli.play_uri(uri.to_string(), false).await;
        } else {
          // Play track by default
          cli.play_uri(uri.to_string(), true).await;
        }
        cli.get_status("%s %t - %a".to_string()).await
      // Never called, just here for the compiler
      } else {
        String::new()
      }
    }
    "query" => {
      if matches.is_present("list") {
        let format = matches.value_of("format").unwrap();
        let category = Type::list_from_matches(matches);
        cli.list(category, format).await
      } else if let Some(search) = matches.value_of("search") {
        let format = matches.value_of("format").unwrap().to_string();
        let query_type = Type::search_from_matches(matches);
        cli.query(search.to_string(), format, query_type).await
      // Never called, just here for the compiler
      } else {
        String::new()
      }
    }
    // Never called, just here for the compiler
    _ => String::new(),
  };

  // Check if there was an error
  let api_error = cli.0.app.lock().await.api_error.clone();
  if api_error.is_empty() {
    output
  } else {
    format!("Err: {}", api_error)
  }
}
