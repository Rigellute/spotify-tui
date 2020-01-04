use super::common_key_events;
use crate::{
    app::{AlbumTableContext, App, RecommendationsContext},
    event::Key,
};

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
        k if common_key_events::down_event(k) => match app.album_table_context {
            AlbumTableContext::Full => {
                if let Some(albums) = &app.library.clone().saved_albums.get_results(None) {
                    if let Some(selected_album) = albums.items.get(app.album_list_index) {
                        let next_index = common_key_events::on_down_press_handler(
                            &selected_album.album.tracks.items,
                            Some(app.saved_album_tracks_index),
                        );

                        app.saved_album_tracks_index = next_index;
                    }
                };
            }
            AlbumTableContext::Simplified => {
                if let Some(selected_album) = &mut app.selected_album {
                    let next_index = common_key_events::on_down_press_handler(
                        &selected_album.tracks.items,
                        Some(selected_album.selected_index),
                    );
                    selected_album.selected_index = next_index;
                }
            }
        },
        k if common_key_events::up_event(k) => match app.album_table_context {
            AlbumTableContext::Full => {
                if let Some(albums) = &app.library.clone().saved_albums.get_results(None) {
                    if let Some(selected_album) = albums.items.get(app.album_list_index) {
                        let next_index = common_key_events::on_up_press_handler(
                            &selected_album.album.tracks.items,
                            Some(app.saved_album_tracks_index),
                        );

                        app.saved_album_tracks_index = next_index;
                    }
                };
            }
            AlbumTableContext::Simplified => {
                if let Some(selected_album) = &mut app.selected_album {
                    let next_index = common_key_events::on_up_press_handler(
                        &selected_album.tracks.items,
                        Some(selected_album.selected_index),
                    );
                    selected_album.selected_index = next_index;
                }
            }
        },
        Key::Char('s') => match app.album_table_context {
            AlbumTableContext::Full => {
                if let Some(albums) = &app.library.clone().saved_albums.get_results(None) {
                    if let Some(selected_album) = albums.items.get(app.album_list_index) {
                        if let Some(selected_track) = selected_album
                            .album
                            .tracks
                            .items
                            .get(app.saved_album_tracks_index)
                        {
                            if let Some(track_id) = &selected_track.id {
                                app.toggle_save_track(track_id.clone());
                            };
                        };
                    }
                };
            }
            AlbumTableContext::Simplified => {
                if let Some(selected_album) = app.selected_album.clone() {
                    if let Some(selected_track) = selected_album
                        .tracks
                        .items
                        .get(selected_album.selected_index)
                    {
                        if let Some(track_id) = &selected_track.id {
                            app.toggle_save_track(track_id.clone());
                        };
                    };
                };
            }
        },
        Key::Enter => match app.album_table_context {
            AlbumTableContext::Full => {
                if let Some(albums) = &app.library.clone().saved_albums.get_results(None) {
                    if let Some(selected_album) = albums.items.get(app.album_list_index) {
                        app.start_playback(
                            Some(selected_album.album.uri.to_owned()),
                            None,
                            Some(app.saved_album_tracks_index),
                        );
                    }
                };
            }
            AlbumTableContext::Simplified => {
                if let Some(selected_album) = &app.selected_album.clone() {
                    app.start_playback(
                        selected_album.album.uri.clone(),
                        None,
                        Some(selected_album.selected_index),
                    );
                };
            }
        },
        //recommended playlist based on selected track
        Key::Char('r') => match app.album_table_context {
            AlbumTableContext::Full => {
                if let Some(albums) = &app.library.clone().saved_albums.get_results(None) {
                    if let Some(selected_album) = albums.items.get(app.album_list_index) {
                        if let Some(track) = &selected_album
                            .album
                            .tracks
                            .items
                            .get(app.saved_album_tracks_index)
                        {
                            if let Some(id) = &track.id {
                                app.recommendations_context = Some(RecommendationsContext::Song);
                                app.recommendations_seed = track.name.clone();
                                app.get_recommendations_for_trackid(&id);
                            }
                        }
                    }
                }
            }
            AlbumTableContext::Simplified => {
                if let Some(selected_album) = &app.selected_album.clone() {
                    if let Some(track) = &selected_album
                        .tracks
                        .items
                        .get(selected_album.selected_index)
                    {
                        if let Some(id) = &track.id {
                            app.recommendations_context = Some(RecommendationsContext::Song);
                            app.recommendations_seed = track.name.clone();
                            app.get_recommendations_for_trackid(&id);
                        }
                    }
                };
            }
        },
        _ => {}
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::ActiveBlock;

    #[test]
    fn on_left_press() {
        let mut app = App::new();
        app.set_current_route_state(
            Some(ActiveBlock::AlbumTracks),
            Some(ActiveBlock::AlbumTracks),
        );

        handler(Key::Left, &mut app);
        let current_route = app.get_current_route();
        assert_eq!(current_route.active_block, ActiveBlock::Empty);
        assert_eq!(current_route.hovered_block, ActiveBlock::Library);
    }

    #[test]
    fn on_esc() {
        let mut app = App::new();

        handler(Key::Esc, &mut app);

        let current_route = app.get_current_route();
        assert_eq!(current_route.active_block, ActiveBlock::Empty);
    }
}
