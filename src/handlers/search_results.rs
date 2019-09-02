use super::super::app::{ActiveBlock, App, SearchResultBlock, SongTableContext};
use super::common_key_events;
use termion::event::Key;

// TODO: break this down into smaller functions and add tests
pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Char('d') => {
            app.handle_get_devices();
        }
        // Press space to toggle playback
        Key::Char(' ') => {
            app.toggle_playback();
        }
        Key::Char('?') => {
            app.active_block = ActiveBlock::HelpMenu;
        }
        Key::Char('/') => {
            app.active_block = ActiveBlock::Input;
        }
        Key::Esc => {
            app.search_results.selected_block = SearchResultBlock::Empty;
        }
        k if common_key_events::down_event(k) => {
            if app.search_results.selected_block != SearchResultBlock::Empty {
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
            } else {
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
        }
        k if common_key_events::up_event(k) => {
            if app.search_results.selected_block != SearchResultBlock::Empty {
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
            } else {
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
        }
        k if common_key_events::left_event(k) => {
            app.search_results.selected_block = SearchResultBlock::Empty;
            match app.search_results.hovered_block {
                SearchResultBlock::AlbumSearch => {
                    app.active_block = ActiveBlock::MyPlaylists;
                }
                SearchResultBlock::SongSearch => {
                    app.active_block = ActiveBlock::MyPlaylists;
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
                match &app.search_results.selected_block {
                    SearchResultBlock::AlbumSearch => {
                        if let (Some(index), Some(albums_result)) = (
                            &app.search_results.selected_album_index,
                            &app.search_results.albums,
                        ) {
                            if let Some(album) = albums_result.albums.items.get(index.to_owned()) {
                                if let Some(album_id) = &album.id {
                                    app.song_table_context = Some(SongTableContext::AlbumSearch);
                                    if let Some(spotify) = &app.spotify {
                                        match spotify.album_track(
                                            album_id,
                                            app.large_search_limit,
                                            0,
                                        ) {
                                            Ok(album_tracks) => {
                                                // TODO: that query returns SimplifiedTrack rather
                                                // than FullTrack
                                                //
                                                // app.songs_for_table = album_tracks.items.clone();

                                                // app.playlist_tracks = playlist_tracks.items;
                                                // app.active_block = ActiveBlock::SongTable;
                                                // app.navigation_stack.push(Routes::SongTable);
                                            }
                                            Err(e) => {
                                                app.active_block = ActiveBlock::Error;
                                                app.api_error = e.to_string();
                                            }
                                        }
                                    }
                                }
                            };
                        }
                    }
                    SearchResultBlock::SongSearch => {
                        if let Some(index) = &app.search_results.selected_tracks_index {
                            if let Some(result) = app.search_results.tracks.clone() {
                                if let Some(track) = result.tracks.items.get(index.to_owned()) {
                                    app.start_playback(
                                        None,
                                        Some(vec![track.uri.to_owned()]),
                                        Some(0),
                                    );
                                };
                            };
                        };
                    }
                    SearchResultBlock::ArtistSearch => {
                        // TODO: Go to artist view (not yet implemented)
                    }
                    SearchResultBlock::PlaylistSearch => {
                        if let (Some(index), Some(playlists_result)) = (
                            app.search_results.selected_playlists_index,
                            &app.search_results.playlists,
                        ) {
                            if let Some(playlist) = playlists_result.playlists.items.get(index) {
                                // Go to playlist tracks table
                                app.song_table_context = Some(SongTableContext::PlaylistSearch);
                                let playlist_id = playlist.id.to_owned();
                                app.get_playlist_tracks(playlist_id);
                            };
                        }
                    }
                    SearchResultBlock::Empty => {}
                };
            } else {
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
        }
        // Add `s` to "see more" on each option
        _ => {}
    }
}
