use crate::network::IoEvent;
use crate::network::Network;

use anyhow::{anyhow, Result};
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
const PLAYING_EMOJI: &str = "契";
const PAUSED_EMOJI: &str = " ";

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

// Types to create a Format enum from
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
  Volume(u32),
  // This is a bit long, should it be splitted up?
  Flags((RepeatState, bool, bool)),
  Playing(bool),
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
      Self::Album(s) => s.clone(),
      Self::Artist(s) => s.clone(),
      Self::Playlist(s) => s.clone(),
      Self::Track(s) => s.clone(),
      Self::Show(s) => s.clone(),
      Self::Uri(s) => s.clone(),
      Self::Device(s) => s.clone(),
      // Because this match statements
      // needs to return a &String I have to do it this way
      Self::Volume(s) => s.to_string(),
      Self::Flags((r, s, l)) => {
        let like = if *l { LIKED_EMOJI } else { "" };
        let shuffle = if *s { SHUFFLE_EMOJI } else { "" };
        let repeat = match r {
          RepeatState::Off => "",
          RepeatState::Track => REPEAT_T_EMOJI,
          RepeatState::Context => REPEAT_C_EMOJI,
        };

        [shuffle, repeat, like]
          .iter()
          .filter(|a| !a.is_empty())
          // Reduce the &&str to &str
          .copied()
          .collect::<Vec<&str>>()
          .join(" ")
      }
      Self::Playing(s) => if *s { PLAYING_EMOJI } else { PAUSED_EMOJI }.to_string(),
    }
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
      Self::Volume(_) => "%v",
      Self::Flags(_) => "%f",
      Self::Playing(_) => "%s",
    }
  }
}

fn format_output(mut format: String, values: Vec<Format>) -> String {
  for val in values {
    format = format.replace(val.get_placeholder(), &val.inner());
  }
  // Replace unsupported flags with 'None'
  for p in &["%a", "%b", "%t", "%p", "%h", "%u", "%d", "%v", "%f", "%s"] {
    format = format.replace(p, "None");
  }
  format.trim().to_string()
}

//
// Commands
//

struct CliApp<'a>(Network<'a>);

// Non-concurrent functions
// I feel that async in a cli is not working
// I just .await all processes and directly interact
// by calling network.handle_network_event (network = self.0)
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

  // spt ... -d ... (specify device to control)
  async fn set_device(&mut self, name: String) -> Result<()> {
    // Change the device if specified by user
    let mut app = self.0.app.lock().await;
    let mut device_index = 0;
    if let Some(dp) = &app.devices {
      for (i, d) in dp.devices.iter().enumerate() {
        if d.name == name {
          device_index = i;
          // Save the id of the device
          self
            .0
            .client_config
            .set_device_id(d.id.clone())
            .map_err(|_e| anyhow!("failed to set device with name '{}'", d.name))?;
        }
      }
    } else {
      // Error out if no device is avaible
      return Err(anyhow!("no device avaible"));
    }
    app.selected_device_index = Some(device_index);
    Ok(())
  }

  async fn volume(&mut self, vol: String) -> Result<()> {
    let num = vol
      .parse::<u32>()
      .map_err(|_e| anyhow!("failed to convert {} to u32", vol))?;

    // Check if it's in range
    if num > 100 {
      return Err(anyhow!("{} is too big, max volume is 100", num));
    };

    self
      .0
      .handle_network_event(IoEvent::ChangeVolume(num as u8))
      .await;
    Ok(())
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
                vec![
                  Format::Device(d.name.clone()),
                  Format::Volume(d.volume_percent),
                ],
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
      format!("no device with name {}", device)
    } else {
      self
        .0
        .handle_network_event(IoEvent::TransferPlaybackToDevice(id.to_string()))
        .await;
      String::new()
    }
  }

  // spt playback --like / --shuffle / --repeat
  async fn mark(&mut self, flag: Flag) -> Result<()> {
    let c = {
      let app = self.0.app.lock().await;
      app
        .current_playback_context
        .clone()
        .ok_or_else(|| anyhow!("no context avaible"))?
    };

    match flag {
      Flag::Like => {
        // Get the id of the current song
        let id = match c.item {
          Some(i) => match i {
            PlayingItem::Track(t) => t.id.ok_or_else(|| anyhow!("item has no id")),
            PlayingItem::Episode(_) => Err(anyhow!("saving episodes not yet implemented")),
          },
          None => Err(anyhow!("no item playing")),
        };
        self
          .0
          .handle_network_event(IoEvent::ToggleSaveTrack(id?))
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
        self
          .0
          .handle_network_event(IoEvent::Repeat(c.repeat_state))
          .await;
      }
    }

    Ok(())
  }

  // spt playback -s
  async fn get_status(&mut self, format: String) -> Result<String> {
    // Update info on current playback
    self
      .0
      .handle_network_event(IoEvent::GetCurrentPlayback)
      .await;
    self
      .0
      .handle_network_event(IoEvent::GetCurrentSavedTracks(None))
      .await;

    let context = self
      .0
      .app
      .lock()
      .await
      .current_playback_context
      .clone()
      .ok_or_else(|| anyhow!("no context avaible"))?;

    let playing_item = context.item.ok_or_else(|| anyhow!("no track playing"))?;

    let mut hs = match playing_item {
      PlayingItem::Track(track) => {
        let id = track.id.clone().unwrap_or_default();
        let mut hs = Format::from_type(FormatType::Track(Box::new(track)));
        hs.push(Format::Flags((
          context.repeat_state,
          context.shuffle_state,
          self.0.app.lock().await.liked_song_ids_set.contains(&id),
        )));
        hs
      }
      PlayingItem::Episode(episode) => {
        let mut hs = Format::from_type(FormatType::Episode(Box::new(episode)));
        hs.push(Format::Flags((
          context.repeat_state,
          context.shuffle_state,
          false,
        )));
        hs
      }
    };

    hs.push(Format::Device(context.device.name));
    hs.push(Format::Volume(context.device.volume_percent));
    hs.push(Format::Playing(context.is_playing));

    Ok(format_output(format, hs))
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
  async fn play(&mut self, name: String, item: Type, queue: bool) -> Result<()> {
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
            return Err(anyhow!("no tracks with name {}", name));
          }
        }
        Type::Album => {
          if let Some(r) = &results.albums {
            let album = &r.items[0];
            if let Some(uri) = &album.uri {
              uri.clone()
            } else {
              return Err(anyhow!("album {} has no uri", album.name));
            }
          } else {
            return Err(anyhow!("no albums with name {}", name));
          }
        }
        Type::Artist => {
          if let Some(r) = &results.artists {
            r.items[0].uri.clone()
          } else {
            return Err(anyhow!("no artists with name {}", name));
          }
        }
        Type::Show => {
          if let Some(r) = &results.shows {
            r.items[0].uri.clone()
          } else {
            return Err(anyhow!("no shows with name {}", name));
          }
        }
        Type::Playlist => {
          if let Some(r) = &results.playlists {
            r.items[0].uri.clone()
          } else {
            return Err(anyhow!("no playlists with name {}", name));
          }
        }
        _ => String::new(),
      }
    };

    // Play or queue the uri
    if queue {
      self
        .0
        .handle_network_event(IoEvent::AddItemToQueue(uri))
        .await;
    } else {
      self.play_uri(uri).await;
    }

    Ok(())
  }

  // Query for a playlist, track, artist, shows and albums
  // Returns result and their respective uris (to play them)
  //
  // spt query -s SEARCH ...
  async fn query(&mut self, search: String, format: String, item: Type) -> String {
    self
      .0
      .handle_network_event(IoEvent::GetSearchResults(search.clone(), None))
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
              )
            })
            .collect::<Vec<String>>()
            .join("\n")
        } else {
          format!("no playlists with name {}", search)
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
              )
            })
            .collect::<Vec<String>>()
            .join("\n")
        } else {
          format!("no tracks with name {}", search)
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
              )
            })
            .collect::<Vec<String>>()
            .join("\n")
        } else {
          format!("no artists with name {}", search)
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
              )
            })
            .collect::<Vec<String>>()
            .join("\n")
        } else {
          format!("no shows with name {}", search)
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
              )
            })
            .collect::<Vec<String>>()
            .join("\n")
        } else {
          format!("no albums with name {}", search)
        }
      }
      // Never called, just here for compiler
      _ => String::new(),
    }
  }
}

pub async fn handle_matches(
  matches: &ArgMatches<'_>,
  cmd: String,
  net: Network<'_>,
) -> Result<String> {
  // Tuple struct
  let mut cli = CliApp(net);

  cli.0.handle_network_event(IoEvent::GetDevices).await;
  cli
    .0
    .handle_network_event(IoEvent::GetCurrentPlayback)
    .await;

  if let Some(d) = matches.value_of("device") {
    cli.set_device(d.to_string()).await?
  }

  // Evalute the subcommand
  let output = match cmd.as_str() {
    "playback" => {
      let format = matches.value_of("format").unwrap();
      // Run the action, and print out the status
      if matches.is_present("toggle") {
        cli.toggle_playback().await;
      } else if let Some(d) = matches.value_of("transfer") {
        let output = cli.transfer_playback(d).await;
        if !output.is_empty() {
          return Ok(output);
        }
      } else if matches.is_present("flags") {
        let flag = Flag::from_matches(matches);
        cli.mark(flag).await?;
      } else if matches.is_present("jumps") {
        let direction = JumpDirection::from_matches(matches);
        cli.jump(direction).await;
      } else if let Some(vol) = matches.value_of("volume") {
        cli.volume(vol.to_string()).await?;
      }
      cli.get_status(format.to_string()).await
    }
    "play" => {
      if let Some(uri) = matches.value_of("uri") {
        cli.play_uri(uri.to_string()).await;
      } else if let Some(name) = matches.value_of("name") {
        let category = Type::play_from_matches(matches);
        cli
          .play(name.to_string(), category, matches.is_present("queue"))
          .await?;
      }
      cli.get_status("%s %t - %a".to_string()).await
    }
    "query" => {
      if matches.is_present("list") {
        let format = matches.value_of("format").unwrap();
        let category = Type::list_from_matches(matches);
        Ok(cli.list(category, format).await)
      } else if let Some(search) = matches.value_of("search") {
        let format = matches.value_of("format").unwrap().to_string();
        let query_type = Type::search_from_matches(matches);
        Ok(cli.query(search.to_string(), format, query_type).await)
      // Never called, just here for the compiler
      } else {
        Ok(String::new())
      }
    }
    // Never called, just here for the compiler
    _ => Ok(String::new()),
  };

  // Check if there was an error
  let api_error = cli.0.app.lock().await.api_error.clone();
  if api_error.is_empty() {
    output
  } else {
    Err(anyhow!("{}", api_error))
  }
}
