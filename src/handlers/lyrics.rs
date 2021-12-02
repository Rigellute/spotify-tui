//new: new module for the lyrics
use super::{super::app::App, common_key_events};
use crate::event::Key;

const LARGE_SCROLL: u16 = 10;
const SMALL_SCROLL: u16 = 1;

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
    k if common_key_events::down_event(k) => {
      app.home_scroll += SMALL_SCROLL;
    }
    k if common_key_events::up_event(k) => {
      if app.home_scroll > 0 {
        app.home_scroll -= SMALL_SCROLL;
      }
    }
    k if k == app.user_config.keys.next_page => {
      app.home_scroll += LARGE_SCROLL;
    }
    k if k == app.user_config.keys.previous_page => {
      if app.home_scroll > LARGE_SCROLL {
        app.home_scroll -= LARGE_SCROLL;
      } else {
        app.home_scroll = 0;
      }
    }
    _ => {}
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn on_small_down_press() {
    let mut app = App::default();

    handler(Key::Down, &mut app);
    assert_eq!(app.home_scroll, SMALL_SCROLL);

    handler(Key::Down, &mut app);
    assert_eq!(app.home_scroll, SMALL_SCROLL * 2);
  }

  #[test]
  fn on_small_up_press() {
    let mut app = App::default();

    handler(Key::Up, &mut app);
    assert_eq!(app.home_scroll, 0);

    app.home_scroll = 1;

    handler(Key::Up, &mut app);
    assert_eq!(app.home_scroll, 0);

    // Check that smashing the up button doesn't go to negative scroll (which would cause a crash)
    handler(Key::Up, &mut app);
    handler(Key::Up, &mut app);
    handler(Key::Up, &mut app);
    assert_eq!(app.home_scroll, 0);
  }

  #[test]
  fn on_large_down_press() {
    let mut app = App::default();

    handler(Key::Ctrl('d'), &mut app);
    assert_eq!(app.home_scroll, LARGE_SCROLL);

    handler(Key::Ctrl('d'), &mut app);
    assert_eq!(app.home_scroll, LARGE_SCROLL * 2);
  }

  #[test]
  fn on_large_up_press() {
    let mut app = App::default();

    let scroll = 37;
    app.home_scroll = scroll;

    handler(Key::Ctrl('u'), &mut app);
    assert_eq!(app.home_scroll, scroll - LARGE_SCROLL);

    handler(Key::Ctrl('u'), &mut app);
    assert_eq!(app.home_scroll, scroll - LARGE_SCROLL * 2);

    // Check that smashing the up button doesn't go to negative scroll (which would cause a crash)
    handler(Key::Ctrl('u'), &mut app);
    handler(Key::Ctrl('u'), &mut app);
    handler(Key::Ctrl('u'), &mut app);
    assert_eq!(app.home_scroll, 0);
  }
}
