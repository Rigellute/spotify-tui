use super::{
  super::app::{App, TrackTableContext},
  common_key_events,
};
use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
    k if common_key_events::up_event(k) => {
      if let Some(playlists) = &mut app.library.made_for_you_playlists.get_results(None) {
        let next_index =
          common_key_events::on_up_press_handler(&playlists.items, Some(app.made_for_you_index));
        app.made_for_you_index = next_index;
      }
    }
    k if common_key_events::down_event(k) => {
      if let Some(playlists) = &mut app.library.made_for_you_playlists.get_results(None) {
        let next_index =
          common_key_events::on_down_press_handler(&playlists.items, Some(app.made_for_you_index));
        app.made_for_you_index = next_index;
      }
    }
    k if common_key_events::high_event(k) => {
      if let Some(_playlists) = &mut app.library.made_for_you_playlists.get_results(None) {
        let next_index = common_key_events::on_high_press_handler();
        app.made_for_you_index = next_index;
      }
    }
    k if common_key_events::middle_event(k) => {
      if let Some(playlists) = &mut app.library.made_for_you_playlists.get_results(None) {
        let next_index = common_key_events::on_middle_press_handler(&playlists.items);
        app.made_for_you_index = next_index;
      }
    }
    k if common_key_events::low_event(k) => {
      if let Some(playlists) = &mut app.library.made_for_you_playlists.get_results(None) {
        let next_index = common_key_events::on_low_press_handler(&playlists.items);
        app.made_for_you_index = next_index;
      }
    }
    Key::Enter => {
      if let (Some(playlists), selected_playlist_index) = (
        &app.library.made_for_you_playlists.get_results(Some(0)),
        &app.made_for_you_index,
      ) {
        app.track_table.context = Some(TrackTableContext::MadeForYou);
        app.playlist_track_offset = 0;
        if let Some(selected_playlist) = playlists.items.get(selected_playlist_index.to_owned()) {
          app.made_for_you_offset = 0;
          let playlist_id = selected_playlist.id.to_owned();
          app.dispatch(IoEvent::GetMadeForYouPlaylistTracks(
            playlist_id,
            app.made_for_you_offset,
          ));
        }
      };
    }
    _ => {}
  }
}
