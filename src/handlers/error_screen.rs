use crate::app::App;
use termion::event::Key;

pub fn handler(key: Key, _app: &mut App) {
    match key {
        _ => {}
    };
}
