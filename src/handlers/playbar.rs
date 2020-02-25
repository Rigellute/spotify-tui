use super::{
    super::app::{ActiveBlock, App},
    common_key_events,
};
use crate::event::Key;

pub async fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::up_event(k) => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::MyPlaylists));
        }
        Key::Char('s') => {
            if let Some(playing_context) = &app.current_playback_context {
                if let Some(track) = &playing_context.item {
                    if let Some(id) = track.id.to_owned() {
                        app.toggle_save_track(id).await;
                    }
                }
            }
        }
        _ => {}
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn on_left_press() {
        let mut app = App::new();
        app.set_current_route_state(Some(ActiveBlock::PlayBar), Some(ActiveBlock::PlayBar));

        handler(Key::Up, &mut app).await;
        let current_route = app.get_current_route();
        assert_eq!(current_route.active_block, ActiveBlock::Empty);
        assert_eq!(current_route.hovered_block, ActiveBlock::MyPlaylists);
    }
}
