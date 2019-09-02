use super::super::app::{ActiveBlock, App};
use super::common_key_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::left_event(k) => {
            app.active_block = ActiveBlock::MyPlaylists;
        }
        Key::Char('d') => {
            app.handle_get_devices();
        }
        Key::Char('?') => {
            app.active_block = ActiveBlock::HelpMenu;
        }
        Key::Char('/') => {
            app.active_block = ActiveBlock::Input;
        }
        // Press space to toggle playback
        Key::Char(' ') => {
            app.toggle_playback();
        }
        _ => {}
    }
}
