use crate::network::IoEvent;
use crate::network::Network;

use clap::ArgMatches;
use rspotify::{model::PlayingItem, senum::RepeatState};

enum Type {
  Playlist,
  Track,
  Artist,
  Album,
  Show,
}

impl Type {
  fn from_args(m: &ArgMatches<'_>) -> Self {
    for (k, v) in &m.args {
      if v.occurs >= 1 {
        match k {
          &"playlist" => return Self::Playlist,
          &"track" => return Self::Track,
          &"artist" => return Self::Artist,
          &"album" => return Self::Album,
          &"show" => return Self::Show,
          _ => continue,
        }
      }
    }
    // Search for tracks by default
    Self::Track
  }
}

enum Flag {
  Like,
  Shuffle,
  Repeat,
}

impl Flag {
  fn from_string(s: &str) -> Result<Self, String> {
    match s {
      "l" | "like" => Ok(Self::Like),
      "s" | "shuffle" => Ok(Self::Shuffle),
      "r" | "repeat" => Ok(Self::Repeat),
      _ => Err(format!("Err: no such flag '{}'", s)),
    }
  }
}

//
// Utils functions
//

fn format_output(
  format: String,
  album: Option<&str>,
  artist: Option<&str>,
  playlist: Option<&str>,
  track: Option<&str>,
  show: Option<&str>,
  uri: Option<&str>,
  device: Option<&str>,
  // (repeat, shuffle, like)
  flags: Option<(RepeatState, bool, bool)>,
  playing: bool,
) -> String {
  let flags_string = if let Some((r, s, l)) = flags {
    let shuffle = if s { "🔀" } else { "" };
    let repeat = match r {
      RepeatState::Off => "",
      RepeatState::Track => "🔂",
      RepeatState::Context => "🔁",
    };
    let like = if l { "❤️ " } else { "" };
    format!("{} {} {}", shuffle, repeat, like)
  } else {
    "".to_string()
  };
  let playing_string = if playing { "▶ " } else { "⏸ " };

  format
    .replace("%l", album.unwrap_or("None"))
    .replace("%a", artist.unwrap_or("None"))
    .replace("%p", playlist.unwrap_or("None"))
    .replace("%t", track.unwrap_or("None"))
    .replace("%h", show.unwrap_or("None"))
    .replace("%u", uri.unwrap_or("None"))
    .replace("%d", device.unwrap_or("None"))
    .replace("%f", &flags_string)
    .replace("%s", &playing_string)
}

//
// Commands
//

// For "spt toggle"
// Non-concurrent copy of app.toggle_playback
async fn toggle_playback(net: &mut Network<'_>) {
  let context = net.app.lock().await.current_playback_context.clone();
  if let Some(c) = context {
    if c.is_playing {
      net.handle_network_event(IoEvent::PausePlayback).await;
      return;
    }
  }
  net
    .handle_network_event(IoEvent::StartPlayback(None, None, None))
    .await;
}

async fn list_playlists(net: &mut Network<'_>, format: String) -> String {
  net.handle_network_event(IoEvent::GetPlaylists).await;
  let mut output = String::new();
  if let Some(playlists) = &net.app.lock().await.playlists {
    for p in &playlists.items {
      output.push_str(
        format_output(
          format.clone(),
          None,
          None,
          Some(&p.name),
          None,
          None,
          Some(&p.uri),
          None,
          None,
          false,
        )
        .as_str(),
      );
      output.push('\n');
    }
    // Remove the last newline
    output[..(output.len() - 1)].to_string()
  } else {
    "No playlists".to_string()
  }
}

async fn mark(net: &mut Network<'_>, flag: Flag) -> Result<(), String> {
  let c = {
    let app = net.app.lock().await;
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
      net.handle_network_event(IoEvent::ToggleSaveTrack(id)).await;
    }
    Flag::Shuffle => {
      net
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
      net.handle_network_event(IoEvent::Repeat(r)).await;
    }
    _ => {}
  }

  Ok(())
}

async fn list_devices(net: &mut Network<'_>) -> String {
  if let Some(devices) = &net.app.lock().await.devices {
    let mut output = String::new();
    for d in &devices.devices {
      output.push_str(d.name.as_str());
      output.push('\n');
    }
    // Remove the last unnecessary \n
    output[..(output.len() - 1)].to_string()
  } else {
    "No devices avaible".to_string()
  }
}

async fn set_device(net: &mut Network<'_>, name: String) -> Result<(), String> {
  // Change the device if specified by user
  let mut app = net.app.lock().await;
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

// Format is to be implemented
async fn get_status(net: &mut Network<'_>, format: String) -> String {
  // Update info on current playback
  net.handle_network_event(IoEvent::GetCurrentPlayback).await;
  net
    .handle_network_event(IoEvent::GetCurrentSavedTracks(None))
    .await;

  let context = match net.app.lock().await.current_playback_context.clone() {
    Some(c) => c,
    None => return "Err: no context avaible".to_string(),
  };

  let playing_item = match context.item {
    Some(item) => item,
    None => return "No track playing".to_string(),
  };

  match playing_item {
    PlayingItem::Track(track) => format_output(
      format,
      Some(&track.album.name),
      Some(
        &track
          .artists
          .iter()
          .map(|a| a.name.as_str())
          .collect::<Vec<&str>>()
          .join(", "),
      ),
      None,
      Some(&track.name),
      None,
      Some(&track.uri),
      Some(&context.device.name),
      Some((
        context.repeat_state,
        context.shuffle_state,
        net
          .app
          .lock()
          .await
          .liked_song_ids_set
          .contains(&track.id.unwrap_or(String::new())),
      )),
      context.is_playing,
    ),
    PlayingItem::Episode(episode) => format_output(
      format,
      Some(&episode.show.name),
      Some(&episode.show.publisher),
      None,
      Some(&episode.name),
      None,
      Some(&episode.uri),
      Some(&context.device.name),
      Some((context.repeat_state, context.shuffle_state, false)),
      context.is_playing,
    ),
  }
}

async fn play_uri(net: &mut Network<'_>, uri: String, track: bool) {
  // Track was requested
  if track {
    net
      .handle_network_event(IoEvent::StartPlayback(
        None,
        Some(vec![uri.clone()]),
        Some(0),
      ))
      .await;
  } else {
    net
      .handle_network_event(IoEvent::StartPlayback(Some(uri.clone()), None, None))
      .await;
  }
}

// Query for a playlist, track, artist, shows and albums
// Returns result and their respective uris (to play them)
async fn query(net: &mut Network<'_>, search: String, format: String, item: Type) -> String {
  net
    .handle_network_event(IoEvent::GetSearchResults(search, None))
    .await;

  let mut output = String::new();
  let app = net.app.lock().await;
  match item {
    Type::Playlist => {
      if let Some(results) = &app.search_results.playlists {
        for r in &results.items {
          output.push_str(
            format_output(
              format.clone(),
              None,
              None,
              Some(&r.name),
              None,
              None,
              Some(&r.uri),
              None,
              None,
              false,
            )
            .as_str(),
          );
          output.push('\n');
        }
      } else {
        return "No playlists found".to_string();
      }
    }
    Type::Track => {
      if let Some(results) = &app.search_results.tracks {
        for r in &results.items {
          output.push_str(
            format_output(
              format.clone(),
              Some(&r.album.name),
              Some(
                &r.artists
                  .iter()
                  .map(|a| a.name.as_str())
                  .collect::<Vec<&str>>()
                  .join(", "),
              ),
              None,
              Some(&r.name),
              None,
              Some(&r.uri),
              None,
              None,
              false,
            )
            .as_str(),
          );
          output.push('\n');
        }
      } else {
        return "No tracks found".to_string();
      }
    }
    Type::Artist => {
      if let Some(results) = &app.search_results.artists {
        for r in &results.items {
          output.push_str(
            format_output(
              format.clone(),
              None,
              Some(&r.name),
              None,
              None,
              None,
              Some(&r.uri),
              None,
              None,
              false,
            )
            .as_str(),
          );
          output.push('\n');
        }
      } else {
        return "No artists found".to_string();
      }
    }
    Type::Show => {
      if let Some(results) = &app.search_results.shows {
        for r in &results.items {
          output.push_str(
            format_output(
              format.clone(),
              None,
              Some(&r.publisher),
              None,
              None,
              Some(&r.name),
              Some(&r.uri),
              None,
              None,
              false,
            )
            .as_str(),
          );
          output.push('\n');
        }
      } else {
        return "No shows found".to_string();
      }
    }
    Type::Album => {
      if let Some(results) = &app.search_results.albums {
        for r in &results.items {
          output.push_str(
            format_output(
              format.clone(),
              Some(&r.name),
              Some(
                &r.artists
                  .iter()
                  .map(|a| a.name.as_str())
                  .collect::<Vec<&str>>()
                  .join(", "),
              ),
              None,
              None,
              None,
              // This is actually already an Option<>
              r.uri.as_deref(),
              None,
              None,
              false,
            )
            .as_str(),
          );
          output.push('\n');
        }
      } else {
        return "No albums found".to_string();
      }
    }
  }

  // Remove the last newline
  output[..(output.len() - 1)].to_string()
}

async fn transfer_playback(net: &mut Network<'_>, device: &str) -> String {
  // Get the device id by name
  let mut id = String::new();
  if let Some(devices) = &net.app.lock().await.devices {
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
    net
      .handle_network_event(IoEvent::TransferPlaybackToDevice(id.to_string()))
      .await;
    String::new()
  }
}

pub async fn handle_matches(
  matches: &ArgMatches<'_>,
  cmd: String,
  net: &mut Network<'_>,
) -> String {
  // Query devices
  net.handle_network_event(IoEvent::GetDevices).await;
  net.handle_network_event(IoEvent::GetCurrentPlayback).await;

  if let Some(d) = matches.value_of("device") {
    if set_device(net, d.to_string()).await.is_err() {
      return "Err: failed to set device".to_string();
    }
  }

  // Evalute the subcommand
  let output = match cmd.as_str() {
    "playback" => {
      let format = matches.value_of("format").unwrap();
      if matches.is_present("toggle") {
        toggle_playback(net).await;
      } else if let Some(d) = matches.value_of("transfer") {
        let output = transfer_playback(net, d).await;
        if !output.is_empty() {
          return output;
        }
      } else if let Some(f) = matches.value_of("mark") {
        let flag = match Flag::from_string(f) {
          Ok(r) => r,
          Err(e) => return e,
        };
        if let Err(e) = mark(net, flag).await {
          return e;
        }
      }

      get_status(net, format.to_string()).await
    }
    "list" => {
      if matches.is_present("devices") {
        list_devices(net).await
      } else if matches.is_present("playlists") {
        let format = matches.value_of("format").unwrap().to_string();
        list_playlists(net, format).await
      // Never called, just here for the compiler
      } else {
        String::new()
      }
    }
    "status" => {
      // Does not panic because it has a default value
      let format = matches.value_of("format").unwrap().to_string();
      get_status(net, format).await
    }
    "play" => {
      if let Some(uri) = matches.value_of("URI") {
        if matches.is_present("context") {
          play_uri(net, uri.to_string(), false).await;
        } else {
          // Play track by default
          play_uri(net, uri.to_string(), true).await;
        }
        get_status(net, "%s %t - %a".to_string()).await
      // Never called, just here for the compiler
      } else {
        String::new()
      }
    }
    "query" => {
      let format = matches.value_of("format").unwrap().to_string();
      if let Some(search) = matches.value_of("SEARCH") {
        let query_type = Type::from_args(matches);
        query(net, search.to_string(), format, query_type).await
      } else {
        String::new()
      }
    }
    // Never called, just here for the compiler
    _ => String::new(),
  };

  // Check if there was an error
  let api_error = net.app.lock().await.api_error.clone();
  if api_error.is_empty() {
    output
  } else {
    format!("Err: {}", api_error)
  }
}
