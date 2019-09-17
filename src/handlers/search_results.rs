use super::super::app::{App, SearchResultBlock, SongTableContext};
use super::common_key_events;
use termion::event::Key;

fn handle_down_press_on_selected_block(app: &mut App) {
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

fn handle_enter_event_on_selected_block(app: &mut App) {
    match &app.search_results.selected_block {
        SearchResultBlock::AlbumSearch => {
            if let (Some(index), Some(albums_result)) = (
                &app.search_results.selected_album_index,
                &app.search_results.albums,
            ) {
                if let Some(album) = albums_result.albums.items.get(index.to_owned()).cloned() {
                    app.track_table.context = Some(SongTableContext::AlbumSearch);
                    app.get_album_tracks(album);
                };
            }
        }
        SearchResultBlock::SongSearch => {
            if let Some(index) = &app.search_results.selected_tracks_index {
                if let Some(result) = app.search_results.tracks.clone() {
                    if let Some(track) = result.tracks.items.get(index.to_owned()) {
                        app.start_playback(None, Some(vec![track.uri.to_owned()]), Some(0));
                    };
                };
            };
        }
        SearchResultBlock::ArtistSearch => {
            if let Some(index) = &app.search_results.selected_artists_index {
                if let Some(result) = app.search_results.artists.clone() {
                    if let Some(artist) = result.artists.items.get(index.to_owned()) {
                        app.get_artist_albums(&artist.id, &artist.name);
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
                    app.track_table.context = Some(SongTableContext::PlaylistSearch);
                    let playlist_id = playlist.id.to_owned();
                    app.get_playlist_tracks(playlist_id);
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

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.search_results.selected_block = SearchResultBlock::Empty;
        }
        k if common_key_events::down_event(k) => {
            if app.search_results.selected_block != SearchResultBlock::Empty {
                handle_down_press_on_selected_block(app);
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
        // Handle pressing enter when block is selected to start playing track
        Key::Char('\n') => {
            if app.search_results.selected_block != SearchResultBlock::Empty {
                handle_enter_event_on_selected_block(app);
            } else {
                handle_enter_event_on_hovered_block(app)
            }
        }
        // Add `s` to "see more" on each option
        _ => {}
    }
}
