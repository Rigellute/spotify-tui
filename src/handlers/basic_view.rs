use crate::{app::App, event::Key, network::IoEvent};
use rspotify::model::{context::CurrentlyPlaybackContext, PlayingItem};

pub fn handler(key: Key, app: &mut App) {
  if let Key::Char('s') = key {
    if let Some(CurrentlyPlaybackContext {
      item: Some(item), ..
    }) = app.current_playback_context.to_owned()
    {
      match item {
        PlayingItem::Track(track) => {
          if let Some(track_id) = track.id {
            app.dispatch(IoEvent::ToggleSaveTrack(track_id));
          }
        }
        PlayingItem::Episode(episode) => {
          app.dispatch(IoEvent::ToggleSaveTrack(episode.id));
        }
      };
    };
  }
}
