use super::super::app::{ActiveBlock, App, SongTableContext, TrackTable};
use super::common_key_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::left_event(k) => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));
        }
        k if common_key_events::down_event(k) => {
            let next_index = common_key_events::on_down_press_handler(
                &app.track_table.tracks,
                Some(app.track_table.selected_index),
            );
            app.track_table.selected_index = next_index;
        }
        k if common_key_events::up_event(k) => {
            let next_index = common_key_events::on_up_press_handler(
                &app.track_table.tracks,
                Some(app.track_table.selected_index),
            );
            app.track_table.selected_index = next_index;
        }
        Key::Char('\n') => {
            let TrackTable {
                context,
                selected_index,
                tracks,
            } = &app.track_table;
            match &context {
                Some(context) => match context {
                    SongTableContext::MyPlaylists => {
                        if let Some(_track) = tracks.get(*selected_index) {
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

                            app.start_playback(
                                context_uri,
                                None,
                                Some(app.track_table.selected_index),
                            );
                        };
                    }
                    SongTableContext::SavedTracks => {
                        if let Some(saved_tracks) = &app.library.saved_tracks.get_results(None) {
                            let track_uris: Vec<String> = saved_tracks
                                .items
                                .iter()
                                .map(|item| item.track.uri.to_owned())
                                .collect();

                            app.start_playback(
                                None,
                                Some(track_uris),
                                Some(app.track_table.selected_index),
                            );
                        };
                    }
                    SongTableContext::AlbumSearch => {}
                    SongTableContext::SongSearch => {}
                    SongTableContext::ArtistSearch => {}
                    SongTableContext::PlaylistSearch => {
                        let TrackTable {
                            selected_index,
                            tracks,
                            ..
                        } = &app.track_table;
                        if let Some(_track) = tracks.get(*selected_index) {
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

                            app.start_playback(
                                context_uri,
                                None,
                                Some(app.track_table.selected_index),
                            );
                        };
                    }
                },
                None => {}
            };
        }
        // Scroll down
        Key::Ctrl('d') => {
            match &app.track_table.context {
                Some(context) => match context {
                    SongTableContext::MyPlaylists => {}
                    SongTableContext::SavedTracks => {
                        app.get_current_user_saved_tracks_next();
                    }
                    SongTableContext::AlbumSearch => {}
                    SongTableContext::SongSearch => {}
                    SongTableContext::ArtistSearch => {}
                    SongTableContext::PlaylistSearch => {}
                },
                None => {}
            };
        }
        // Scroll up
        Key::Ctrl('u') => {
            match &app.track_table.context {
                Some(context) => match context {
                    SongTableContext::MyPlaylists => {}
                    SongTableContext::SavedTracks => {
                        app.get_current_user_saved_tracks_previous();
                    }
                    SongTableContext::AlbumSearch => {}
                    SongTableContext::SongSearch => {}
                    SongTableContext::ArtistSearch => {}
                    SongTableContext::PlaylistSearch => {}
                },
                None => {}
            };
        }
        _ => {}
    };
}
