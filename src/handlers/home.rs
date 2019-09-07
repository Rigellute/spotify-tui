use super::super::app::{ActiveBlock, App};
use super::common_key_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Empty), None);
        }
        k if common_key_events::left_event(k) => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));
        }
        Key::Char('d') => {
            app.handle_get_devices();
        }
        Key::Char('?') => {
            app.set_current_route_state(Some(ActiveBlock::HelpMenu), None);
        }
        Key::Char('/') => {
            app.set_current_route_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
        }
        // Press space to toggle playback
        Key::Char(' ') => {
            app.toggle_playback();
        }
        _ => {}
    }
}
