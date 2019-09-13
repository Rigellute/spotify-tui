use super::super::app::{ActiveBlock, AlbumTableContext, App, RouteId, SelectedFullAlbum};
use super::common_key_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::left_event(k) => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));
        }
        k if common_key_events::down_event(k) => {
            if let Some(albums) = &mut app.library.saved_albums.get_results(None) {
                let next_index = common_key_events::on_down_press_handler(
                    &albums.items,
                    Some(app.album_list_index),
                );
                app.album_list_index = next_index;
            }
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
        Key::Char('\n') => {
            if let Some(albums) = app.library.saved_albums.get_results(None) {
                if let Some(selected_album) = albums.items.get(app.album_list_index) {
                    app.selected_album_full = Some(SelectedFullAlbum {
                        album: selected_album.album.clone(),
                        selected_index: 0,
                    });
                    app.album_table_context = AlbumTableContext::Full;
                    app.push_navigation_stack(RouteId::AlbumTracks, ActiveBlock::AlbumTracks);
                };
            }
        }
        _ => {}
    };
}

#[cfg(test)]
mod tests {
    use super::super::super::app::RouteId;
    use super::*;

    #[test]
    fn help_menu() {
        let mut app = App::new();
        app.push_navigation_stack(RouteId::AlbumTracks, ActiveBlock::AlbumTracks);
        handler(Key::Char('?'), &mut app);
        let current_route = app.get_current_route();

        assert_eq!(current_route.active_block, ActiveBlock::HelpMenu);
    }

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
    fn go_to_search_input() {
        let mut app = App::new();

        handler(Key::Char('/'), &mut app);

        let current_route = app.get_current_route();
        assert_eq!(current_route.active_block, ActiveBlock::Input);
        assert_eq!(current_route.hovered_block, ActiveBlock::Input);
    }

    #[test]
    fn on_esc() {
        let mut app = App::new();

        handler(Key::Esc, &mut app);

        let current_route = app.get_current_route();
        assert_eq!(current_route.active_block, ActiveBlock::Empty);
    }
}
