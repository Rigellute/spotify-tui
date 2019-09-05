use super::super::app::{ActiveBlock, App, Routes, SongTableContext};
use super::common_key_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.active_block = ActiveBlock::Empty;
        }
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
        k if common_key_events::right_event(k) => {
            match app.get_current_route() {
                Some(route) => match route {
                    Routes::Search => {
                        app.active_block = ActiveBlock::SearchResultBlock;
                    }
                    Routes::SongTable => {
                        app.active_block = ActiveBlock::SongTable;
                    }
                    Routes::Album => {
                        app.active_block = ActiveBlock::Album;
                    }
                    Routes::Artist(_artist_id) => {}
                },
                None => {
                    app.active_block = ActiveBlock::Home;
                }
            };
        }
        k if common_key_events::down_event(k) => {
            match &app.playlists {
                Some(p) => {
                    if let Some(selected_playlist_index) = app.selected_playlist_index {
                        let next_index = common_key_events::on_down_press_handler(
                            &p.items,
                            Some(selected_playlist_index),
                        );
                        app.selected_playlist_index = Some(next_index);
                    }
                }
                None => {}
            };
        }
        k if common_key_events::up_event(k) => {
            match &app.playlists {
                Some(p) => {
                    let next_index = common_key_events::on_up_press_handler(
                        &p.items,
                        app.selected_playlist_index,
                    );
                    app.selected_playlist_index = Some(next_index);
                }
                None => {}
            };
        }
        Key::Char('/') => {
            app.active_block = ActiveBlock::Input;
            app.hovered_block = ActiveBlock::Input;
        }
        Key::Char('\n') => {
            if let (Some(playlists), Some(selected_playlist_index)) =
                (&app.playlists, &app.selected_playlist_index)
            {
                app.song_table_context = Some(SongTableContext::MyPlaylists);
                if let Some(selected_playlist) =
                    playlists.items.get(selected_playlist_index.to_owned())
                {
                    let playlist_id = selected_playlist.id.to_owned();
                    app.get_playlist_tracks(playlist_id);
                }
            };
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playlist_handler_activate_help_menu() {
        let mut app = App::new();

        handler(Key::Char('?'), &mut app);
        assert_eq!(app.active_block, ActiveBlock::HelpMenu);
    }
}
