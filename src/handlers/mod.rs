mod common_key_events;
mod input;
mod playlist;
mod search_results;
mod select_device;
mod song_table;

use super::app::{ActiveBlock, App};
use termion::event::Key;

// TODO: move to another file
fn help_menu_handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.active_block = ActiveBlock::MyPlaylists;
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

// TODO: move to another file
fn api_error_menu_handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.active_block = ActiveBlock::MyPlaylists;
        }
        Key::Char('d') => {
            app.handle_get_devices();
        }
        _ => (),
    };
}

// TODO: move to another file
fn home_handler(key: Key, app: &mut App) {
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

pub fn handle_app(app: &mut App, key: Key) {
    // Match events for different app states
    match app.active_block {
        ActiveBlock::Input => {
            input::handler(key, app);
        }
        ActiveBlock::MyPlaylists => {
            playlist::handler(key, app);
        }
        ActiveBlock::SongTable => {
            song_table::handler(key, app);
        }
        ActiveBlock::HelpMenu => {
            help_menu_handler(key, app);
        }
        ActiveBlock::Error => {
            api_error_menu_handler(key, app);
        }
        ActiveBlock::SelectDevice => {
            select_device::handler(key, app);
        }
        ActiveBlock::SearchResultBlock => {
            search_results::handler(key, app);
        }
        ActiveBlock::Home => {
            home_handler(key, app);
        }
    }
}
