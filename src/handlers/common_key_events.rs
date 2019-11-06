use crate::app::{ActiveBlock, AlbumTableContext, App, RouteId, SearchResultBlock};
use termion::event::Key;

pub fn down_event(key: Key) -> bool {
    match key {
        Key::Down | Key::Char('j') | Key::Ctrl('n') => true,
        _ => false,
    }
}

pub fn up_event(key: Key) -> bool {
    match key {
        Key::Up | Key::Char('k') | Key::Ctrl('p') => true,
        _ => false,
    }
}

pub fn left_event(key: Key) -> bool {
    match key {
        Key::Left | Key::Char('h') | Key::Ctrl('b') => true,
        _ => false,
    }
}

pub fn right_event(key: Key) -> bool {
    match key {
        Key::Right | Key::Char('l') | Key::Ctrl('f') => true,
        _ => false,
    }
}

pub fn on_down_press_handler<T>(selection_data: &[T], selection_index: Option<usize>) -> usize {
    match selection_index {
        Some(selection_index) => {
            if !selection_data.is_empty() {
                let next_index = selection_index + 1;
                if next_index > selection_data.len() - 1 {
                    return 0;
                } else {
                    return next_index;
                }
            }
            0
        }
        None => 0,
    }
}

pub fn on_up_press_handler<T>(selection_data: &[T], selection_index: Option<usize>) -> usize {
    match selection_index {
        Some(selection_index) => {
            if !selection_data.is_empty() {
                if selection_index > 0 {
                    return selection_index - 1;
                } else {
                    return selection_data.len() - 1;
                }
            }
            0
        }
        None => 0,
    }
}

pub fn handle_right_event(app: &mut App) {
    match app.get_current_route().hovered_block {
        ActiveBlock::MyPlaylists | ActiveBlock::Library => {
            match app.get_current_route().id {
                RouteId::AlbumTracks => {
                    app.set_current_route_state(
                        Some(ActiveBlock::AlbumTracks),
                        Some(ActiveBlock::AlbumTracks),
                    );
                }
                RouteId::TrackTable => {
                    app.set_current_route_state(
                        Some(ActiveBlock::TrackTable),
                        Some(ActiveBlock::TrackTable),
                    );
                }
                RouteId::Podcasts => {
                    app.set_current_route_state(
                        Some(ActiveBlock::Podcasts),
                        Some(ActiveBlock::Podcasts),
                    );
                }
                RouteId::AlbumList => {
                    app.set_current_route_state(
                        Some(ActiveBlock::AlbumList),
                        Some(ActiveBlock::AlbumList),
                    );
                }
                RouteId::MadeForYou => {
                    app.set_current_route_state(
                        Some(ActiveBlock::MadeForYou),
                        Some(ActiveBlock::MadeForYou),
                    );
                }
                RouteId::Artists => {
                    app.set_current_route_state(
                        Some(ActiveBlock::Artists),
                        Some(ActiveBlock::Artists),
                    );
                }
                RouteId::RecentlyPlayed => {
                    app.set_current_route_state(
                        Some(ActiveBlock::RecentlyPlayed),
                        Some(ActiveBlock::RecentlyPlayed),
                    );
                }
                RouteId::Search => {
                    app.set_current_route_state(
                        Some(ActiveBlock::SearchResultBlock),
                        Some(ActiveBlock::SearchResultBlock),
                    );
                }
                RouteId::Artist => {
                    // TODO
                }
                RouteId::Home => {
                    app.set_current_route_state(Some(ActiveBlock::Home), Some(ActiveBlock::Home));
                }
                RouteId::SelectedDevice => {}
                RouteId::Error => {}
            }
        }
        _ => {}
    };
}

pub fn handle_left_event(app: &mut App) {
    // TODO: This should send you back to either library or playlist based on last selection
    app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));
}

pub fn go_first_line(app: &mut App) {
    match app.get_current_route().active_block {
        ActiveBlock::AlbumList => {
            app.album_list_index = 0;
        }
        ActiveBlock::TrackTable => {
            app.track_table.selected_index = 0;
        }
        ActiveBlock::AlbumTracks => match app.album_table_context {
            AlbumTableContext::Full => {
                app.saved_album_tracks_index = 0;
            }
            AlbumTableContext::Simplified => {
                if let Some(selected_album) = &mut app.selected_album {
                    selected_album.selected_index = 0;
                }
            }
        },
        ActiveBlock::Artists => {
            app.artists_list_index = 0;
        }
        ActiveBlock::Artist => {
            if let Some(artist_albums) = &mut app.artist_albums {
                artist_albums.selected_index = 0;
            }
        }
        ActiveBlock::RecentlyPlayed => {
            app.recently_played.index = 0;
        }
        ActiveBlock::SearchResultBlock => match app.search_results.selected_block {
            SearchResultBlock::AlbumSearch => {
                app.search_results.selected_album_index = Some(0);
            }
            SearchResultBlock::SongSearch => {
                app.search_results.selected_tracks_index = Some(0);
            }
            SearchResultBlock::ArtistSearch => {
                app.search_results.selected_artists_index = Some(0);
            }
            SearchResultBlock::PlaylistSearch => {
                app.search_results.selected_playlists_index = Some(0);
            }
            SearchResultBlock::Empty => {}
        },
        ActiveBlock::MyPlaylists => {
            app.selected_playlist_index = Some(0);
        }
        _ => {}
    }
}

pub fn go_last_line(app: &mut App) {
    match app.get_current_route().active_block {
        ActiveBlock::AlbumList => {
            if let Some(albums) = app.library.saved_albums.get_results(None) {
                if !albums.items.is_empty() {
                    app.album_list_index = albums.items.len() - 1;
                }
            }
        }
        ActiveBlock::AlbumTracks => match app.album_table_context {
            AlbumTableContext::Full => {
                if let Some(albums) = &app.library.clone().saved_albums.get_results(None) {
                    if let Some(selected_album) = albums.items.get(app.album_list_index) {
                        let last_index = selected_album.album.tracks.items.len() - 1;
                        app.saved_album_tracks_index = last_index;
                    }
                }
            }
            AlbumTableContext::Simplified => {
                if let Some(selected_album) = &mut app.selected_album {
                    let last_index = selected_album.tracks.items.len() - 1;
                    selected_album.selected_index = last_index;
                }
            }
        },
        ActiveBlock::Artists => {
            if let Some(artists) = &mut app.library.saved_artists.get_results(None) {
                let last_index = artists.items.len() - 1;
                app.artists_list_index = last_index;
            }
        }
        ActiveBlock::Artist => {
            if let Some(artist_albums) = &mut app.artist_albums {
                let last_index = artist_albums.albums.items.len() - 1;
                artist_albums.selected_index = last_index;
            }
        }
        ActiveBlock::RecentlyPlayed => {
            if let Some(recently_played_result) = app.recently_played.result.clone() {
                let last_index = recently_played_result.items.len() - 1;
                app.recently_played.index = last_index;
            }
        }
        ActiveBlock::TrackTable => {
            let last_index = app.track_table.tracks.len() - 1;
            app.track_table.selected_index = last_index;
        }
        ActiveBlock::SearchResultBlock => match app.search_results.selected_block {
            SearchResultBlock::AlbumSearch => {
                if let Some(result) = &app.search_results.albums {
                    let last_index = Some(result.albums.items.len() - 1);
                    app.search_results.selected_album_index = last_index;
                }
            }
            SearchResultBlock::SongSearch => {
                if let Some(result) = &app.search_results.tracks {
                    let last_index = Some(result.tracks.items.len() - 1);
                    app.search_results.selected_tracks_index = last_index;
                }
            }
            SearchResultBlock::ArtistSearch => {
                if let Some(result) = &app.search_results.artists {
                    let last_index = Some(result.artists.items.len() - 1);
                    app.search_results.selected_artists_index = last_index;
                }
            }
            SearchResultBlock::PlaylistSearch => {
                if let Some(result) = &app.search_results.playlists {
                    let last_index = Some(result.playlists.items.len() - 1);
                    app.search_results.selected_playlists_index = last_index;
                }
            }
            SearchResultBlock::Empty => {}
        },
        ActiveBlock::MyPlaylists => {
            if let Some(playlists) = &app.playlists {
                let last_index = playlists.items.len() - 1;
                app.selected_playlist_index = Some(last_index);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_down_press_handler() {
        let data = vec!["Choice 1", "Choice 2", "Choice 3"];

        let index = 0;
        let next_index = on_down_press_handler(&data, Some(index));

        assert_eq!(next_index, 1);

        // Selection wrap if on last item
        let index = data.len() - 1;
        let next_index = on_down_press_handler(&data, Some(index));
        assert_eq!(next_index, 0);
    }

    #[test]
    fn test_on_up_press_handler() {
        let data = vec!["Choice 1", "Choice 2", "Choice 3"];

        let index = data.len() - 1;
        let next_index = on_up_press_handler(&data, Some(index));

        assert_eq!(next_index, index - 1);

        // Selection wrap if on first item
        let index = 0;
        let next_index = on_up_press_handler(&data, Some(index));
        assert_eq!(next_index, data.len() - 1);
    }
}
