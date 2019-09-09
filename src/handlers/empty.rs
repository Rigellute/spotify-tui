use super::super::app::{ActiveBlock, App, RouteId};
use super::common_key_events;
use termion::event::Key;

// When no block is actively selected, just handle regular events
pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Char('d') => {
            app.handle_get_devices();
        }
        Key::Char(' ') => {
            app.toggle_playback();
        }
        Key::Char('?') => {
            app.set_current_route_state(Some(ActiveBlock::HelpMenu), None);
        }
        Key::Char('/') => {
            app.set_current_route_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
        }
        Key::Char('\n') => {
            let current_hovered = app.get_current_route().hovered_block;
            app.set_current_route_state(Some(current_hovered), None);
        }
        k if common_key_events::down_event(k) => match app.get_current_route().hovered_block {
            ActiveBlock::Library => {
                app.set_current_route_state(None, Some(ActiveBlock::MyPlaylists));
            }
            ActiveBlock::MyPlaylists => {
                // Go to player
            }
            ActiveBlock::Album | ActiveBlock::Home | ActiveBlock::SongTable => {
                // Go to player
            }
            _ => {}
        },
        k if common_key_events::up_event(k) => {
            if let ActiveBlock::MyPlaylists = app.get_current_route().hovered_block {
                app.set_current_route_state(None, Some(ActiveBlock::Library));
            }
        }
        k if common_key_events::left_event(k) => match app.get_current_route().hovered_block {
            ActiveBlock::Album | ActiveBlock::Home | ActiveBlock::SongTable => {
                app.set_current_route_state(None, Some(ActiveBlock::Library));
            }
            _ => {}
        },
        k if common_key_events::right_event(k) => match app.get_current_route().hovered_block {
            ActiveBlock::MyPlaylists | ActiveBlock::Library => {
                match app.get_current_route().id {
                    RouteId::Album => {
                        app.set_current_route_state(
                            Some(ActiveBlock::Album),
                            Some(ActiveBlock::Album),
                        );
                    }
                    RouteId::SongTable => {
                        app.set_current_route_state(
                            Some(ActiveBlock::SongTable),
                            Some(ActiveBlock::SongTable),
                        );
                    }
                    RouteId::Search => {
                        app.set_current_route_state(
                            Some(ActiveBlock::SearchResultBlock),
                            Some(ActiveBlock::SearchResultBlock),
                        );
                    }
                    RouteId::Artist => {
                        // TODO
                    }
                    RouteId::Home => {
                        app.set_current_route_state(
                            Some(ActiveBlock::Home),
                            Some(ActiveBlock::Home),
                        );
                    }
                    _ => {}
                }
            }
            _ => {}
        },
        _ => (),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_menu() {
        let mut app = App::new();

        app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));
        handler(Key::Char('?'), &mut app);
        let current_route = app.get_current_route();

        assert_eq!(current_route.active_block, ActiveBlock::HelpMenu);
    }

    #[test]
    fn go_to_search_input() {
        let mut app = App::new();

        app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));
        handler(Key::Char('/'), &mut app);
        let current_route = app.get_current_route();

        assert_eq!(current_route.active_block, ActiveBlock::Input);
        assert_eq!(current_route.hovered_block, ActiveBlock::Input);
    }

    #[test]
    fn on_enter() {
        let mut app = App::new();

        app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));

        handler(Key::Char('\n'), &mut app);
        let current_route = app.get_current_route();

        assert_eq!(current_route.active_block, ActiveBlock::Library);
        assert_eq!(current_route.hovered_block, ActiveBlock::Library);
    }

    #[test]
    fn on_down_press() {
        let mut app = App::new();

        app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));

        handler(Key::Down, &mut app);
        let current_route = app.get_current_route();

        assert_eq!(current_route.active_block, ActiveBlock::Empty);
        assert_eq!(current_route.hovered_block, ActiveBlock::MyPlaylists);

        // TODO: test the other cases when they are implemented
    }

    #[test]
    fn on_up_press() {
        let mut app = App::new();

        app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::MyPlaylists));

        handler(Key::Up, &mut app);
        let current_route = app.get_current_route();

        assert_eq!(current_route.active_block, ActiveBlock::Empty);
        assert_eq!(current_route.hovered_block, ActiveBlock::Library);
    }

    #[test]
    fn on_left_press() {
        let mut app = App::new();
        app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Album));

        handler(Key::Left, &mut app);
        let current_route = app.get_current_route();
        assert_eq!(current_route.active_block, ActiveBlock::Empty);
        assert_eq!(current_route.hovered_block, ActiveBlock::Library);

        app.set_current_route_state(None, Some(ActiveBlock::Home));
        handler(Key::Left, &mut app);
        let current_route = app.get_current_route();
        assert_eq!(current_route.hovered_block, ActiveBlock::Library);

        app.set_current_route_state(None, Some(ActiveBlock::SongTable));
        handler(Key::Left, &mut app);
        let current_route = app.get_current_route();
        assert_eq!(current_route.hovered_block, ActiveBlock::Library);
    }

    #[test]
    fn on_right_press() {
        let mut app = App::new();

        app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));
        app.push_navigation_stack(RouteId::Album, ActiveBlock::Album);
        handler(Key::Right, &mut app);
        let current_route = app.get_current_route();

        assert_eq!(current_route.active_block, ActiveBlock::Album);
        assert_eq!(current_route.hovered_block, ActiveBlock::Album);

        app.push_navigation_stack(RouteId::Search, ActiveBlock::Empty);
        app.set_current_route_state(None, Some(ActiveBlock::MyPlaylists));
        handler(Key::Right, &mut app);
        let current_route = app.get_current_route();

        assert_eq!(current_route.active_block, ActiveBlock::SearchResultBlock);
        assert_eq!(current_route.hovered_block, ActiveBlock::SearchResultBlock);

        app.set_current_route_state(None, Some(ActiveBlock::Library));
        app.push_navigation_stack(RouteId::SongTable, ActiveBlock::SongTable);
        handler(Key::Right, &mut app);
        let current_route = app.get_current_route();

        assert_eq!(current_route.active_block, ActiveBlock::SongTable);
        assert_eq!(current_route.hovered_block, ActiveBlock::SongTable);

        app.set_current_route_state(None, Some(ActiveBlock::Library));
        app.push_navigation_stack(RouteId::SongTable, ActiveBlock::SongTable);
        handler(Key::Right, &mut app);
        let current_route = app.get_current_route();
        assert_eq!(current_route.active_block, ActiveBlock::SongTable);
        assert_eq!(current_route.hovered_block, ActiveBlock::SongTable);

        app.push_navigation_stack(RouteId::Home, ActiveBlock::Home);
        app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));
        handler(Key::Right, &mut app);
        let current_route = app.get_current_route();
        assert_eq!(current_route.active_block, ActiveBlock::Home);
        assert_eq!(current_route.hovered_block, ActiveBlock::Home);
    }
}
