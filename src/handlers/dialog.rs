use super::super::app::{ActiveBlock, App, DialogContext};
use crate::event::Key;

pub fn handler(key: Key, app: &mut App) {
  match key {
    Key::Enter => {
      if let Some(route) = app.pop_navigation_stack() {
        if app.confirm {
          if let ActiveBlock::Dialog(d) = route.active_block {
            match d {
              DialogContext::Playlist => handle_playlist_dialog(app),
            }
          }
        }
      }
    }
    Key::Char('q') => {
      app.pop_navigation_stack();
    }
    Key::Right => app.confirm = !app.confirm,
    Key::Left => app.confirm = !app.confirm,
    _ => {}
  }
}

fn handle_playlist_dialog(app: &mut App) {
  app.user_unfollow_playlist()
}
