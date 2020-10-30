use crate::network::Network;
use crate::network::IoEvent;

use rspotify::model::{
    artist::SimplifiedArtist,
    PlayingItem
};
use clap::ArgMatches;

// Non-concurrent copy of app.toggle_playback
async fn toggle_playback(net: &mut Network<'_>) -> String {
    let context = net.app.lock().await.current_playback_context.clone();
    if let Some(c) = context {
        if c.is_playing {
            net.handle_network_event(IoEvent::PausePlayback).await;
            return "Paused playback".to_string()
        }
    }
    net.handle_network_event(IoEvent::StartPlayback(None, None, None)).await;
    "Started playback".to_string()
}

async fn list_playlists(net: &mut Network<'_>) -> String {
    net.handle_network_event(IoEvent::GetPlaylists).await;
    let mut output = String::new();
    if let Some(playlists) = &net.app.lock().await.playlists {
        for p in &playlists.items {
            output.push_str(format!(
                "{} ({})", p.name, p.uri
            ).as_str());
            output.push('\n');
        }
        output[..(output.len() - 1)].to_string()
    } else {
        "No playlists".to_string()
    }
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

fn join_artists(vec: Vec<SimplifiedArtist>) -> String {
    let mut output = String::new();
    for artist in vec {
        output.push_str(artist.name.as_str()); 
        output.push_str(", ");
    }
    output[..(output.len() - 2)].to_string()
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
        return Err("Err: no device avaible".to_string())
    }
    app.selected_device_index = selected_device_index;
    Ok(())
}

// Format is to be implemented
async fn get_status(net: &mut Network<'_>) -> String {
    let context = match net.app.lock().await.current_playback_context.clone() {
        Some(c) => c,
        None => return "Err: no context avaible".to_string()
    };
    
    let playing_status = if context.is_playing {
        "▶ "
    } else {
        "⏸ "
    };
    let playing_item = match context.item {
        Some(item) => {
            item
        },
        None => return "No track playing".to_string()
    };

    match playing_item {
        PlayingItem::Track(track) => {
            format!(
                "{} {} - {}",
                playing_status,
                track.name,
                join_artists(track.artists),
            )
        },
        PlayingItem::Episode(episode) => {
            format!(
                "{} {} - {}",
                playing_status,
                episode.name,
                episode.show.publisher
            ) 
        }
    }
}

async fn play_uri(net: &mut Network<'_>, uri: String, track: bool) -> String {
    // Track was requested
    if track {
        net.handle_network_event(
            IoEvent::StartPlayback(None, Some(vec![uri.clone()]), Some(0))
        ).await;
    } else {
        net.handle_network_event(
            IoEvent::StartPlayback(Some(uri.clone()), None, None)
        ).await;
    }
    format!("Started playback of {}", uri)
}

enum Query {
    Playlist,
    Track,
    Artist,
    Album,
    Show
}

impl Query {
    fn from_args(m: &ArgMatches<'_>) -> Self {
        for (k, v) in &m.args {
            if v.occurs >= 1 {
                match k { 
                    &"playlist" => return Self::Playlist,
                    &"track" => return Self::Track,
                    &"artist" => return Self::Artist,
                    &"album" => return Self::Album,
                    &"show" => return Self::Show,
                    _ => continue
                }
            }
        }
        // Search for tracks by default
        Self::Track 
    }
}

// Query for a playlist, track, artist, shows and albums
// Returns result and their respective uris (to play them)
async fn query(
    net: &mut Network<'_>,
    search: String,
    item: Query,
) -> String {
    net.handle_network_event(
        IoEvent::GetSearchResults(search, None)
    ).await;

    let mut output = String::new();
    let app = net.app.lock().await;
    match item {
        Query::Playlist => {
            if let Some(results) = &app.search_results.playlists {
                for r in &results.items {
                    output.push_str(format!(
                        "{} ({})\n", 
                        r.name, r.uri
                    ).as_str());
                }
            } else {
                return "No playlists found".to_string()
            }
        },
        Query::Track => {
            if let Some(results) = &app.search_results.tracks {
                for r in &results.items {
                    output.push_str(format!(
                        "{} - {} ({})\n", 
                        r.name, r.album.name, r.uri
                    ).as_str());
                }
            } else {
                return "No tracks found".to_string()
            }
        },
        Query::Artist => {
            if let Some(results) = &app.search_results.artists {
                for r in &results.items {
                    output.push_str(format!(
                        "{} ({})\n", 
                        r.name, r.uri
                    ).as_str());
                }
            } else {
                return "No artists found".to_string()
            }
        },
        Query::Show => {
            if let Some(results) = &app.search_results.shows {
                for r in &results.items {
                    output.push_str(format!(
                        "{} - {} ({})\n", 
                        r.name, r.publisher, r.uri
                    ).as_str());
                }
            } else {
                return "No shows found".to_string()
            }
        }
        Query::Album => {
            if let Some(results) = &app.search_results.albums {
                for r in &results.items {
                    output.push_str(format!(
                        "{} - {} ({})\n", 
                        r.name, join_artists(r.artists.clone()),
                        r.uri.as_ref().unwrap_or(&"no uri".to_string())
                    ).as_str());
                }
            } else {
                return "No albums found".to_string()
            }
        }
    }

    output
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
            return "Err: failed to set device".to_string()
        }
    }

    // Evalute the subcommand
    match cmd.as_str() {
        "toggle" => {
            toggle_playback(net).await
        },
        "list" => {
            if matches.is_present("devices") {
                list_devices(net).await
            } else if matches.is_present("playlists") {
                list_playlists(net).await
            // Never called, just here for the compiler 
            } else {
                String::new()
            }
        },
        "status" => {
            get_status(net).await
        },
        "play" => {
            if let Some(uri) = matches.value_of("URI") {
                if matches.is_present("playlist") {
                    play_uri(net, uri.to_string(), false).await
                } else {
                    // Play track by default
                    play_uri(net, uri.to_string(), true).await
                }
            // Never called, just here for the compiler 
            } else {
                String::new()
            }
        },
        "query" => {
            if let Some(search) = matches.value_of("SEARCH") {
                let query_type = Query::from_args(matches);
                query(net, search.to_string(), query_type).await
            } else {
                String::new()
            }
        },
        // Never called, just here for the compiler
        _ => {
            String::new()
        }
    }
}
