use super::super::app::{ActiveBlock, App, Routes};
use rspotify::spotify::senum::Country;
use termion::event::Key;

// Handle events when the search input block is active
pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Ctrl('u') => {
            app.input = String::new();
        }
        Key::Esc => {
            // IDEA: Perhaps this should return to previous hovered_block?
            app.active_block = ActiveBlock::Empty;
            app.hovered_block = ActiveBlock::Library;
        }
        Key::Char('\n') => {
            if let Some(spotify) = &app.spotify {
                // TODO: This should be definable by the user
                let country = Some(Country::UnitedKingdom);

                let result = spotify
                    .search_track(&app.input, app.small_search_limit, 0, country)
                    // TODO handle the error properly
                    .expect("Failed to fetch spotify tracks");

                app.songs_for_table = result.tracks.items.clone();
                app.search_results.tracks = Some(result);

                // On searching for a track, clear the playlist selection
                app.selected_playlist_index = None;
                app.active_block = ActiveBlock::SearchResultBlock;
                app.hovered_block = ActiveBlock::SearchResultBlock;
                app.navigation_stack.push(Routes::Search);

                // Can I run these functions in parellel?
                let result = spotify
                    .search_artist(
                        &app.input,
                        app.small_search_limit,
                        0,
                        Some(Country::UnitedKingdom),
                    )
                    .expect("Failed to fetch artists");
                app.search_results.artists = Some(result);

                let result = spotify
                    .search_album(
                        &app.input,
                        app.small_search_limit,
                        0,
                        Some(Country::UnitedKingdom),
                    )
                    .expect("Failed to fetch albums");
                app.search_results.albums = Some(result);

                let result = spotify
                    .search_playlist(
                        &app.input,
                        app.small_search_limit,
                        0,
                        Some(Country::UnitedKingdom),
                    )
                    .expect("Failed to fetch playlists");
                app.search_results.playlists = Some(result);
            }
        }
        Key::Char(c) => {
            app.input.push(c);
        }
        Key::Backspace => {
            app.input.pop();
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

        handler(Key::Esc, &mut app);

        assert_eq!(app.active_block, ActiveBlock::Empty);
    }

    #[test]
    fn test_input_handler_on_enter_text() {
        let mut app = App::new();

        app.input = "My tex".to_string();

        handler(Key::Char('t'), &mut app);

        assert_eq!(app.input, "My text".to_string());
    }

    #[test]
    fn test_input_handler_backspace() {
        let mut app = App::new();

        app.input = "My text".to_string();

        handler(Key::Backspace, &mut app);

        assert_eq!(app.input, "My tex".to_string());
    }
}
