use super::{
    super::app::{App, RecommendationsContext, TrackTable, TrackTableContext},
    common_key_events,
};
use crate::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
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
        k if common_key_events::high_event(k) => {
            let next_index = common_key_events::on_high_press_handler();
            app.track_table.selected_index = next_index;
        }
        k if common_key_events::middle_event(k) => {
            let next_index = common_key_events::on_middle_press_handler(&app.track_table.tracks);
            app.track_table.selected_index = next_index;
        }
        k if common_key_events::low_event(k) => {
            let next_index = common_key_events::on_low_press_handler(&app.track_table.tracks);
            app.track_table.selected_index = next_index;
        }
        Key::Enter => {
            on_enter(app);
        }
        // Scroll down
        Key::Ctrl('d') => {
            match &app.track_table.context {
                Some(context) => match context {
                    TrackTableContext::MyPlaylists => {
                        if let (Some(playlists), Some(selected_playlist_index)) =
                            (&app.playlists, &app.selected_playlist_index)
                        {
                            if let Some(selected_playlist) =
                                playlists.items.get(selected_playlist_index.to_owned())
                            {
                                if let Some(playlist_tracks) = &app.playlist_tracks {
                                    if app.playlist_offset + app.large_search_limit
                                        < playlist_tracks.total
                                    {
                                        app.playlist_offset += app.large_search_limit;
                                        let playlist_id = selected_playlist.id.to_owned();
                                        app.get_playlist_tracks(playlist_id);
                                    }
                                }
                            }
                        };
                    }
                    TrackTableContext::RecommendedTracks => {}
                    TrackTableContext::SavedTracks => {
                        app.get_current_user_saved_tracks_next();
                    }
                    TrackTableContext::AlbumSearch => {}
                    TrackTableContext::PlaylistSearch => {}
                },
                None => {}
            };
        }
        // Scroll up
        Key::Ctrl('u') => {
            match &app.track_table.context {
                Some(context) => match context {
                    TrackTableContext::MyPlaylists => {
                        if let (Some(playlists), Some(selected_playlist_index)) =
                            (&app.playlists, &app.selected_playlist_index)
                        {
                            if app.playlist_offset >= app.large_search_limit {
                                app.playlist_offset -= app.large_search_limit;
                            };
                            if let Some(selected_playlist) =
                                playlists.items.get(selected_playlist_index.to_owned())
                            {
                                let playlist_id = selected_playlist.id.to_owned();
                                app.get_playlist_tracks(playlist_id);
                            }
                        };
                    }
                    TrackTableContext::RecommendedTracks => {}
                    TrackTableContext::SavedTracks => {
                        app.get_current_user_saved_tracks_previous();
                    }
                    TrackTableContext::AlbumSearch => {}
                    TrackTableContext::PlaylistSearch => {}
                },
                None => {}
            };
        }
        Key::Ctrl('e') => jump_to_end(app),
        Key::Ctrl('a') => jump_to_start(app),
        //recommended song radio
        Key::Char('r') => {
            handle_recommended_tracks(app);
        }
        _ => {}
    }
}

fn handle_recommended_tracks(app: &mut App) {
    let (selected_index, tracks) = (&app.track_table.selected_index, &app.track_table.tracks);
    if let Some(track) = tracks.get(*selected_index) {
        let first_track = track.clone();
        let track_id_list: Option<Vec<String>> = match &track.id {
            Some(id) => Some(vec![id.to_string()]),
            None => None,
        };
        app.recommendations_context = Some(RecommendationsContext::Song);
        app.recommendations_seed = first_track.name.clone();
        app.get_recommendations_for_seed(None, track_id_list, Some(&first_track));
    };
}

fn jump_to_end(app: &mut App) {
    match &app.track_table.context {
        Some(context) => match context {
            TrackTableContext::MyPlaylists => {
                if let (Some(playlists), Some(selected_playlist_index)) =
                    (&app.playlists, &app.selected_playlist_index)
                {
                    if let Some(selected_playlist) =
                        playlists.items.get(selected_playlist_index.to_owned())
                    {
                        let total_tracks = selected_playlist
                            .tracks
                            .get("total")
                            .and_then(|total| total.as_u64())
                            .expect("playlist.tracks object should have a total field")
                            as u32;

                        if app.large_search_limit < total_tracks {
                            app.playlist_offset =
                                total_tracks - (total_tracks % app.large_search_limit);
                            let playlist_id = selected_playlist.id.to_owned();
                            app.get_playlist_tracks(playlist_id);
                        }
                    }
                }
            }
            TrackTableContext::RecommendedTracks => {}
            TrackTableContext::SavedTracks => {}
            TrackTableContext::AlbumSearch => {}
            TrackTableContext::PlaylistSearch => {}
        },
        None => {}
    }
}

fn on_enter(app: &mut App) {
    let TrackTable {
        context,
        selected_index,
        tracks,
    } = &app.track_table;
    match &context {
        Some(context) => match context {
            TrackTableContext::MyPlaylists => {
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
                        Some(app.track_table.selected_index + app.playlist_offset as usize),
                    );
                };
            }
            TrackTableContext::RecommendedTracks => {
                if let Some(_track) = tracks.get(*selected_index) {
                    app.start_recommendations_playback(Some(app.track_table.selected_index));
                };
            }
            TrackTableContext::SavedTracks => {
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
            TrackTableContext::AlbumSearch => {}
            TrackTableContext::PlaylistSearch => {
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

                    app.start_playback(context_uri, None, Some(app.track_table.selected_index));
                };
            }
        },
        None => {}
    };
}
fn jump_to_start(app: &mut App) {
    match &app.track_table.context {
        Some(context) => match context {
            TrackTableContext::MyPlaylists => {
                if let (Some(playlists), Some(selected_playlist_index)) =
                    (&app.playlists, &app.selected_playlist_index)
                {
                    if let Some(selected_playlist) =
                        playlists.items.get(selected_playlist_index.to_owned())
                    {
                        app.playlist_offset = 0;
                        let playlist_id = selected_playlist.id.to_owned();
                        app.get_playlist_tracks(playlist_id);
                    }
                }
            }
            TrackTableContext::RecommendedTracks => {}
            TrackTableContext::SavedTracks => {}
            TrackTableContext::AlbumSearch => {}
            TrackTableContext::PlaylistSearch => {}
        },
        None => {}
    }
}
