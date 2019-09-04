use super::super::app::{ActiveBlock, App};
use super::common_key_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Char('d') => {
            app.handle_get_devices();
        }
        // Press space to toggle playback
        Key::Char(' ') => {
            app.toggle_playback();
        }
        k if common_key_events::left_event(k) => {
            app.active_block = ActiveBlock::MyPlaylists;
        }
        k if common_key_events::down_event(k) => {
            if let Some(selected_album) = &mut app.selected_album {
                let next_index = common_key_events::on_down_press_handler(
                    &selected_album.tracks.items,
                    selected_album.selected_index,
                );
                selected_album.selected_index = Some(next_index);
            }
        }
        Key::Char('?') => {
            app.active_block = ActiveBlock::HelpMenu;
        }
        k if common_key_events::up_event(k) => {
            if let Some(selected_album) = &mut app.selected_album {
                let next_index = common_key_events::on_up_press_handler(
                    &selected_album.tracks.items,
                    selected_album.selected_index,
                );
                selected_album.selected_index = Some(next_index);
            }
        }
        Key::Char('/') => {
            app.active_block = ActiveBlock::Input;
        }
        Key::Char('\n') => {
            if let Some(selected_album) = &app.selected_album.clone() {
                app.start_playback(
                    selected_album.album.uri.clone(),
                    None,
                    selected_album.selected_index,
                );
            };
        }
        _ => {}
    };
}
