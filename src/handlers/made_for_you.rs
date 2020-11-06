use super::{
  super::app::{App, TrackTableContext},
  common_key_events,
};
use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
  match key {
    key if common_key_events::is_list_navigation_key_event(key, app) => {
      app.library.made_for_you_playlists.selected_index = app
        .library
        .made_for_you_playlists
        .handle_list_navigation_event(key, app);
    }
    Key::Enter => {
      if let Some(selected_playlist) = app.library.made_for_you_playlists.get_selected_item() {
        let playlist_id = selected_playlist.id.to_owned();
        app.track_table.context = Some(TrackTableContext::MadeForYou);
        app.playlist_offset = 0;
        app.dispatch(IoEvent::GetMadeForYouPlaylistTracks(
          Some(playlist_id),
          app.made_for_you_offset,
        ));
      }
    }
    _ => {}
  }
}
