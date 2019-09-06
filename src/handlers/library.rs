use super::super::app::{ActiveBlock, App, Routes, SongTableContext, LIBRARY_OPTIONS};
use super::common_key_events;
use rspotify::spotify::model::track::FullTrack;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.active_block = ActiveBlock::Empty;
        }
        Key::Char('d') => {
            app.handle_get_devices();
        }
        Key::Char(' ') => {
            app.toggle_playback();
        }
        k if common_key_events::down_event(k) => {
            let next_index = common_key_events::on_down_press_handler(
                &LIBRARY_OPTIONS,
                Some(app.library.selected_index),
            );
            app.library.selected_index = next_index;
        }
        Key::Char('?') => {
            app.active_block = ActiveBlock::HelpMenu;
        }
        k if common_key_events::up_event(k) => {
            let next_index = common_key_events::on_up_press_handler(
                &LIBRARY_OPTIONS,
                Some(app.library.selected_index),
            );
            app.library.selected_index = next_index;
        }
        Key::Char('/') => {
            app.active_block = ActiveBlock::Input;
            app.hovered_block = ActiveBlock::Input;
        }
        // This should probably be an array of structs with enums rather than just using indexes
        // like this
        Key::Char('\n') => match app.library.selected_index {
            // Made For You,
            0 => {}
            // Recently Played,
            1 => {}
            // Liked Songs,
            2 => {
                if let Some(spotify) = &app.spotify {
                    match spotify.current_user_saved_tracks(app.large_search_limit, None) {
                        Ok(saved_tracks) => {
                            app.songs_for_table = saved_tracks
                                .items
                                .clone()
                                .into_iter()
                                .map(|item| item.track)
                                .collect::<Vec<FullTrack>>();

                            app.library.saved_tracks = Some(saved_tracks);
                            app.active_block = ActiveBlock::SongTable;
                            app.hovered_block = ActiveBlock::SongTable;
                            app.song_table_context = Some(SongTableContext::SavedTracks);
                            app.navigation_stack.push(Routes::SongTable);
                        }
                        Err(e) => {
                            app.active_block = ActiveBlock::Error;
                            app.api_error = e.to_string();
                        }
                    }
                }
            }
            // Albums,
            3 => {}
            //  Artists,
            4 => {}
            // Podcasts,
            5 => {}
            _ => {}
        },
        _ => (),
    };
}
