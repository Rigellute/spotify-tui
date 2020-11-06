use super::{
  super::app::{App, RecommendationsContext, TrackTable, TrackTableContext},
  common_key_events,
};
use crate::event::Key;
use crate::network::IoEvent;
use rand::{thread_rng, Rng};
use serde_json::from_value;

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::is_list_navigation_key_event(k, app) => {
      app.track_table.tracks.selected_index = app
        .track_table
        .tracks
        .handle_list_navigation_event(key, app);
    }
    Key::Char('s') => handle_save_track_event(app),
    Key::Char('S') => play_random_song(app),
    //recommended song radio
    Key::Char('r') => {
      handle_recommended_tracks(app);
    }
    _ if key == app.user_config.keys.add_item_to_queue => on_queue(app),
    _ => {}
  }
}

fn play_random_song(app: &mut App) {
  if let Some(context) = &app.track_table.context {
    match context {
      TrackTableContext::MyPlaylists => {
        let (context_uri, track_json) = match (&app.selected_playlist_index, &app.playlists) {
          (Some(selected_playlist_index), Some(playlists)) => {
            if let Some(selected_playlist) = playlists.items.get(selected_playlist_index.to_owned())
            {
              (
                Some(selected_playlist.uri.to_owned()),
                selected_playlist.tracks.get("total"),
              )
            } else {
              (None, None)
            }
          }
          _ => (None, None),
        };

        if let Some(val) = track_json {
          let num_tracks: usize = from_value(val.clone()).unwrap();
          app.dispatch(IoEvent::StartPlayback(
            context_uri,
            None,
            Some(thread_rng().gen_range(0, num_tracks)),
          ));
        }
      }
      TrackTableContext::RecommendedTracks => {}
      TrackTableContext::SavedTracks => {
        let track_uris: Vec<String> = app
          .library
          .saved_tracks
          .items
          .iter()
          .map(|item| item.track.uri.to_owned())
          .collect();
        let rand_idx = thread_rng().gen_range(0, track_uris.len());
        app.dispatch(IoEvent::StartPlayback(
          None,
          Some(track_uris),
          Some(rand_idx),
        ))
      }
      TrackTableContext::AlbumSearch => {}
      TrackTableContext::PlaylistSearch => {
        let (context_uri, playlist_track_json) = match (
          &app.search_results.selected_playlists_index,
          &app.search_results.playlists,
        ) {
          (Some(selected_playlist_index), Some(playlist_result)) => {
            if let Some(selected_playlist) = playlist_result
              .items
              .get(selected_playlist_index.to_owned())
            {
              (
                Some(selected_playlist.uri.to_owned()),
                selected_playlist.tracks.get("total"),
              )
            } else {
              (None, None)
            }
          }
          _ => (None, None),
        };
        if let Some(val) = playlist_track_json {
          let num_tracks: usize = from_value(val.clone()).unwrap();
          app.dispatch(IoEvent::StartPlayback(
            context_uri,
            None,
            Some(thread_rng().gen_range(0, num_tracks)),
          ))
        }
      }
      TrackTableContext::MadeForYou => {
        if let Some(playlist) = app.library.made_for_you_playlists.get_selected_item() {
          if let Some(num_tracks) = &playlist
            .tracks
            .get("total")
            .and_then(|total| -> Option<usize> { from_value(total.clone()).ok() })
          {
            let uri = Some(playlist.uri.clone());
            app.dispatch(IoEvent::StartPlayback(
              uri,
              None,
              Some(thread_rng().gen_range(0, num_tracks)),
            ))
          };
        };
      }
    }
  };
}

fn handle_save_track_event(app: &mut App) {
  if let Some(track) = app.track_table.tracks.get_selected_item() {
    if let Some(id) = &track.id {
      let id = id.to_string();
      app.dispatch(IoEvent::ToggleSaveTrack(id));
    };
  };
}

fn handle_recommended_tracks(app: &mut App) {
  if let Some(track) = app.track_table.tracks.get_selected_item() {
    let first_track = track.clone();
    let track_id_list: Option<Vec<String>> = match &track.id {
      Some(id) => Some(vec![id.to_string()]),
      None => None,
    };
    app.recommendations_context = Some(RecommendationsContext::Song);
    app.recommendations_seed = first_track.name.clone();
    app.get_recommendations_for_seed(None, track_id_list, Some(first_track));
  };
}

fn on_enter(app: &mut App) {}
//fn on_enter(app: &mut App) {
//let TrackTable {
//made_for_you_tracks: _,
//context,
//tracks,
//} = &app.track_table;
//match &context {
//Some(context) => match context {
//TrackTableContext::MyPlaylists => {
//if let Some(_track) = app.track_table.tracks.get_selected_item() {
////TODO: Replace this with by storing the playlist_id in the TrackTable directly (maybe
////wrapped in an enum
//let context_uri = match (&app.selected_playlist_index, &app.playlists) {
//(Some(selected_playlist_index), Some(playlists)) => {
//if let Some(selected_playlist) =
//playlists.items.get(selected_playlist_index.to_owned())
//{
//Some(selected_playlist.uri.to_owned())
//} else {
//None
//}
//}
//_ => None,
//};

//app.dispatch(IoEvent::StartPlayback(
//context_uri,
//None,
//Some(tracks.selected_index + app.playlist_offset as usize),
//));
//};
//}
//TrackTableContext::RecommendedTracks => {
//app.dispatch(IoEvent::StartPlayback(
//None,
//Some(
//app
//.recommended_tracks
//.iter()
//.map(|x| x.uri.clone())
//.collect::<Vec<String>>(),
//),
//Some(tracks.selected_index),
//));
//}
//TrackTableContext::SavedTracks => {
//let track_uris: Vec<String> = app
//.library
//.saved_tracks
//.items
//.iter()
//.map(|item| item.track.uri.to_owned())
//.collect();

//app.dispatch(IoEvent::StartPlayback(
//None,
//Some(track_uris),
//Some(tracks.selected_index),
//));
//}
//TrackTableContext::AlbumSearch => {}
//TrackTableContext::PlaylistSearch => {
//if let Some(_track) = tracks.get_selected_item() {
//let context_uri = match (
//&app.search_results.selected_playlists_index,
//&app.search_results.playlists,
//) {
//(Some(selected_playlist_index), Some(playlist_result)) => {
//if let Some(selected_playlist) = playlist_result
//.items
//.get(selected_playlist_index.to_owned())
//{
//Some(selected_playlist.uri.to_owned())
//} else {
//None
//}
//}
//_ => None,
//};

//app.dispatch(IoEvent::StartPlayback(
//context_uri,
//None,
//Some(tracks.selected_index),
//));
//};
//}
//TrackTableContext::MadeForYou => {
//if let Some(_track) = tracks.get_selected_item() {
//if let Some(playlist) = app.library.made_for_you_playlists.get_selected_item() {
//let context_uri = Some(playlist.uri.to_owned());

//app.dispatch(IoEvent::StartPlayback(
//context_uri,
//None,
//Some(tracks.selected_index + app.made_for_you_offset as usize),
//));
//}
//}
//}
//},
//None => {}
//};
//}

fn on_queue(app: &mut App) {}
//fn on_queue(app: &mut App) {
//let TrackTable {
//made_for_you_tracks: _,
//context,
//tracks,
//} = &app.track_table;
//match &context {
//Some(context) => match context {
//TrackTableContext::MyPlaylists => {
//if let Some(track) = tracks.get_selected_item() {
//let uri = track.uri.clone();
//app.dispatch(IoEvent::AddItemToQueue(uri));
//};
//}
//TrackTableContext::RecommendedTracks => {
//if let Some(full_track) = tracks.get_selected_item() {
//let uri = full_track.uri.clone();
//app.dispatch(IoEvent::AddItemToQueue(uri));
//}
//}
//TrackTableContext::SavedTracks => {
//if let Some(saved_track) = app.library.saved_tracks.get_selected_item() {
//let uri = saved_track.track.uri.clone();
//app.dispatch(IoEvent::AddItemToQueue(uri));
//}
//}
//TrackTableContext::AlbumSearch => {}
//TrackTableContext::PlaylistSearch => {
//let TrackTable { tracks, .. } = &app.track_table;
//if let Some(track) = tracks.get_selected_item() {
//let uri = track.uri.clone();
//app.dispatch(IoEvent::AddItemToQueue(uri));
//};
//}
//TrackTableContext::MadeForYou => {
//if let Some(track) = tracks.get_selected_item() {
//let uri = track.uri.clone();
//app.dispatch(IoEvent::AddItemToQueue(uri));
//}
//}
//},
//None => {}
//};
//}
