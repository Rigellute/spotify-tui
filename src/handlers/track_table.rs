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
    k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
    k if common_key_events::down_event(k) => {
      let next_index = common_key_events::on_down_press_handler(
        &app.track_table.tracks,
        Some(app.track_table.selected_index),
      );
      app.track_table.selected_index = next_index;
    }
    k if common_key_events::up_event(k) => {
      let next_index = common_key_events::on_up_press_handler(
        &app.track_table.tracks,
        Some(app.track_table.selected_index),
      );
      app.track_table.selected_index = next_index;
    }
    k if common_key_events::high_event(k) => {
      let next_index = common_key_events::on_high_press_handler();
      app.track_table.selected_index = next_index;
    }
    k if common_key_events::middle_event(k) => {
      let next_index = common_key_events::on_middle_press_handler(&app.track_table.tracks);
      app.track_table.selected_index = next_index;
    }
    k if common_key_events::low_event(k) => {
      let next_index = common_key_events::on_low_press_handler(&app.track_table.tracks);
      app.track_table.selected_index = next_index;
    }
    Key::Enter => {
      on_enter(app);
    }
    // Scroll down
    k if k == app.user_config.keys.next_page => {
      match &app.track_table.context {
        Some(context) => match context {
          TrackTableContext::MyPlaylists => {
            if let (Some(playlist_page), Some(selected_playlist_index)) = (
              &app.playlists.get_results(None),
              &app.selected_playlist_index,
            ) {
              if let Some(selected_playlist) =
                playlist_page.items.get(selected_playlist_index.to_owned())
              {
                if let Some(playlist_tracks) = &app.playlist_tracks {
                  if app.playlist_track_offset + app.large_search_limit < playlist_tracks.total {
                    app.playlist_track_offset += app.large_search_limit;
                    let playlist_id = selected_playlist.id.to_owned();
                    app.dispatch(IoEvent::GetPlaylistTracks(
                      playlist_id,
                      app.playlist_track_offset,
                    ));
                  }
                }
              }
            };
          }
          TrackTableContext::RecommendedTracks => {}
          TrackTableContext::SavedTracks => {
            app.get_current_user_saved_tracks_next();
          }
          TrackTableContext::AlbumSearch => {}
          TrackTableContext::PlaylistSearch => {}
          TrackTableContext::MadeForYou => {
            let (playlists, selected_playlist_index) =
              (&app.library.made_for_you_playlists, &app.made_for_you_index);

            if let Some(selected_playlist) = playlists
              .get_results(Some(0))
              .unwrap()
              .items
              .get(selected_playlist_index.to_owned())
            {
              if let Some(playlist_tracks) = &app.made_for_you_tracks {
                if app.made_for_you_offset + app.large_search_limit < playlist_tracks.total {
                  app.made_for_you_offset += app.large_search_limit;
                  let playlist_id = selected_playlist.id.to_owned();
                  app.dispatch(IoEvent::GetMadeForYouPlaylistTracks(
                    playlist_id,
                    app.made_for_you_offset,
                  ));
                }
              }
            }
          }
        },
        None => {}
      };
    }
    // Scroll up
    k if k == app.user_config.keys.previous_page => {
      match &app.track_table.context {
        Some(context) => match context {
          TrackTableContext::MyPlaylists => {
            if let (Some(playlist_page), Some(selected_playlist_index)) = (
              &app.playlists.get_results(None),
              &app.selected_playlist_index,
            ) {
              if app.playlist_track_offset >= app.large_search_limit {
                app.playlist_track_offset -= app.large_search_limit;
              };
              if let Some(selected_playlist) =
                playlist_page.items.get(selected_playlist_index.to_owned())
              {
                let playlist_id = selected_playlist.id.to_owned();
                app.dispatch(IoEvent::GetPlaylistTracks(
                  playlist_id,
                  app.playlist_track_offset,
                ));
              }
            };
          }
          TrackTableContext::RecommendedTracks => {}
          TrackTableContext::SavedTracks => {
            app.get_current_user_saved_tracks_previous();
          }
          TrackTableContext::AlbumSearch => {}
          TrackTableContext::PlaylistSearch => {}
          TrackTableContext::MadeForYou => {
            let (playlists, selected_playlist_index) = (
              &app
                .library
                .made_for_you_playlists
                .get_results(Some(0))
                .unwrap(),
              app.made_for_you_index,
            );
            if app.made_for_you_offset >= app.large_search_limit {
              app.made_for_you_offset -= app.large_search_limit;
            }
            if let Some(selected_playlist) = playlists.items.get(selected_playlist_index) {
              let playlist_id = selected_playlist.id.to_owned();
              app.dispatch(IoEvent::GetMadeForYouPlaylistTracks(
                playlist_id,
                app.made_for_you_offset,
              ));
            }
          }
        },
        None => {}
      };
    }
    Key::Char('s') => handle_save_track_event(app),
    Key::Char('S') => play_random_song(app),
    k if k == app.user_config.keys.jump_to_end => jump_to_end(app),
    k if k == app.user_config.keys.jump_to_start => jump_to_start(app),
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
        let (context_uri, track_json) = match (
          &app.selected_playlist_index,
          &app.playlists.get_results(None),
        ) {
          (Some(selected_playlist_index), Some(playlist_page)) => {
            if let Some(selected_playlist) =
              playlist_page.items.get(selected_playlist_index.to_owned())
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
            Some(thread_rng().gen_range(0..num_tracks)),
          ));
        }
      }
      TrackTableContext::RecommendedTracks => {}
      TrackTableContext::SavedTracks => {
        if let Some(saved_tracks) = &app.library.saved_tracks.get_results(None) {
          let track_uris: Vec<String> = saved_tracks
            .items
            .iter()
            .map(|item| item.track.uri.to_owned())
            .collect();
          let rand_idx = thread_rng().gen_range(0..track_uris.len());
          app.dispatch(IoEvent::StartPlayback(
            None,
            Some(track_uris),
            Some(rand_idx),
          ))
        }
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
            Some(thread_rng().gen_range(0..num_tracks)),
          ))
        }
      }
      TrackTableContext::MadeForYou => {
        if let Some(playlist) = &app
          .library
          .made_for_you_playlists
          .get_results(Some(0))
          .and_then(|playlist| playlist.items.get(app.made_for_you_index))
        {
          if let Some(num_tracks) = &playlist
            .tracks
            .get("total")
            .and_then(|total| -> Option<usize> { from_value(total.clone()).ok() })
          {
            let uri = Some(playlist.uri.clone());
            app.dispatch(IoEvent::StartPlayback(
              uri,
              None,
              Some(thread_rng().gen_range(0..*num_tracks)),
            ))
          };
        };
      }
    }
  };
}

fn handle_save_track_event(app: &mut App) {
  let (selected_index, tracks) = (&app.track_table.selected_index, &app.track_table.tracks);
  if let Some(track) = tracks.get(*selected_index) {
    if let Some(id) = &track.id {
      let id = id.to_string();
      app.dispatch(IoEvent::ToggleSaveTrack(id));
    };
  };
}

fn handle_recommended_tracks(app: &mut App) {
  let (selected_index, tracks) = (&app.track_table.selected_index, &app.track_table.tracks);
  if let Some(track) = tracks.get(*selected_index) {
    let first_track = track.clone();
    let track_id_list = track.id.as_ref().map(|id| vec![id.to_string()]);

    app.recommendations_context = Some(RecommendationsContext::Song);
    app.recommendations_seed = first_track.name.clone();
    app.get_recommendations_for_seed(None, track_id_list, Some(first_track));
  };
}

fn jump_to_end(app: &mut App) {
  match &app.track_table.context {
    Some(context) => match context {
      TrackTableContext::MyPlaylists => {
        if let (Some(playlists), Some(selected_playlist_index)) = (
          &app.playlists.get_results(None),
          &app.selected_playlist_index,
        ) {
          if let Some(selected_playlist) = playlists.items.get(selected_playlist_index.to_owned()) {
            let total_tracks = selected_playlist
              .tracks
              .get("total")
              .and_then(|total| total.as_u64())
              .expect("playlist.tracks object should have a total field")
              as u32;

            if app.large_search_limit < total_tracks {
              app.playlist_track_offset = total_tracks - (total_tracks % app.large_search_limit);
              let playlist_id = selected_playlist.id.to_owned();
              app.dispatch(IoEvent::GetPlaylistTracks(
                playlist_id,
                app.playlist_track_offset,
              ));
            }
          }
        }
      }
      TrackTableContext::RecommendedTracks => {}
      TrackTableContext::SavedTracks => {}
      TrackTableContext::AlbumSearch => {}
      TrackTableContext::PlaylistSearch => {}
      TrackTableContext::MadeForYou => {}
    },
    None => {}
  }
}

fn on_enter(app: &mut App) {
  let TrackTable {
    context,
    selected_index,
    tracks,
  } = &app.track_table;
  match &context {
    Some(context) => match context {
      TrackTableContext::MyPlaylists => {
        if let Some(_track) = tracks.get(*selected_index) {
          let context_uri = match (&app.active_playlist_index, &app.playlists.get_results(None)) {
            (Some(active_playlist_index), Some(playlist_page)) => playlist_page
              .items
              .get(active_playlist_index.to_owned())
              .map(|selected_playlist| selected_playlist.uri.to_owned()),
            _ => None,
          };

          app.dispatch(IoEvent::StartPlayback(
            context_uri,
            None,
            Some(app.track_table.selected_index + app.playlist_track_offset as usize),
          ));
        };
      }
      TrackTableContext::RecommendedTracks => {
        app.dispatch(IoEvent::StartPlayback(
          None,
          Some(
            app
              .recommended_tracks
              .iter()
              .map(|x| x.uri.clone())
              .collect::<Vec<String>>(),
          ),
          Some(app.track_table.selected_index),
        ));
      }
      TrackTableContext::SavedTracks => {
        if let Some(saved_tracks) = &app.library.saved_tracks.get_results(None) {
          let track_uris: Vec<String> = saved_tracks
            .items
            .iter()
            .map(|item| item.track.uri.to_owned())
            .collect();

          app.dispatch(IoEvent::StartPlayback(
            None,
            Some(track_uris),
            Some(app.track_table.selected_index),
          ));
        };
      }
      TrackTableContext::AlbumSearch => {}
      TrackTableContext::PlaylistSearch => {
        let TrackTable {
          selected_index,
          tracks,
          ..
        } = &app.track_table;
        if let Some(_track) = tracks.get(*selected_index) {
          let context_uri = match (
            &app.search_results.selected_playlists_index,
            &app.search_results.playlists,
          ) {
            (Some(selected_playlist_index), Some(playlist_result)) => playlist_result
              .items
              .get(selected_playlist_index.to_owned())
              .map(|selected_playlist| selected_playlist.uri.to_owned()),
            _ => None,
          };

          app.dispatch(IoEvent::StartPlayback(
            context_uri,
            None,
            Some(app.track_table.selected_index),
          ));
        };
      }
      TrackTableContext::MadeForYou => {
        if let Some(_track) = tracks.get(*selected_index) {
          let context_uri = Some(
            app
              .library
              .made_for_you_playlists
              .get_results(Some(0))
              .unwrap()
              .items
              .get(app.made_for_you_index)
              .unwrap()
              .uri
              .to_owned(),
          );

          app.dispatch(IoEvent::StartPlayback(
            context_uri,
            None,
            Some(app.track_table.selected_index + app.made_for_you_offset as usize),
          ));
        }
      }
    },
    None => {}
  };
}

fn on_queue(app: &mut App) {
  let TrackTable {
    context,
    selected_index,
    tracks,
  } = &app.track_table;
  match &context {
    Some(context) => match context {
      TrackTableContext::MyPlaylists => {
        if let Some(track) = tracks.get(*selected_index) {
          let uri = track.uri.clone();
          app.dispatch(IoEvent::AddItemToQueue(uri));
        };
      }
      TrackTableContext::RecommendedTracks => {
        if let Some(full_track) = app.recommended_tracks.get(app.track_table.selected_index) {
          let uri = full_track.uri.clone();
          app.dispatch(IoEvent::AddItemToQueue(uri));
        }
      }
      TrackTableContext::SavedTracks => {
        if let Some(page) = app.library.saved_tracks.get_results(None) {
          if let Some(saved_track) = page.items.get(app.track_table.selected_index) {
            let uri = saved_track.track.uri.clone();
            app.dispatch(IoEvent::AddItemToQueue(uri));
          }
        }
      }
      TrackTableContext::AlbumSearch => {}
      TrackTableContext::PlaylistSearch => {
        let TrackTable {
          selected_index,
          tracks,
          ..
        } = &app.track_table;
        if let Some(track) = tracks.get(*selected_index) {
          let uri = track.uri.clone();
          app.dispatch(IoEvent::AddItemToQueue(uri));
        };
      }
      TrackTableContext::MadeForYou => {
        if let Some(track) = tracks.get(*selected_index) {
          let uri = track.uri.clone();
          app.dispatch(IoEvent::AddItemToQueue(uri));
        }
      }
    },
    None => {}
  };
}

fn jump_to_start(app: &mut App) {
  match &app.track_table.context {
    Some(context) => match context {
      TrackTableContext::MyPlaylists => {
        if let (Some(selected_playlist_index), Some(playlist_page)) = (
          &app.selected_playlist_index,
          &app.playlists.get_results(None),
        ) {
          if let Some(selected_playlist) =
            playlist_page.items.get(selected_playlist_index.to_owned())
          {
            app.playlist_track_offset = 0;
            let playlist_id = selected_playlist.id.to_owned();
            app.dispatch(IoEvent::GetPlaylistTracks(
              playlist_id,
              app.playlist_track_offset,
            ));
          }
        }
      }
      TrackTableContext::RecommendedTracks => {}
      TrackTableContext::SavedTracks => {}
      TrackTableContext::AlbumSearch => {}
      TrackTableContext::PlaylistSearch => {}
      TrackTableContext::MadeForYou => {}
    },
    None => {}
  }
}
