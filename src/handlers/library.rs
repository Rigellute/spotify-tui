use super::super::app::{ActiveBlock, App, LIBRARY_OPTIONS};
use super::common_key_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Empty), None);
        }
        Key::Char('d') => {
            app.handle_get_devices();
        }
        Key::Char(' ') => {
            app.toggle_playback();
        }
        k if common_key_events::down_event(k) => {
            let next_index = common_key_events::on_down_press_handler(
                &LIBRARY_OPTIONS,
                Some(app.library.selected_index),
            );
            app.library.selected_index = next_index;
        }
        Key::Char('?') => {
            app.set_current_route_state(Some(ActiveBlock::HelpMenu), None);
        }
        k if common_key_events::up_event(k) => {
            let next_index = common_key_events::on_up_press_handler(
                &LIBRARY_OPTIONS,
                Some(app.library.selected_index),
            );
            app.library.selected_index = next_index;
        }
        Key::Char('/') => {
            app.set_current_route_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
        }
        // This should probably be an array of structs with enums rather than just using indexes
        // like this
        Key::Char('\n') => match app.library.selected_index {
            // Made For You,
            0 => {}
            // Recently Played,
            1 => {}
            // Liked Songs,
            2 => {
                app.get_current_user_saved_tracks(None);
            }
            // Albums,
            3 => {}
            //  Artists,
            4 => {}
            // Podcasts,
            5 => {}
            _ => {}
        },
        _ => (),
    };
}
