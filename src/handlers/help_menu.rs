use super::super::app::{ActiveBlock, App};
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.active_block = ActiveBlock::Empty;
        }
        // Press space to toggle playback
        Key::Char(' ') => {
            app.toggle_playback();
        }
        Key::Char('d') => {
            app.handle_get_devices();
        }
        _ => {}
    };
}
