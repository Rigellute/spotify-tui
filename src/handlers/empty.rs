use super::super::app::{ActiveBlock, App, Routes};
use super::common_key_events;
use termion::event::Key;

// When no block is actively selected, just handle regular events
pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Char('d') => {
            app.handle_get_devices();
        }
        Key::Char(' ') => {
            app.toggle_playback();
        }
        Key::Char('?') => {
            app.active_block = ActiveBlock::HelpMenu;
        }
        Key::Char('/') => {
            app.active_block = ActiveBlock::Input;
            app.hovered_block = ActiveBlock::Input;
        }
        Key::Char('\n') => {
            app.active_block = app.hovered_block;
        }
        k if common_key_events::down_event(k) => match app.hovered_block {
            ActiveBlock::Library => {
                app.hovered_block = ActiveBlock::MyPlaylists;
            }
            ActiveBlock::MyPlaylists => {
                // Go to player
            }
            ActiveBlock::Album | ActiveBlock::Home | ActiveBlock::SongTable => {
                // Go to player
            }
            _ => {}
        },
        k if common_key_events::up_event(k) => match app.hovered_block {
            ActiveBlock::MyPlaylists => {
                app.hovered_block = ActiveBlock::Library;
            }
            _ => {}
        },
        k if common_key_events::left_event(k) => match app.hovered_block {
            ActiveBlock::Album | ActiveBlock::Home | ActiveBlock::SongTable => {
                app.hovered_block = ActiveBlock::Library;
            }
            _ => {}
        },
        k if common_key_events::right_event(k) => match app.hovered_block {
            ActiveBlock::MyPlaylists | ActiveBlock::Library => {
                match app.get_current_route() {
                    Some(current_route) => {
                        match current_route {
                            Routes::Album => {
                                app.active_block = ActiveBlock::Album;
                                app.hovered_block = ActiveBlock::Album;
                            }
                            Routes::SongTable => {
                                app.active_block = ActiveBlock::SongTable;
                                app.hovered_block = ActiveBlock::SongTable;
                            }
                            Routes::Search => {
                                app.active_block = ActiveBlock::SearchResultBlock;
                                app.hovered_block = ActiveBlock::SearchResultBlock;
                            }
                            Routes::Artist(_) => {
                                // TODO
                            }
                        }
                    }
                    None => {
                        app.hovered_block = ActiveBlock::Home;
                    }
                }
            }
            _ => {}
        },
        _ => (),
    };
}
