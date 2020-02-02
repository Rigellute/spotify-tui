extern crate unicode_width;

use super::super::app::{ActiveBlock, App, RouteId};
use crate::event::Key;
use rspotify::spotify::senum::Country;
use std::convert::TryInto;
use std::str::FromStr;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

// Handle event when the search input block is active
pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Ctrl('u') => {
            app.input = vec![];
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        Key::Ctrl('e') => {
            app.input_idx = app.input.len();
            let input_string: String = app.input.iter().collect();
            app.input_cursor_position = UnicodeWidthStr::width(input_string.as_str())
                .try_into()
                .unwrap();
        }
        Key::Ctrl('a') => {
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        Key::Left => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let last_c = app.input[app.input_idx - 1];
                app.input_idx -= 1;
                app.input_cursor_position -= compute_character_width(last_c);
            }
        }
        Key::Right => {
            if app.input_idx < app.input.len() {
                let next_c = app.input[app.input_idx];
                app.input_idx += 1;
                app.input_cursor_position += compute_character_width(next_c);
            }
        }
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));
        }
        Key::Enter => {
            if let (Some(spotify), Some(user)) = (app.spotify.clone(), app.user.clone()) {
                let country =
                    Country::from_str(&user.country.unwrap_or_else(|| "".to_string())).ok();
                let input_str: String = app.input.iter().collect();
                // Can I run these functions in parellel?
                match spotify.search_track(&input_str, app.small_search_limit, 0, country) {
                    Ok(result) => {
                        app.set_tracks_to_table(result.tracks.items.clone());
                        app.search_results.tracks = Some(result);
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }

                match spotify.search_artist(&input_str, app.small_search_limit, 0, country) {
                    Ok(result) => {
                        app.search_results.artists = Some(result);
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }

                match spotify.search_album(&input_str, app.small_search_limit, 0, country) {
                    Ok(result) => {
                        app.search_results.albums = Some(result);
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }

                match spotify.search_playlist(&input_str, app.small_search_limit, 0, country) {
                    Ok(result) => {
                        app.search_results.playlists = Some(result);
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }

                // On searching for a track, clear the playlist selection
                app.selected_playlist_index = Some(0);
                app.push_navigation_stack(RouteId::Search, ActiveBlock::SearchResultBlock);
            }
        }
        Key::Char(c) => {
            app.input.insert(app.input_idx, c);
            app.input_idx += 1;
            app.input_cursor_position += compute_character_width(c);
        }
        Key::Backspace => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let last_c = app.input.remove(app.input_idx - 1);
                app.input_idx -= 1;
                app.input_cursor_position -= compute_character_width(last_c);
            }
        }
        Key::Delete => {
            if !app.input.is_empty() && app.input_idx < app.input.len() {
                app.input.remove(app.input_idx);
            }
        }
        _ => {}
    }
}

fn compute_character_width(character: char) -> u16 {
    UnicodeWidthChar::width(character)
        .unwrap()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_vec_char(s: &str) -> Vec<char> {
        String::from(s).chars().collect()
    }

    #[test]
    fn test_input_handler_clear_input_on_ctrl_u() {
        let mut app = App::new();

        app.input = str_to_vec_char("My text");

        handler(Key::Ctrl('u'), &mut app);

        assert_eq!(app.input, str_to_vec_char(""));
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

        app.input = str_to_vec_char("My tex");
        app.input_cursor_position = app.input.len().try_into().unwrap();
        app.input_idx = app.input.len();

        handler(Key::Char('t'), &mut app);

        assert_eq!(app.input, str_to_vec_char("My text"));
    }

    #[test]
    fn test_input_handler_backspace() {
        let mut app = App::new();

        app.input = str_to_vec_char("My text");
        app.input_cursor_position = app.input.len().try_into().unwrap();
        app.input_idx = app.input.len();

        handler(Key::Backspace, &mut app);
        assert_eq!(app.input, str_to_vec_char("My tex"));

        // Test that backspace deletes from the cursor position
        app.input_idx = 2;
        app.input_cursor_position = 2;

        handler(Key::Backspace, &mut app);
        assert_eq!(app.input, str_to_vec_char("M tex"));
    }

    #[test]
    fn test_input_handler_delete() {
        let mut app = App::new();

        app.input = str_to_vec_char("My text");
        app.input_idx = 3;
        app.input_cursor_position = 3;

        handler(Key::Delete, &mut app);
        assert_eq!(app.input, str_to_vec_char("My ext"));

        app.input = str_to_vec_char("ラスト");
        app.input_idx = 1;
        app.input_cursor_position = 1;

        handler(Key::Delete, &mut app);
        assert_eq!(app.input, str_to_vec_char("ラト"));
    }

    #[test]
    fn test_input_handler_left_event() {
        let mut app = App::new();

        app.input = str_to_vec_char("My text");
        let input_len = app.input.len().try_into().unwrap();
        app.input_idx = app.input.len();
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

    #[test]
    fn test_input_handler_on_enter_text_non_english_char() {
        let mut app = App::new();

        app.input = str_to_vec_char("ыа");
        app.input_cursor_position = app.input.len().try_into().unwrap();
        app.input_idx = app.input.len();

        handler(Key::Char('ы'), &mut app);

        assert_eq!(app.input, str_to_vec_char("ыаы"));
    }

    #[test]
    fn test_input_handler_on_enter_text_wide_char() {
        let mut app = App::new();

        app.input = str_to_vec_char("你");
        app.input_cursor_position = 2; // 你 is 2 char wide
        app.input_idx = 1; // 1 char

        handler(Key::Char('好'), &mut app);

        assert_eq!(app.input, str_to_vec_char("你好"));
        assert_eq!(app.input_idx, 2);
        assert_eq!(app.input_cursor_position, 4);
    }
}
