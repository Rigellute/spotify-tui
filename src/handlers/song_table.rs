use super::super::app::{ActiveBlock, App, SongTableContext, TrackTable};
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
        // Press space to toggle playback
        Key::Char(' ') => {
            app.toggle_playback();
        }
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
        Key::Char('?') => {
            app.set_current_route_state(Some(ActiveBlock::HelpMenu), None);
        }
        k if common_key_events::up_event(k) => {
            let next_index = common_key_events::on_up_press_handler(
                &app.track_table.tracks,
                Some(app.track_table.selected_index),
            );
            app.track_table.selected_index = next_index;
        }
        Key::Char('/') => {
            app.set_current_route_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
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
                            // TODO get context for saved tracks
                            if let Some(item) =
                                saved_tracks.items.get(app.track_table.selected_index)
                            {
                                let track_uri = item.track.uri.to_owned();
                                app.start_playback(None, Some(vec![track_uri]), Some(0));
                            };
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
