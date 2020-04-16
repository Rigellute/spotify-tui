use super::super::app::App;
use crate::event::Key;

pub fn handler(key: Key, app: &mut App) {
  match key {
    Key::Enter => {
      app.pop_navigation_stack();

      if app.confirm {
        app.user_unfollow_playlist()
      }
    }
    Key::Esc => {
      app.pop_navigation_stack();
    }
    Key::Char('q') => {
      app.pop_navigation_stack();
    }
    Key::Right => app.confirm = !app.confirm,
    Key::Left => app.confirm = !app.confirm,
    _ => {}
  }
}
