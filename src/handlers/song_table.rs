use super::super::app::{ActiveBlock, App, SongTableContext};
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
            let next_index = common_key_events::on_down_press_handler(
                &app.songs_for_table,
                Some(app.select_song_index),
            );
            app.select_song_index = next_index;
        }
        Key::Char('?') => {
            app.active_block = ActiveBlock::HelpMenu;
        }
        k if common_key_events::up_event(k) => {
            let next_index = common_key_events::on_up_press_handler(
                &app.songs_for_table,
                Some(app.select_song_index),
            );
            app.select_song_index = next_index;
        }
        Key::Char('/') => {
            app.active_block = ActiveBlock::Input;
        }
        Key::Char('\n') => {
            match &app.song_table_context {
                Some(context) => match context {
                    SongTableContext::MyPlaylists => {
                        if let Some(_track) = app.songs_for_table.get(app.select_song_index) {
                            let context_uri = match (&app.selected_playlist_index, &app.playlists) {
                                (Some(selected_playlist_index), Some(playlists)) => {
                                    if let Some(selected_playlist) =
                                        playlists.items.get(selected_playlist_index.to_owned())
                                    {
                                        Some(selected_playlist.uri.to_owned())
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            };

                            app.start_playback(context_uri, None, Some(app.select_song_index));
                        };
                    }
                    SongTableContext::SavedTracks => {
                        if let Some(saved_tracks) = &app.library.saved_tracks.clone() {
                            // TODO get context for saved tracks
                            if let Some(item) = saved_tracks.items.get(app.select_song_index) {
                                app.start_playback(
                                    None,
                                    Some(vec![item.track.uri.to_owned()]),
                                    Some(0),
                                );
                            };
                        };
                    }
                    SongTableContext::AlbumSearch => {}
                    SongTableContext::SongSearch => {}
                    SongTableContext::ArtistSearch => {}
                    SongTableContext::PlaylistSearch => {
                        if let Some(_track) = app.songs_for_table.get(app.select_song_index) {
                            let context_uri = match (
                                &app.search_results.selected_playlists_index,
                                &app.search_results.playlists,
                            ) {
                                (Some(selected_playlist_index), Some(playlist_result)) => {
                                    if let Some(selected_playlist) = playlist_result
                                        .playlists
                                        .items
                                        .get(selected_playlist_index.to_owned())
                                    {
                                        Some(selected_playlist.uri.to_owned())
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            };

                            app.start_playback(context_uri, None, Some(app.select_song_index));
                        };
                    }
                },
                None => {}
            };
        }
        _ => {}
    };
}
