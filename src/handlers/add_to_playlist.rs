use crate::app::{App, RouteId};
use crate::event::Key;
use crate::network::IoEvent;

use super::common_key_events;

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::down_event(k) => {
      match &app.playlists {
        Some(p) => {
          if let Some(selected_add_to_playlist_index) = app.selected_add_to_playlist_index {
            let next_index = common_key_events::on_down_press_handler(
              &p.items,
              Some(selected_add_to_playlist_index),
            );
            app.selected_add_to_playlist_index = Some(next_index);
          }
        }
        None => {}
      };
    }
    k if common_key_events::up_event(k) => {
      match &app.playlists {
        Some(p) => {
          let next_index =
            common_key_events::on_up_press_handler(&p.items, app.selected_add_to_playlist_index);
          app.selected_add_to_playlist_index = Some(next_index);
        }
        None => {}
      };
    }
    Key::Enter => {
      add_to_playlist(app);
    }
    _ => {}
  };
}

fn add_to_playlist(app: &mut App) {
  if let (RouteId::AddToPlaylist(track_id), Some(playlist_index), Some(playlists)) = (
    &app.get_current_route().id,
    app.selected_add_to_playlist_index,
    &app.playlists,
  ) {
    if let Some(playlist) = playlists.items.get(playlist_index) {
      let playlist_id = &playlist.id;
      app.dispatch(IoEvent::AddToPlaylist {
        track_id: track_id.to_owned(),
        playlist_id: playlist_id.to_owned(),
      });
      app.pop_navigation_stack();
    }
  }
}
