use super::super::app::{ActiveBlock, App};
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.active_block = ActiveBlock::Empty;
        }
        Key::Char('d') => {
            app.handle_get_devices();
        }
        _ => (),
    };
}
