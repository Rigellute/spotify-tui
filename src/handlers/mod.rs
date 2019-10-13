mod album_list;
mod album_tracks;
mod artist_albums;
mod artists;
mod common_key_events;
mod empty;
mod error_screen;
mod help_menu;
mod home;
mod input;
mod library;
mod made_for_you;
mod playlist;
mod podcasts;
mod recently_played;
mod search_results;
mod select_device;
mod track_table;

use super::app::{ActiveBlock, App, SearchResultBlock};
use termion::event::Key;

pub use input::handler as input_handler;

pub fn handle_app(key: Key, app: &mut App) {
    // First handle any global events and then move to block events
    match key {
        Key::Esc => match app.get_current_route().active_block {
            ActiveBlock::SearchResultBlock => {
                app.search_results.selected_block = SearchResultBlock::Empty;
            }
            ActiveBlock::Error => {
                app.pop_navigation_stack();
            }
            _ => {
                app.set_current_route_state(Some(ActiveBlock::Empty), None);
            }
        },
        // Jump to currently playing album
        Key::Char('a') => {
            if let Some(current_playback_context) = &app.current_playback_context {
                if let Some(full_track) = &current_playback_context.item.clone() {
                    app.get_album_tracks(full_track.album.clone());
                }
            };
        }
        // Jump to currently playing artist's album list.
        // NOTE: this only finds the first artist of the song and jumps to their albums
        Key::Char('A') => {
            if let Some(current_playback_context) = &app.current_playback_context {
                if let Some(playing_item) = &current_playback_context.item.clone() {
                    if let Some(artist) = playing_item.artists.first() {
                        if let Some(artist_id) = &artist.id {
                            app.get_artist_albums(artist_id, &artist.name);
                        }
                    }
                }
            };
        }
        Key::Char('d') => {
            app.handle_get_devices();
        }
        Key::Char('-') => {
            app.decrease_volume();
        }
        Key::Char('+') => {
            app.increase_volume();
        }
        // Press space to toggle playback
        Key::Char(' ') => {
            app.toggle_playback();
        }
        Key::Char('<') => {
            app.seek_backwards();
        }
        Key::Char('>') => {
            app.seek_forwards();
        }
        Key::Char('n') => {
            app.next_track();
        }
        Key::Char('p') => {
            app.previous_track();
        }
        Key::Char('?') => {
            app.set_current_route_state(Some(ActiveBlock::HelpMenu), None);
        }

        Key::Ctrl('s') => {
            app.shuffle();
        }
        Key::Ctrl('r') => {
            app.repeat();
        }
        Key::Char('/') => {
            app.set_current_route_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
        }
        _ => handle_block_events(key, app),
    }
}

// Handle events for the current active block
fn handle_block_events(key: Key, app: &mut App) {
    let current_route = app.get_current_route();
    match current_route.active_block {
        ActiveBlock::Artist => {
            artist_albums::handler(key, app);
        }
        ActiveBlock::Input => {
            input::handler(key, app);
        }
        ActiveBlock::MyPlaylists => {
            playlist::handler(key, app);
        }
        ActiveBlock::TrackTable => {
            track_table::handler(key, app);
        }
        ActiveBlock::HelpMenu => {
            help_menu::handler(key, app);
        }
        ActiveBlock::Error => {
            error_screen::handler(key, app);
        }
        ActiveBlock::SelectDevice => {
            select_device::handler(key, app);
        }
        ActiveBlock::SearchResultBlock => {
            search_results::handler(key, app);
        }
        ActiveBlock::Home => {
            home::handler(key, app);
        }
        ActiveBlock::AlbumList => {
            album_list::handler(key, app);
        }
        ActiveBlock::AlbumTracks => {
            album_tracks::handler(key, app);
        }
        ActiveBlock::Library => {
            library::handler(key, app);
        }
        ActiveBlock::Empty => {
            empty::handler(key, app);
        }
        ActiveBlock::RecentlyPlayed => {
            recently_played::handler(key, app);
        }
        ActiveBlock::Artists => {
            artists::handler(key, app);
        }
        ActiveBlock::MadeForYou => {
            made_for_you::handler(key, app);
        }
        ActiveBlock::Podcasts => {
            podcasts::handler(key, app);
        }
    }
}
