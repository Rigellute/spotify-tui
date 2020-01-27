mod album_list;
mod album_tracks;
mod artist;
mod artists;
mod common_key_events;
mod empty;
mod error_screen;
mod help_menu;
mod home;
mod input;
mod library;
mod made_for_you;
mod playbar;
mod playlist;
mod podcasts;
mod recently_played;
mod search_results;
mod select_device;
mod track_table;

use super::app::{ActiveBlock, App, ArtistBlock, RouteId, SearchResultBlock};
use crate::event::Key;

pub use input::handler as input_handler;

pub fn handle_app(key: Key, app: &mut App) {
    // First handle any global event and then move to block event
    match key {
        Key::Esc => match app.get_current_route().active_block {
            ActiveBlock::SearchResultBlock => {
                app.search_results.selected_block = SearchResultBlock::Empty;
            }
            ActiveBlock::ArtistBlock => {
                if let Some(artist) = &mut app.artist {
                    artist.artist_selected_block = ArtistBlock::Empty;
                }
            }
            ActiveBlock::Error => {
                app.pop_navigation_stack();
            }
            _ => {
                app.set_current_route_state(Some(ActiveBlock::Empty), None);
            }
        },
        // Jump to currently playing album
        _ if key == app.user_config.keys.jump_to_album => {
            if let Some(current_playback_context) = &app.current_playback_context {
                if let Some(full_track) = &current_playback_context.item.clone() {
                    app.get_album_tracks(full_track.album.clone());
                }
            };
        }
        // Jump to currently playing artist's album list.
        // NOTE: this only finds the first artist of the song and jumps to their albums
        _ if key == app.user_config.keys.jump_to_artist_album => {
            if let Some(current_playback_context) = &app.current_playback_context {
                if let Some(playing_item) = &current_playback_context.item.clone() {
                    if let Some(artist) = playing_item.artists.first() {
                        if let Some(artist_id) = &artist.id {
                            app.get_artist(artist_id, &artist.name);
                            app.push_navigation_stack(RouteId::Artist, ActiveBlock::ArtistBlock);
                        }
                    }
                }
            };
        }
        _ if key == app.user_config.keys.manage_devices => {
            app.handle_get_devices();
        }
        _ if key == app.user_config.keys.decrease_volume => {
            app.decrease_volume();
        }
        _ if key == app.user_config.keys.increase_volume => {
            app.increase_volume();
        }
        // Press space to toggle playback
        _ if key == app.user_config.keys.toggle_playback => {
            app.toggle_playback();
        }
        _ if key == app.user_config.keys.seek_backwards => {
            app.seek_backwards();
        }
        _ if key == app.user_config.keys.seek_forwards => {
            app.seek_forwards();
        }
        _ if key == app.user_config.keys.next_track => {
            app.next_track();
        }
        _ if key == app.user_config.keys.previous_track => {
            app.previous_track();
        }
        _ if key == app.user_config.keys.help => {
            app.set_current_route_state(Some(ActiveBlock::HelpMenu), None);
        }

        _ if key == app.user_config.keys.shuffle => {
            app.shuffle();
        }
        _ if key == app.user_config.keys.repeat => {
            app.repeat();
        }
        _ if key == app.user_config.keys.search => {
            app.set_current_route_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
        }
        _ if key == app.user_config.keys.copy_song_url => {
            app.copy_song_url();
        }
        _ if key == app.user_config.keys.copy_album_url => {
            app.copy_album_url();
        }
        _ => handle_block_events(key, app),
    }
}

// Handle event for the current active block
fn handle_block_events(key: Key, app: &mut App) {
    let current_route = app.get_current_route();
    match current_route.active_block {
        ActiveBlock::ArtistBlock => {
            artist::handler(key, app);
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
        ActiveBlock::PlayBar => {
            playbar::handler(key, app);
        }
    }
}
