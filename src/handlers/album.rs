use super::super::app::{ActiveBlock, App};
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
        k if common_key_events::left_event(k) => {
            app.active_block = ActiveBlock::Empty;
            app.hovered_block = ActiveBlock::Library;
        }
        k if common_key_events::down_event(k) => {
            if let Some(selected_album) = &mut app.selected_album {
                let next_index = common_key_events::on_down_press_handler(
                    &selected_album.tracks.items,
                    selected_album.selected_index,
                );
                selected_album.selected_index = Some(next_index);
            }
        }
        Key::Char('?') => {
            app.active_block = ActiveBlock::HelpMenu;
        }
        k if common_key_events::up_event(k) => {
            if let Some(selected_album) = &mut app.selected_album {
                let next_index = common_key_events::on_up_press_handler(
                    &selected_album.tracks.items,
                    selected_album.selected_index,
                );
                selected_album.selected_index = Some(next_index);
            }
        }
        Key::Char('/') => {
            app.active_block = ActiveBlock::Input;
            app.hovered_block = ActiveBlock::Input;
        }
        Key::Char('\n') => {
            if let Some(selected_album) = &app.selected_album.clone() {
                app.start_playback(
                    selected_album.album.uri.clone(),
                    None,
                    selected_album.selected_index,
                );
            };
        }
        _ => {}
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_menu() {
        let mut app = App::new();

        handler(Key::Char('?'), &mut app);

        assert_eq!(app.active_block, ActiveBlock::HelpMenu);
    }

    #[test]
    fn on_left_press() {
        let mut app = App::new();
        app.active_block = ActiveBlock::Album;
        app.hovered_block = ActiveBlock::Album;

        handler(Key::Left, &mut app);
        assert_eq!(app.active_block, ActiveBlock::Empty);
        assert_eq!(app.hovered_block, ActiveBlock::Library);
    }

    #[test]
    fn go_to_search_input() {
        let mut app = App::new();

        handler(Key::Char('/'), &mut app);

        assert_eq!(app.active_block, ActiveBlock::Input);
        assert_eq!(app.hovered_block, ActiveBlock::Input);
    }

    #[test]
    fn on_esc() {
        let mut app = App::new();

        handler(Key::Esc, &mut app);

        assert_eq!(app.active_block, ActiveBlock::Empty);
    }
}
