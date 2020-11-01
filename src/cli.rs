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

const LIKED_EMOJI: &str = " ";
const SHUFFLE_EMOJI: &str = "咽";
const REPEAT_T_EMOJI: &str = "綾";
const REPEAT_C_EMOJI: &str = "凌";

//
// Possible types to list or search
//

#[derive(Debug)]
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
  fn play_from_matches(m: &ArgMatches<'_>) -> Self {
    if m.is_present("playlist") {
      Self::Playlist
    } else if m.is_present("track") {
      Self::Track
    } else if m.is_present("artist") {
      Self::Artist
    } else if m.is_present("album") {
      Self::Album
    } else if m.is_present("show") {
      Self::Show
    }
    // Default: track
    else {
      Self::Track
    }
  }
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

//
// Possible flags to set
//

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

//
// Possible directions to jump to
//

enum JumpDirection {
  Next,
  Previous,
}

impl JumpDirection {
  fn from_matches(m: &ArgMatches<'_>) -> Self {
    if m.is_present("next") {
      Self::Next
    } else if m.is_present("previous") {
      Self::Previous
    // Again: there is no default value
    // If this function was called, one of these above
    // has to be specified
    } else {
      Self::Next
    }
  }
}

//
// For fomatting (-f / --format flag)
//

// Types to creat a Format enum from
// Boxing was proposed by cargo clippy
// to reduce the size of this enum
enum FormatType {
  Album(Box<SimplifiedAlbum>),
  Artist(Box<FullArtist>),
  Playlist(Box<SimplifiedPlaylist>),
  Track(Box<FullTrack>),
  Episode(Box<FullEpisode>),
  Show(Box<SimplifiedShow>),
}

// Types that get formatted
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
      .copied()
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

  // spt playback --next
  async fn jump(&mut self, d: JumpDirection) {
    match d {
      JumpDirection::Next => self.0.handle_network_event(IoEvent::NextTrack).await,
      JumpDirection::Previous => self.0.handle_network_event(IoEvent::PreviousTrack).await,
    }
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
                Format::from_type(FormatType::Playlist(Box::new(p.clone()))),
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
              Format::from_type(FormatType::Track(Box::new(t.clone()))),
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
        let id = track.id.clone().unwrap_or_default();
        let mut hs = Format::from_type(FormatType::Track(Box::new(track)));
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
        let mut hs = Format::from_type(FormatType::Episode(Box::new(episode)));
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

  // spt play -u URI
  async fn play_uri(&mut self, uri: String) {
    if uri.contains("spotify:track:") {
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

  // spt play -n NAME ...
  async fn play(&mut self, name: String, item: Type) -> Result<(), String> {
    self
      .0
      .handle_network_event(IoEvent::GetSearchResults(name.clone(), None))
      .await;
    // Get the uri of the first found
    // item or return an error message
    let uri = {
      let results = &self.0.app.lock().await.search_results;
      match item {
        Type::Track => {
          if let Some(r) = &results.tracks {
            r.items[0].uri.clone()
          } else {
            return Err(format!("No tracks with name {} found", name));
          }
        }
        Type::Album => {
          if let Some(r) = &results.albums {
            let album = &r.items[0];
            if let Some(uri) = &album.uri {
              uri.clone()
            } else {
              return Err(format!("Album {} has no uri", album.name));
            }
          } else {
            return Err(format!("No albums with name {} found", name));
          }
        }
        Type::Artist => {
          if let Some(r) = &results.artists {
            r.items[0].uri.clone()
          } else {
            return Err(format!("No artists with name {} found", name));
          }
        }
        Type::Show => {
          if let Some(r) = &results.shows {
            r.items[0].uri.clone()
          } else {
            return Err(format!("No shows with name {} found", name));
          }
        }
        Type::Playlist => {
          if let Some(r) = &results.playlists {
            r.items[0].uri.clone()
          } else {
            return Err(format!("No playlists with name {} found", name));
          }
        }
        _ => String::new(),
      }
    };

    // Play the uri
    self.play_uri(uri).await;
    Ok(())
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
                Format::from_type(FormatType::Playlist(Box::new(r.clone()))),
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
                Format::from_type(FormatType::Track(Box::new(r.clone()))),
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
                Format::from_type(FormatType::Artist(Box::new(r.clone()))),
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
                Format::from_type(FormatType::Show(Box::new(r.clone()))),
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
                Format::from_type(FormatType::Album(Box::new(r.clone()))),
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
      } else if matches.is_present("flags") {
        let flag = Flag::from_matches(matches);
        if let Err(e) = cli.mark(flag).await {
          return e;
        }
      } else if matches.is_present("jumps") {
        let direction = JumpDirection::from_matches(matches);
        cli.jump(direction).await;
      }

      cli.get_status(format.to_string()).await
    }
    "play" => {
      if let Some(uri) = matches.value_of("uri") {
        cli.play_uri(uri.to_string()).await;
      } else if let Some(name) = matches.value_of("name") {
        let category = Type::play_from_matches(matches);
        if let Err(e) = cli.play(name.to_string(), category).await {
          return e;
        }
      }
      cli.get_status("%s %t - %a".to_string()).await
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
