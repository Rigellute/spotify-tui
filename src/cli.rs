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

async fn list_devices(net: &mut Network<'_>) -> String {
    if let Some(devices) = &net.app.lock().await.devices {
        let mut devices_string = String::new();
        for d in &devices.devices {
            devices_string.push_str(d.name.as_str());
            devices_string.push_str("\n");
        }
        devices_string[..(devices_string.len() - 1)].to_string()
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

async fn set_device(net: &mut Network<'_>, matches: &ArgMatches<'_>) -> Result<(), String> {
    // Change the device if specified by user
    let mut app = net.app.lock().await;
    let mut selected_device_index = Some(0);
    if let Some(dp) = &app.devices {
        if let Some(device_name) = matches.value_of("device") {
            for (i, d) in dp.devices.iter().enumerate() {
                if d.name == device_name {
                    selected_device_index = Some(i);
                }
            }
        }
    } else {
        // Error out if no device is avaible
        return Err("Err: no device avaible".to_string())
    }
    app.selected_device_index = selected_device_index;
    Ok(())
}

async fn get_status(net: &mut Network<'_>) -> String {
    let context = match net.app.lock().await.current_playback_context.clone() {
        Some(c) => c,
        None => return "Err: no context avaible".to_string()
    };
    
    let playing_status = if context.is_playing {
        "▶"
    } else {
        "⏸"
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

async fn play_track(net: &mut Network<'_>, uri: String) -> String {
    net.handle_network_event(
        IoEvent::StartPlayback(Some(uri.clone()), None, None)
    ).await;
    format!("Started playback of {}", uri)
}

enum Query {
    Playlist,
    Track,
    Artist,
    Album,
    Show
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
    net: &mut Network<'_>,
) -> String {
    // Query devices
    net.handle_network_event(IoEvent::GetDevices).await;

    if matches.is_present("list-devices") {
        return list_devices(net).await
    }

    // Update the playback and the default device
    if let Err(e) = set_device(net, matches).await {
        return e
    }
    net.handle_network_event(IoEvent::GetCurrentPlayback).await;

    if let Some(search) = matches.value_of("query") {
        // Get type of query (to be implemented)
        return query(net, search.to_string(), Query::Track).await
    } else if matches.is_present("status") {
        return get_status(net).await
    } else if matches.is_present("toggle") {
        return toggle_playback(net).await
    } else if let Some(uri) = matches.value_of("play") {
        return play_track(net, uri.to_string()).await
    }

    String::new()
}
