use super::super::app::{ActiveBlock, App, RouteId};
use std::convert::TryInto;
use termion::event::Key;

// Handle events when the search input block is active
pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Ctrl('u') => {
            app.input = String::new();
            app.input_cursor_position = 0;
        }
        Key::Ctrl('e') => {
            app.input_cursor_position = app.input.len().try_into().unwrap();
        }
        Key::Ctrl('a') => {
            app.input_cursor_position = 0;
        }
        Key::Left => {
            if !app.input.is_empty() && app.input_cursor_position > 0 {
                app.input_cursor_position -= 1;
            }
        }
        Key::Right => {
            if app.input_cursor_position < app.input.len().try_into().unwrap() {
                app.input_cursor_position += 1;
            }
        }
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));
        }
        Key::Char('\n') => {
            if let Some(spotify) = app.spotify.clone() {
                // Can I run these functions in parellel?
                match spotify.search_track(
                    &app.input,
                    app.small_search_limit,
                    0,
                    Some(app.country.clone()),
                ) {
                    Ok(result) => {
                        app.track_table.tracks = result.tracks.items.clone();
                        app.search_results.tracks = Some(result);
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }

                match spotify.search_artist(
                    &app.input,
                    app.small_search_limit,
                    0,
                    Some(app.country.clone()),
                ) {
                    Ok(result) => {
                        app.search_results.artists = Some(result);
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }

                match spotify.search_album(
                    &app.input,
                    app.small_search_limit,
                    0,
                    Some(app.country.clone()),
                ) {
                    Ok(result) => {
                        app.search_results.albums = Some(result);
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }

                match spotify.search_playlist(
                    &app.input,
                    app.small_search_limit,
                    0,
                    Some(app.country.clone()),
                ) {
                    Ok(result) => {
                        app.search_results.playlists = Some(result);
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }

                // On searching for a track, clear the playlist selection
                app.selected_playlist_index = None;
                app.push_navigation_stack(RouteId::Search, ActiveBlock::SearchResultBlock);
            }
        }
        Key::Char(c) => {
            app.input
                .insert(app.input_cursor_position.try_into().unwrap(), c);
            app.input_cursor_position += 1;
        }
        Key::Backspace => {
            if !app.input.is_empty() && app.input_cursor_position > 0 {
                app.input
                    .remove((app.input_cursor_position - 1).try_into().unwrap());
                app.input_cursor_position -= 1;
            }
        }
        Key::Delete => {
            if !app.input.is_empty() && app.input_cursor_position < app.input.len().try_into().unwrap() {
                app.input
                    .remove((app.input_cursor_position).try_into().unwrap());
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_handler_clear_input_on_ctrl_u() {
        let mut app = App::new();

        app.input = "My text".to_string();

        handler(Key::Ctrl('u'), &mut app);

        assert_eq!(app.input, "".to_string());
    }

    #[test]
    fn test_input_handler_esc_back_to_playlist() {
        let mut app = App::new();

        app.set_current_route_state(Some(ActiveBlock::MyPlaylists), None);
        handler(Key::Esc, &mut app);

        let current_route = app.get_current_route();
        assert_eq!(current_route.active_block, ActiveBlock::Empty);
    }

    #[test]
    fn test_input_handler_on_enter_text() {
        let mut app = App::new();

        app.input = "My tex".to_string();
        app.input_cursor_position = app.input.len().try_into().unwrap();

        handler(Key::Char('t'), &mut app);

        assert_eq!(app.input, "My text".to_string());
    }

    #[test]
    fn test_input_handler_backspace() {
        let mut app = App::new();

        app.input = "My text".to_string();
        app.input_cursor_position = app.input.len().try_into().unwrap();

        handler(Key::Backspace, &mut app);
        assert_eq!(app.input, "My tex".to_string());

        // Test that backspace deletes from the cursor position
        app.input_cursor_position = 2;

        handler(Key::Backspace, &mut app);
        assert_eq!(app.input, "M tex".to_string());
    }

    #[test]
    fn test_input_handler_delete() {
        let mut app = App::new();

        app.input = "My text".to_string();
        app.input_cursor_position = 3;

        handler(Key::Delete, &mut app);
        assert_eq!(app.input, "My ext".to_string());
    }

    #[test]
    fn test_input_handler_left_event() {
        let mut app = App::new();

        app.input = "My text".to_string();
        let input_len = app.input.len().try_into().unwrap();
        app.input_cursor_position = input_len;

        handler(Key::Left, &mut app);
        assert_eq!(app.input_cursor_position, input_len - 1);
        handler(Key::Left, &mut app);
        assert_eq!(app.input_cursor_position, input_len - 2);
        handler(Key::Left, &mut app);
        assert_eq!(app.input_cursor_position, input_len - 3);

        // Pretend to smash the left event to test the we have no out-of-bounds crash
        for _ in 0..20 {
            handler(Key::Left, &mut app);
        }

        assert_eq!(app.input_cursor_position, 0);
    }
}
