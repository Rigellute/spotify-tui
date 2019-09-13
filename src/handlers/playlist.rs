use super::super::app::{ActiveBlock, App, RouteId, SongTableContext};
use super::common_key_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::right_event(k) => {
            match app.get_current_route().id {
                RouteId::Search => {
                    app.set_current_route_state(Some(ActiveBlock::SearchResultBlock), None);
                }
                RouteId::TrackTable => {
                    app.set_current_route_state(Some(ActiveBlock::TrackTable), None);
                }
                RouteId::AlbumTracks => {
                    app.set_current_route_state(Some(ActiveBlock::AlbumTracks), None);
                }
                RouteId::Artist => {
                    // TODO
                }
                RouteId::Home => {
                    app.set_current_route_state(Some(ActiveBlock::Home), None);
                }
                _ => {}
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
        Key::Char('\n') => {
            if let (Some(playlists), Some(selected_playlist_index)) =
                (&app.playlists, &app.selected_playlist_index)
            {
                app.track_table.context = Some(SongTableContext::MyPlaylists);
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
    #[test]
    fn test() {}
}
