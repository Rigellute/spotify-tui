use super::{
  super::app::{App, DialogContext, TrackTableContext},
  common_key_events,
};
use crate::app::{ActiveBlock, RouteId};
use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
    k if common_key_events::down_event(k) => match &app.playlists.get_results(None) {
      Some(p) => {
        if let Some(selected_playlist_index) = app.selected_playlist_index {
          let next_index =
            common_key_events::on_down_press_handler(&p.items, Some(selected_playlist_index));
          app.selected_playlist_index = Some(next_index);
        }
      }
      None => {}
    },
    k if common_key_events::up_event(k) => match &app.playlists.get_results(None) {
      Some(playlist_page) => {
        let next_index =
          common_key_events::on_up_press_handler(&playlist_page.items, app.selected_playlist_index);
        app.selected_playlist_index = Some(next_index);
      }
      None => {}
    },
    k if common_key_events::high_event(k) => {
      match &app.playlists.get_results(None) {
        Some(_p) => {
          let next_index = common_key_events::on_high_press_handler();
          app.selected_playlist_index = Some(next_index);
        }
        None => {}
      };
    }
    k if common_key_events::middle_event(k) => {
      match &app.playlists.get_results(None) {
        Some(p) => {
          let next_index = common_key_events::on_middle_press_handler(&p.items);
          app.selected_playlist_index = Some(next_index);
        }
        None => {}
      };
    }
    k if common_key_events::low_event(k) => {
      match &app.playlists.get_results(None) {
        Some(p) => {
          let next_index = common_key_events::on_low_press_handler(&p.items);
          app.selected_playlist_index = Some(next_index);
        }
        None => {}
      };
    }
    Key::Enter => {
      if let (Some(playlists), Some(selected_playlist_index)) = (
        &app.playlists.get_results(None),
        &app.selected_playlist_index,
      ) {
        app.active_playlist_index = Some(selected_playlist_index.to_owned());
        app.track_table.context = Some(TrackTableContext::MyPlaylists);
        app.playlist_track_offset = 0;
        if let Some(selected_playlist) = playlists.items.get(selected_playlist_index.to_owned()) {
          let playlist_id = selected_playlist.id.to_owned();
          app.dispatch(IoEvent::GetPlaylistTracks(
            playlist_id,
            app.playlist_track_offset,
          ));
        }
      };
    }

    k if k == app.user_config.keys.previous_page => {
      if app.playlists.index() > 0 {
        app.playlists.previous_page();
        app.selected_playlist_index = Some(0)
      }
    }
    k if k == app.user_config.keys.next_page => {
      match app.playlists.get_results(Some(app.playlists.index() + 1)) {
        Some(_) => {
          app.playlists.next_page();
          app.selected_playlist_index = Some(0);
        }
        None => {
          if let Some(saved_playlists) = &app.playlists.get_results(None) {
            let offset = Some(saved_playlists.offset + saved_playlists.limit);
            app.dispatch(IoEvent::GetPlaylists(offset));
          }
        }
      }
    }
    Key::Char('D') => {
      if let (Some(playlists), Some(selected_index)) = (
        &app.playlists.get_results(None),
        &app.selected_playlist_index,
      ) {
        let selected_playlist = &playlists.items[selected_index.to_owned()].name;
        app.dialog = Some(selected_playlist.clone());
        app.confirm = false;
        app.push_navigation_stack(
          RouteId::Dialog,
          ActiveBlock::Dialog(DialogContext::PlaylistWindow),
        );
      }
    }
    _ => {}
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test() {}
}
