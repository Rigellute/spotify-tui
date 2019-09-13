use super::super::app::{ActiveBlock, App};
use super::common_key_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::left_event(k) => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));
        }
        k if common_key_events::down_event(k) => {
            if let Some(recently_played_result) = &app.recently_played.result {
                let next_index = common_key_events::on_down_press_handler(
                    &recently_played_result.items,
                    Some(app.recently_played.index),
                );
                app.recently_played.index = next_index;
            }
        }
        k if common_key_events::up_event(k) => {
            if let Some(recently_played_result) = &app.recently_played.result {
                let next_index = common_key_events::on_up_press_handler(
                    &recently_played_result.items,
                    Some(app.recently_played.index),
                );
                app.recently_played.index = next_index;
            }
        }
        Key::Char('\n') => {
            if let Some(recently_played_result) = &app.recently_played.result.clone() {
                let track_uris: Vec<String> = recently_played_result
                    .items
                    .iter()
                    .map(|item| item.track.uri.to_owned())
                    .collect();

                app.start_playback(None, Some(track_uris), Some(app.recently_played.index));
            };
        }
        _ => {}
    };
}

#[cfg(test)]
mod tests {
    use super::*;

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
