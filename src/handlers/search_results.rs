use super::{
    super::app::{
        ActiveBlock, App, RecommendationsContext, RouteId, SearchResultBlock, TrackTableContext,
    },
    common_key_events,
};
use crate::event::Key;

async fn handle_down_press_on_selected_block(app: &mut App) {
    // Start selecting within the selected block
    match app.search_results.selected_block {
        SearchResultBlock::AlbumSearch => {
            if let Some(result) = &app.search_results.albums {
                let next_index = common_key_events::on_down_press_handler(
                    &result.albums.items,
                    app.search_results.selected_album_index,
                );
                app.search_results.selected_album_index = Some(next_index);
            }
        }
        SearchResultBlock::SongSearch => {
            if let Some(result) = &app.search_results.tracks {
                let next_index = common_key_events::on_down_press_handler(
                    &result.tracks.items,
                    app.search_results.selected_tracks_index,
                );
                app.search_results.selected_tracks_index = Some(next_index);
            }
        }
        SearchResultBlock::ArtistSearch => {
            if let Some(result) = &app.search_results.artists {
                let next_index = common_key_events::on_down_press_handler(
                    &result.artists.items,
                    app.search_results.selected_artists_index,
                );
                app.search_results.selected_artists_index = Some(next_index);
            }
        }
        SearchResultBlock::PlaylistSearch => {
            if let Some(result) = &app.search_results.playlists {
                let next_index = common_key_events::on_down_press_handler(
                    &result.playlists.items,
                    app.search_results.selected_playlists_index,
                );
                app.search_results.selected_playlists_index = Some(next_index);
            }
        }
        SearchResultBlock::Empty => {}
    }
}

fn handle_down_press_on_hovered_block(app: &mut App) {
    match app.search_results.hovered_block {
        SearchResultBlock::AlbumSearch => {
            app.search_results.hovered_block = SearchResultBlock::SongSearch;
        }
        SearchResultBlock::SongSearch => {
            app.search_results.hovered_block = SearchResultBlock::AlbumSearch;
        }
        SearchResultBlock::ArtistSearch => {
            app.search_results.hovered_block = SearchResultBlock::PlaylistSearch;
        }
        SearchResultBlock::PlaylistSearch => {
            app.search_results.hovered_block = SearchResultBlock::ArtistSearch;
        }
        SearchResultBlock::Empty => {}
    }
}

fn handle_up_press_on_selected_block(app: &mut App) {
    // Start selecting within the selected block
    match app.search_results.selected_block {
        SearchResultBlock::AlbumSearch => {
            if let Some(result) = &app.search_results.albums {
                let next_index = common_key_events::on_up_press_handler(
                    &result.albums.items,
                    app.search_results.selected_album_index,
                );
                app.search_results.selected_album_index = Some(next_index);
            }
        }
        SearchResultBlock::SongSearch => {
            if let Some(result) = &app.search_results.tracks {
                let next_index = common_key_events::on_up_press_handler(
                    &result.tracks.items,
                    app.search_results.selected_tracks_index,
                );
                app.search_results.selected_tracks_index = Some(next_index);
            }
        }
        SearchResultBlock::ArtistSearch => {
            if let Some(result) = &app.search_results.artists {
                let next_index = common_key_events::on_up_press_handler(
                    &result.artists.items,
                    app.search_results.selected_artists_index,
                );
                app.search_results.selected_artists_index = Some(next_index);
            }
        }
        SearchResultBlock::PlaylistSearch => {
            if let Some(result) = &app.search_results.playlists {
                let next_index = common_key_events::on_up_press_handler(
                    &result.playlists.items,
                    app.search_results.selected_playlists_index,
                );
                app.search_results.selected_playlists_index = Some(next_index);
            }
        }
        SearchResultBlock::Empty => {}
    }
}

fn handle_up_press_on_hovered_block(app: &mut App) {
    match app.search_results.hovered_block {
        SearchResultBlock::AlbumSearch => {
            app.search_results.hovered_block = SearchResultBlock::SongSearch;
        }
        SearchResultBlock::SongSearch => {
            app.search_results.hovered_block = SearchResultBlock::AlbumSearch;
        }
        SearchResultBlock::ArtistSearch => {
            app.search_results.hovered_block = SearchResultBlock::PlaylistSearch;
        }
        SearchResultBlock::PlaylistSearch => {
            app.search_results.hovered_block = SearchResultBlock::ArtistSearch;
        }
        SearchResultBlock::Empty => {}
    }
}

fn handle_high_press_on_selected_block(app: &mut App) {
    match app.search_results.selected_block {
        SearchResultBlock::AlbumSearch => {
            if let Some(_result) = &app.search_results.albums {
                let next_index = common_key_events::on_high_press_handler();
                app.search_results.selected_album_index = Some(next_index);
            }
        }
        SearchResultBlock::SongSearch => {
            if let Some(_result) = &app.search_results.tracks {
                let next_index = common_key_events::on_high_press_handler();
                app.search_results.selected_tracks_index = Some(next_index);
            }
        }
        SearchResultBlock::ArtistSearch => {
            if let Some(_result) = &app.search_results.artists {
                let next_index = common_key_events::on_high_press_handler();
                app.search_results.selected_artists_index = Some(next_index);
            }
        }
        SearchResultBlock::PlaylistSearch => {
            if let Some(_result) = &app.search_results.playlists {
                let next_index = common_key_events::on_high_press_handler();
                app.search_results.selected_playlists_index = Some(next_index);
            }
        }
        SearchResultBlock::Empty => {}
    }
}

fn handle_middle_press_on_selected_block(app: &mut App) {
    match app.search_results.selected_block {
        SearchResultBlock::AlbumSearch => {
            if let Some(result) = &app.search_results.albums {
                let next_index = common_key_events::on_middle_press_handler(&result.albums.items);
                app.search_results.selected_album_index = Some(next_index);
            }
        }
        SearchResultBlock::SongSearch => {
            if let Some(result) = &app.search_results.tracks {
                let next_index = common_key_events::on_middle_press_handler(&result.tracks.items);
                app.search_results.selected_tracks_index = Some(next_index);
            }
        }
        SearchResultBlock::ArtistSearch => {
            if let Some(result) = &app.search_results.artists {
                let next_index = common_key_events::on_middle_press_handler(&result.artists.items);
                app.search_results.selected_artists_index = Some(next_index);
            }
        }
        SearchResultBlock::PlaylistSearch => {
            if let Some(result) = &app.search_results.playlists {
                let next_index =
                    common_key_events::on_middle_press_handler(&result.playlists.items);
                app.search_results.selected_playlists_index = Some(next_index);
            }
        }
        SearchResultBlock::Empty => {}
    }
}

fn handle_low_press_on_selected_block(app: &mut App) {
    match app.search_results.selected_block {
        SearchResultBlock::AlbumSearch => {
            if let Some(result) = &app.search_results.albums {
                let next_index = common_key_events::on_low_press_handler(&result.albums.items);
                app.search_results.selected_album_index = Some(next_index);
            }
        }
        SearchResultBlock::SongSearch => {
            if let Some(result) = &app.search_results.tracks {
                let next_index = common_key_events::on_low_press_handler(&result.tracks.items);
                app.search_results.selected_tracks_index = Some(next_index);
            }
        }
        SearchResultBlock::ArtistSearch => {
            if let Some(result) = &app.search_results.artists {
                let next_index = common_key_events::on_low_press_handler(&result.artists.items);
                app.search_results.selected_artists_index = Some(next_index);
            }
        }
        SearchResultBlock::PlaylistSearch => {
            if let Some(result) = &app.search_results.playlists {
                let next_index = common_key_events::on_low_press_handler(&result.playlists.items);
                app.search_results.selected_playlists_index = Some(next_index);
            }
        }
        SearchResultBlock::Empty => {}
    }
}

async fn handle_enter_event_on_selected_block(app: &mut App) {
    match &app.search_results.selected_block {
        SearchResultBlock::AlbumSearch => {
            if let (Some(index), Some(albums_result)) = (
                &app.search_results.selected_album_index,
                &app.search_results.albums,
            ) {
                if let Some(album) = albums_result.albums.items.get(index.to_owned()).cloned() {
                    app.track_table.context = Some(TrackTableContext::AlbumSearch);
                    app.get_album_tracks(album).await;
                };
            }
        }
        SearchResultBlock::SongSearch => {
            if let Some(index) = &app.search_results.selected_tracks_index {
                if let Some(result) = app.search_results.tracks.clone() {
                    if let Some(track) = result.tracks.items.get(index.to_owned()) {
                        app.start_playback(None, Some(vec![track.uri.to_owned()]), Some(0))
                            .await;
                    };
                };
            };
        }
        SearchResultBlock::ArtistSearch => {
            if let Some(index) = &app.search_results.selected_artists_index {
                if let Some(result) = app.search_results.artists.clone() {
                    if let Some(artist) = result.artists.items.get(index.to_owned()) {
                        app.get_artist(&artist.id, &artist.name).await;
                        app.push_navigation_stack(RouteId::Artist, ActiveBlock::ArtistBlock);
                    };
                };
            };
        }
        SearchResultBlock::PlaylistSearch => {
            if let (Some(index), Some(playlists_result)) = (
                app.search_results.selected_playlists_index,
                &app.search_results.playlists,
            ) {
                if let Some(playlist) = playlists_result.playlists.items.get(index) {
                    // Go to playlist tracks table
                    app.track_table.context = Some(TrackTableContext::PlaylistSearch);
                    let playlist_id = playlist.id.to_owned();
                    app.get_playlist_tracks(playlist_id).await;
                };
            }
        }
        SearchResultBlock::Empty => {}
    };
}

fn handle_enter_event_on_hovered_block(app: &mut App) {
    match app.search_results.hovered_block {
        SearchResultBlock::AlbumSearch => {
            let next_index = match app.search_results.selected_album_index {
                Some(index) => index,
                None => 0,
            };

            app.search_results.selected_album_index = Some(next_index);
            app.search_results.selected_block = SearchResultBlock::AlbumSearch;
        }
        SearchResultBlock::SongSearch => {
            let next_index = match app.search_results.selected_tracks_index {
                Some(index) => index,
                None => 0,
            };

            app.search_results.selected_tracks_index = Some(next_index);
            app.search_results.selected_block = SearchResultBlock::SongSearch;
        }
        SearchResultBlock::ArtistSearch => {
            let next_index = match app.search_results.selected_artists_index {
                Some(index) => index,
                None => 0,
            };

            app.search_results.selected_artists_index = Some(next_index);
            app.search_results.selected_block = SearchResultBlock::ArtistSearch;
        }
        SearchResultBlock::PlaylistSearch => {
            let next_index = match app.search_results.selected_playlists_index {
                Some(index) => index,
                None => 0,
            };

            app.search_results.selected_playlists_index = Some(next_index);
            app.search_results.selected_block = SearchResultBlock::PlaylistSearch;
        }
        SearchResultBlock::Empty => {}
    };
}

async fn handle_recommended_tracks(app: &mut App) {
    match app.search_results.selected_block {
        SearchResultBlock::AlbumSearch => {}
        SearchResultBlock::SongSearch => {
            if let Some(index) = &app.search_results.selected_tracks_index {
                if let Some(result) = app.search_results.tracks.clone() {
                    if let Some(track) = result.tracks.items.get(index.to_owned()) {
                        let track_id_list: Option<Vec<String>> = match &track.id {
                            Some(id) => Some(vec![id.to_string()]),
                            None => None,
                        };
                        app.recommendations_context = Some(RecommendationsContext::Song);
                        app.recommendations_seed = track.name.clone();
                        app.get_recommendations_for_seed(None, track_id_list, Some(track))
                            .await;
                    };
                };
            };
        }
        SearchResultBlock::ArtistSearch => {
            if let Some(index) = &app.search_results.selected_artists_index {
                if let Some(result) = app.search_results.artists.clone() {
                    if let Some(artist) = result.artists.items.get(index.to_owned()) {
                        let artist_id_list: Option<Vec<String>> = Some(vec![artist.id.clone()]);
                        app.recommendations_context = Some(RecommendationsContext::Artist);
                        app.recommendations_seed = artist.name.clone();
                        app.get_recommendations_for_seed(artist_id_list, None, None)
                            .await;
                    };
                };
            };
        }
        SearchResultBlock::PlaylistSearch => {}
        SearchResultBlock::Empty => {}
    }
}

pub async fn handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.search_results.selected_block = SearchResultBlock::Empty;
        }
        k if common_key_events::down_event(k) => {
            if app.search_results.selected_block != SearchResultBlock::Empty {
                handle_down_press_on_selected_block(app).await;
            } else {
                handle_down_press_on_hovered_block(app);
            }
        }
        k if common_key_events::up_event(k) => {
            if app.search_results.selected_block != SearchResultBlock::Empty {
                handle_up_press_on_selected_block(app);
            } else {
                handle_up_press_on_hovered_block(app);
            }
        }
        k if common_key_events::left_event(k) => {
            app.search_results.selected_block = SearchResultBlock::Empty;
            match app.search_results.hovered_block {
                SearchResultBlock::AlbumSearch => {
                    common_key_events::handle_left_event(app);
                }
                SearchResultBlock::SongSearch => {
                    common_key_events::handle_left_event(app);
                }
                SearchResultBlock::ArtistSearch => {
                    app.search_results.hovered_block = SearchResultBlock::SongSearch;
                }
                SearchResultBlock::PlaylistSearch => {
                    app.search_results.hovered_block = SearchResultBlock::AlbumSearch;
                }
                SearchResultBlock::Empty => {}
            }
        }
        k if common_key_events::right_event(k) => {
            app.search_results.selected_block = SearchResultBlock::Empty;
            match app.search_results.hovered_block {
                SearchResultBlock::AlbumSearch => {
                    app.search_results.hovered_block = SearchResultBlock::PlaylistSearch;
                }
                SearchResultBlock::SongSearch => {
                    app.search_results.hovered_block = SearchResultBlock::ArtistSearch;
                }
                SearchResultBlock::ArtistSearch => {
                    app.search_results.hovered_block = SearchResultBlock::SongSearch;
                }
                SearchResultBlock::PlaylistSearch => {
                    app.search_results.hovered_block = SearchResultBlock::AlbumSearch;
                }
                SearchResultBlock::Empty => {}
            }
        }
        k if common_key_events::high_event(k) => {
            if app.search_results.selected_block != SearchResultBlock::Empty {
                handle_high_press_on_selected_block(app);
            }
        }
        k if common_key_events::middle_event(k) => {
            if app.search_results.selected_block != SearchResultBlock::Empty {
                handle_middle_press_on_selected_block(app);
            }
        }
        k if common_key_events::low_event(k) => {
            if app.search_results.selected_block != SearchResultBlock::Empty {
                handle_low_press_on_selected_block(app)
            }
        }
        // Handle pressing enter when block is selected to start playing track
        Key::Enter => match app.search_results.selected_block {
            SearchResultBlock::Empty => handle_enter_event_on_hovered_block(app),
            SearchResultBlock::PlaylistSearch => {
                app.playlist_offset = 0;
                handle_enter_event_on_selected_block(app).await;
            }
            _ => handle_enter_event_on_selected_block(app).await,
        },
        Key::Char('w') => match app.search_results.selected_block {
            SearchResultBlock::AlbumSearch => app.current_user_saved_album_add().await,
            SearchResultBlock::SongSearch => {}
            SearchResultBlock::ArtistSearch => app.user_follow_artists().await,
            SearchResultBlock::PlaylistSearch => {
                app.user_follow_playlists().await;
                if let Some(spotify) = &app.spotify {
                    let playlists = spotify
                        .current_user_playlists(app.large_search_limit, None)
                        .await;

                    match playlists {
                        Ok(p) => app.playlists = Some(p),
                        Err(e) => app.handle_error(e),
                    }
                }
            }
            SearchResultBlock::Empty => {}
        },
        Key::Char('r') => handle_recommended_tracks(app).await,
        // Add `s` to "see more" on each option
        _ => {}
    }
}
