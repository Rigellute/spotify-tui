use crate::{app::App, event::Key, network::IoEvent};

pub fn handler(key: Key, app: &mut App) {
  match key {
    Key::Char('s') => {
      if let Some(playing_context) = &app.current_playback_context {
        if let Some(track) = &playing_context.clone().item {
          if let Some(id) = &track.id {
            app.dispatch(IoEvent::ToggleSaveTrack(id.to_string()));
          }
        }
      }
    }
    _ => {}
  };
}
