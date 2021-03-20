use super::{
  super::app::{App, DialogContext, TrackTableContext},
  common_key_events,
};
use crate::app::ActiveBlock;
use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
    k if common_key_events::down_event(k) => {
      match &app.playlists {
        Some(p) => {
          if let Some(selected_playlist_index) = app.selected_playlist_index {
            let next_index = common_key_events::on_down_press_handler(
              &p.items,
              Some(selected_playlist_index),
              &mut app.movement_count,
            );
            app.selected_playlist_index = Some(next_index);
          }
        }
        None => {}
      };
    }
    k if common_key_events::up_event(k) => {
      match &app.playlists {
        Some(p) => {
          let next_index = common_key_events::on_up_press_handler(
            &p.items,
            app.selected_playlist_index,
            &mut app.movement_count,
          );
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
    k if common_key_events::count_event(k) => common_key_events::handle_count_event(k, app),
    Key::Enter => {
      if let (Some(playlists), Some(selected_playlist_index)) =
        (&app.playlists, &app.selected_playlist_index)
      {
        app.active_playlist_index = Some(selected_playlist_index.to_owned());
        app.track_table.context = Some(TrackTableContext::MyPlaylists);
        app.playlist_offset = 0;
        if let Some(selected_playlist) = playlists.items.get(selected_playlist_index.to_owned()) {
          let playlist_id = selected_playlist.id.to_owned();
          app.dispatch(IoEvent::GetPlaylistTracks(playlist_id, app.playlist_offset));
        }
      };
    }
    Key::Char('D') => {
      if let (Some(playlists), Some(selected_index)) = (&app.playlists, app.selected_playlist_index)
      {
        let selected_playlist = &playlists.items[selected_index].name;
        app.dialog = Some(selected_playlist.clone());
        app.confirm = false;

        let route = app.get_current_route().id.clone();
        app.push_navigation_stack(route, ActiveBlock::Dialog(DialogContext::PlaylistWindow));
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
