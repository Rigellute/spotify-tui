use super::app::{ActiveBlock, App, EventLoop, LIMIT};
use rspotify::spotify::model::offset::for_position;
use rspotify::spotify::model::track::FullTrack;
use rspotify::spotify::senum::Country;

use termion::event::Key;

pub fn input_handler(key: Key, app: &mut App) -> Option<EventLoop> {
    match key {
        Key::Char('q') | Key::Ctrl('c') => Some(EventLoop::Exit),
        Key::Ctrl('u') => {
            app.input = String::new();
            None
        }
        Key::Esc => {
            app.active_block = ActiveBlock::MyPlaylist;
            None
        }
        Key::Char('\n') => {
            if let Some(spotify) = &app.spotify {
                // TODO: This should be definable by the user
                let country = Some(Country::UnitedKingdom);
                let result = spotify
                    .search_track(&app.input, LIMIT / 2, 0, country)
                    // TODO handle the error properly
                    .expect("Failed to fetch spotify tracks");

                app.songs_for_table = result.tracks.items.clone();
                app.searched_tracks = Some(result);

                // On searching for a track, clear the playlist selection
                app.selected_playlist_index = None;
                app.active_block = ActiveBlock::SongTable;

                // Can I run these functions in parellel?
                let result = spotify
                    .search_artist(&app.input, LIMIT / 2, 0, Some(Country::UnitedKingdom))
                    .expect("Failed to fetch artists");
                app.searched_artists = Some(result);

                let result = spotify
                    .search_album(&app.input, LIMIT / 2, 0, Some(Country::UnitedKingdom))
                    .expect("Failed to fetch albums");
                app.searched_albums = Some(result);

                let result = spotify
                    .search_playlist(&app.input, LIMIT / 2, 0, Some(Country::UnitedKingdom))
                    .expect("Failed to fetch playlists");
                app.searched_playlists = Some(result);
            }
            None
        }
        Key::Char(c) => {
            app.input.push(c);
            None
        }
        Key::Backspace => {
            app.input.pop();
            None
        }
        _ => None,
    }
}

pub fn playlist_handler(key: Key, app: &mut App) -> Option<EventLoop> {
    match key {
        Key::Char('q') | Key::Ctrl('c') => Some(EventLoop::Exit),
        Key::Char('d') => {
            if let Some(spotify) = &app.spotify {
                match spotify.device() {
                    Ok(devices) => {
                        app.active_block = ActiveBlock::SelectDevice;
                        app.devices = Some(devices);
                    }

                    Err(e) => {
                        app.active_block = ActiveBlock::ApiError;
                        app.api_error = e.to_string();
                    }
                }
            }
            None
        }
        Key::Char('?') => {
            app.active_block = ActiveBlock::HelpMenu;
            None
        }
        Key::Right | Key::Char('l') => {
            app.active_block = ActiveBlock::SongTable;
            None
        }
        Key::Down | Key::Char('j') => match &app.playlists {
            Some(p) => {
                if !p.items.is_empty() {
                    app.selected_playlist_index =
                        if let Some(selected_playlist_index) = app.selected_playlist_index {
                            if selected_playlist_index >= p.items.len() - 1 {
                                Some(0)
                            } else {
                                Some(selected_playlist_index + 1)
                            }
                        } else {
                            Some(0)
                        }
                }
                None
            }
            None => None,
        },
        Key::Up | Key::Char('k') => match &app.playlists {
            Some(p) => {
                if !p.items.is_empty() {
                    app.selected_playlist_index =
                        if let Some(selected_playlist_index) = app.selected_playlist_index {
                            if selected_playlist_index > 0 {
                                Some(selected_playlist_index - 1)
                            } else {
                                Some(p.items.len() - 1)
                            }
                        } else {
                            Some(0)
                        }
                }
                None
            }
            None => None,
        },
        Key::Char('/') => {
            app.active_block = ActiveBlock::Input;
            None
        }
        Key::Char('\n') => match (&app.spotify, &app.playlists, &app.selected_playlist_index) {
            (Some(spotify), Some(playlists), Some(selected_playlist_index)) => {
                if let Some(selected_playlist) =
                    playlists.items.get(selected_playlist_index.to_owned())
                {
                    let playlist_id = selected_playlist.id.to_owned();
                    if let Ok(playlist_tracks) = spotify.user_playlist_tracks(
                        "spotify",
                        &playlist_id,
                        None,
                        Some(LIMIT),
                        None,
                        None,
                    ) {
                        app.songs_for_table = playlist_tracks
                            .items
                            .clone()
                            .into_iter()
                            .map(|item| item.track)
                            .collect::<Vec<FullTrack>>();

                        app.playlist_tracks = playlist_tracks.items;
                        app.active_block = ActiveBlock::SongTable;
                    };
                }
                None
            }
            _ => None,
        },
        _ => None,
    }
}

pub fn song_table_handler(key: Key, app: &mut App) -> Option<EventLoop> {
    match key {
        Key::Char('q') | Key::Ctrl('c') => Some(EventLoop::Exit),
        Key::Char('d') => {
            if let Some(spotify) = &app.spotify {
                if let Ok(devices) = spotify.device() {
                    app.active_block = ActiveBlock::SelectDevice;
                    app.devices = Some(devices);
                }
            }
            None
        }
        Key::Left | Key::Char('h') => {
            app.active_block = ActiveBlock::MyPlaylist;
            None
        }
        Key::Down | Key::Char('j') => {
            if !app.songs_for_table.is_empty() {
                app.select_song_index += 1;
                if app.select_song_index > app.songs_for_table.len() - 1 {
                    app.select_song_index = 0;
                }
            }
            None
        }
        Key::Char('?') => {
            app.active_block = ActiveBlock::HelpMenu;
            None
        }
        Key::Up | Key::Char('k') => {
            if !app.songs_for_table.is_empty() {
                if app.select_song_index > 0 {
                    app.select_song_index -= 1;
                } else {
                    app.select_song_index = app.songs_for_table.len() - 1;
                }
            }
            None
        }
        Key::Char('/') => {
            app.active_block = ActiveBlock::Input;
            None
        }
        Key::Char('\n') => {
            if let Some(track) = app.songs_for_table.get(app.select_song_index) {
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

                let device_id = app.device_id.take();

                // I need to pass in different arguments here, how can this be
                // nicer?
                if let Some(spotify) = &app.spotify {
                    if let Some(context_uri) = context_uri {
                        match spotify.start_playback(
                            device_id,
                            Some(context_uri),
                            None,
                            for_position(app.select_song_index as u32),
                        ) {
                            Ok(_r) => {
                                app.current_playing_song = Some(track.to_owned());
                            }
                            Err(e) => {
                                app.active_block = ActiveBlock::ApiError;
                                app.api_error = e.to_string();
                            }
                        }
                    } else {
                        match spotify.start_playback(
                            device_id,
                            None,
                            Some(vec![track.uri.to_owned()]),
                            for_position(0),
                        ) {
                            Ok(_r) => {
                                app.current_playing_song = Some(track.to_owned());
                            }
                            Err(e) => {
                                app.active_block = ActiveBlock::ApiError;
                                app.api_error = e.to_string();
                            }
                        }
                    }
                }
            };
            None
        }
        _ => None,
    }
}

pub fn help_menu_handler(key: Key, app: &mut App) -> Option<EventLoop> {
    match key {
        Key::Char('q') | Key::Ctrl('c') => Some(EventLoop::Exit),
        Key::Esc => {
            app.active_block = ActiveBlock::MyPlaylist;
            None
        }
        _ => None,
    }
}

pub fn api_error_menu_handler(key: Key, app: &mut App) -> Option<EventLoop> {
    match key {
        Key::Char('q') | Key::Ctrl('c') => Some(EventLoop::Exit),
        Key::Esc => {
            app.active_block = ActiveBlock::MyPlaylist;
            None
        }
        _ => None,
    }
}

pub fn select_device_handler(key: Key, app: &mut App) -> Option<EventLoop> {
    match key {
        Key::Char('q') | Key::Ctrl('c') => Some(EventLoop::Exit),
        Key::Esc => {
            app.active_block = ActiveBlock::MyPlaylist;
            None
        }
        Key::Down | Key::Char('j') => match &app.devices {
            Some(p) => {
                if !p.devices.is_empty() {
                    app.selected_device_index =
                        if let Some(selected_device_index) = app.selected_device_index {
                            if selected_device_index >= p.devices.len() - 1 {
                                Some(0)
                            } else {
                                Some(selected_device_index + 1)
                            }
                        } else {
                            Some(0)
                        }
                }
                None
            }
            None => None,
        },
        Key::Up | Key::Char('k') => match &app.devices {
            Some(p) => {
                if !p.devices.is_empty() {
                    app.selected_device_index =
                        if let Some(selected_device_index) = app.selected_device_index {
                            if selected_device_index > 0 {
                                Some(selected_device_index - 1)
                            } else {
                                Some(p.devices.len() - 1)
                            }
                        } else {
                            Some(0)
                        }
                }
                None
            }
            None => None,
        },
        Key::Char('\n') => match (&app.devices, app.selected_device_index) {
            (Some(devices), Some(index)) => {
                if let Some(device) = devices.devices.get(index) {
                    app.device_id = Some(device.id.to_owned());
                    app.active_block = ActiveBlock::MyPlaylist;
                }
                None
            }
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_handler_quits() {
        let mut app = App::new();

        let result = input_handler(Key::Char('q'), &mut app);
        assert_eq!(result, Some(EventLoop::Exit));

        let result = input_handler(Key::Ctrl('c'), &mut app);
        assert_eq!(result, Some(EventLoop::Exit));
    }

    #[test]
    fn test_input_handler_clear_input_on_ctrl_u() {
        let mut app = App::new();

        app.input = "My text".to_string();

        let result = input_handler(Key::Ctrl('u'), &mut app);

        assert_eq!(result, None);
        assert_eq!(app.input, "".to_string());
    }

    #[test]
    fn test_input_handler_esc_back_to_playlist() {
        let mut app = App::new();

        let result = input_handler(Key::Esc, &mut app);

        assert_eq!(result, None);
        assert_eq!(app.active_block, ActiveBlock::MyPlaylist);
    }

    #[test]
    fn test_input_handler_on_enter_text() {
        let mut app = App::new();

        app.input = "My tex".to_string();

        let result = input_handler(Key::Char('t'), &mut app);

        assert_eq!(result, None);
        assert_eq!(app.input, "My text".to_string());
    }

    #[test]
    fn test_input_handler_backspace() {
        let mut app = App::new();

        app.input = "My text".to_string();

        let result = input_handler(Key::Backspace, &mut app);

        assert_eq!(result, None);
        assert_eq!(app.input, "My tex".to_string());
    }

    #[test]
    fn test_playlist_handler_quit() {
        let mut app = App::new();

        let result = playlist_handler(Key::Char('q'), &mut app);
        assert_eq!(result, Some(EventLoop::Exit));

        let result = playlist_handler(Key::Ctrl('c'), &mut app);
        assert_eq!(result, Some(EventLoop::Exit));
    }

    #[test]
    fn test_playlist_handler_activate_help_menu() {
        let mut app = App::new();

        let result = playlist_handler(Key::Char('?'), &mut app);
        assert_eq!(result, None);
        assert_eq!(app.active_block, ActiveBlock::HelpMenu);
    }
}
