use super::{
  super::app::{App, TrackTableContext},
  common_key_events,
};
use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
    k if common_key_events::down_event(k) => {
      match &app.playlists {
        Some(p) => {
          if let Some(selected_playlist_index) = app.selected_playlist_index {
            let next_index =
              common_key_events::on_down_press_handler(&p.items, Some(selected_playlist_index));
            app.selected_playlist_index = Some(next_index);
          }
        }
        None => {}
      };
    }
    k if common_key_events::up_event(k) => {
      match &app.playlists {
        Some(p) => {
          let next_index =
            common_key_events::on_up_press_handler(&p.items, app.selected_playlist_index);
          app.selected_playlist_index = Some(next_index);
        }
        None => {}
      };
    }
    k if common_key_events::high_event(k) => {
      match &app.playlists {
        Some(_p) => {
          let next_index = common_key_events::on_high_press_handler();
          app.selected_playlist_index = Some(next_index);
        }
        None => {}
      };
    }
    k if common_key_events::middle_event(k) => {
      match &app.playlists {
        Some(p) => {
          let next_index = common_key_events::on_middle_press_handler(&p.items);
          app.selected_playlist_index = Some(next_index);
        }
        None => {}
      };
    }
    k if common_key_events::low_event(k) => {
      match &app.playlists {
        Some(p) => {
          let next_index = common_key_events::on_low_press_handler(&p.items);
          app.selected_playlist_index = Some(next_index);
        }
        None => {}
      };
    }
    Key::Enter => {
      if let (Some(playlists), Some(selected_playlist_index)) =
        (&app.playlists, &app.selected_playlist_index)
      {
        app.track_table.context = Some(TrackTableContext::MyPlaylists);
        app.playlist_offset = 0;
        if let Some(selected_playlist) = playlists.items.get(selected_playlist_index.to_owned()) {
          let playlist_id = selected_playlist.id.to_owned();
          app.dispatch(IoEvent::GetPlaylistTracks(playlist_id, app.playlist_offset));
        }
      };
    }
    Key::Char('D') => {
      app.user_unfollow_playlist();
    }
    _ => {}
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test() {}
}
