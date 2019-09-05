use super::super::app::{ActiveBlock, App};
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Char('\n') => {
            app.active_block = ActiveBlock::MyPlaylists;
        }
        Key::Char('d') => {
            app.handle_get_devices();
        }
        _ => (),
    };
}
