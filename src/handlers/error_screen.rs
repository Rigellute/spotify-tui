use super::super::app::{ActiveBlock, App};
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Empty), None);
        }
        Key::Char('d') => {
            app.handle_get_devices();
        }
        _ => (),
    };
}
