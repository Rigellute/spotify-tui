use super::common_key_events;
use crate::{app::App, event::Key};

#[derive(PartialEq)]
enum Direction {
  Up,
  Down,
}

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::down_event(k) => {
      move_page(Direction::Down, app);
    }
    k if common_key_events::up_event(k) => {
      move_page(Direction::Up, app);
    }
    Key::Ctrl('d') => {
      move_page(Direction::Down, app);
    }
    Key::Ctrl('u') => {
      move_page(Direction::Up, app);
    }
    _ => {}
  };
}

fn move_page(direction: Direction, app: &mut App) {
  if direction == Direction::Up {
    if app.help_menu_page > 0 {
      app.help_menu_page -= 1;
    }
  } else if direction == Direction::Down {
    app.help_menu_page += 1;
  }
  app.calculate_help_menu_offset();
}
