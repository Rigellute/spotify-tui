use super::super::app::{ActiveBlock, App, Routes};
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
            app.active_block = ActiveBlock::HelpMenu;
        }
        Key::Char('/') => {
            app.active_block = ActiveBlock::Input;
            app.hovered_block = ActiveBlock::Input;
        }
        Key::Char('\n') => {
            app.active_block = app.hovered_block;
        }
        k if common_key_events::down_event(k) => match app.hovered_block {
            ActiveBlock::Library => {
                app.hovered_block = ActiveBlock::MyPlaylists;
            }
            ActiveBlock::MyPlaylists => {
                // Go to player
            }
            ActiveBlock::Album | ActiveBlock::Home | ActiveBlock::SongTable => {
                // Go to player
            }
            _ => {}
        },
        k if common_key_events::up_event(k) => match app.hovered_block {
            ActiveBlock::MyPlaylists => {
                app.hovered_block = ActiveBlock::Library;
            }
            _ => {}
        },
        k if common_key_events::left_event(k) => match app.hovered_block {
            ActiveBlock::Album | ActiveBlock::Home | ActiveBlock::SongTable => {
                app.hovered_block = ActiveBlock::Library;
            }
            _ => {}
        },
        k if common_key_events::right_event(k) => match app.hovered_block {
            ActiveBlock::MyPlaylists | ActiveBlock::Library => {
                match app.get_current_route() {
                    Some(current_route) => {
                        match current_route {
                            Routes::Album => {
                                app.active_block = ActiveBlock::Album;
                                app.hovered_block = ActiveBlock::Album;
                            }
                            Routes::SongTable => {
                                app.active_block = ActiveBlock::SongTable;
                                app.hovered_block = ActiveBlock::SongTable;
                            }
                            Routes::Search => {
                                app.active_block = ActiveBlock::SearchResultBlock;
                                app.hovered_block = ActiveBlock::SearchResultBlock;
                            }
                            Routes::Artist(_) => {
                                // TODO
                            }
                        }
                    }
                    None => {
                        app.hovered_block = ActiveBlock::Home;
                    }
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

        handler(Key::Char('?'), &mut app);

        assert_eq!(app.active_block, ActiveBlock::HelpMenu);
    }

    #[test]
    fn go_to_search_input() {
        let mut app = App::new();

        handler(Key::Char('/'), &mut app);

        assert_eq!(app.active_block, ActiveBlock::Input);
        assert_eq!(app.hovered_block, ActiveBlock::Input);
    }

    #[test]
    fn on_enter() {
        let mut app = App::new();

        app.active_block = ActiveBlock::Empty;
        app.hovered_block = ActiveBlock::Library;

        handler(Key::Char('\n'), &mut app);

        assert_eq!(app.active_block, ActiveBlock::Library);
        assert_eq!(app.hovered_block, ActiveBlock::Library);
    }

    #[test]
    fn on_down_press() {
        let mut app = App::new();

        app.active_block = ActiveBlock::Empty;
        app.hovered_block = ActiveBlock::Library;

        handler(Key::Down, &mut app);

        assert_eq!(app.active_block, ActiveBlock::Empty);
        assert_eq!(app.hovered_block, ActiveBlock::MyPlaylists);

        // TODO: test the other cases when they are implemented
    }

    #[test]
    fn on_up_press() {
        let mut app = App::new();

        app.active_block = ActiveBlock::Empty;
        app.hovered_block = ActiveBlock::MyPlaylists;

        handler(Key::Up, &mut app);

        assert_eq!(app.active_block, ActiveBlock::Empty);
        assert_eq!(app.hovered_block, ActiveBlock::Library);
    }

    #[test]
    fn on_left_press() {
        let mut app = App::new();
        app.active_block = ActiveBlock::Empty;
        app.hovered_block = ActiveBlock::Album;

        handler(Key::Left, &mut app);
        assert_eq!(app.active_block, ActiveBlock::Empty);
        assert_eq!(app.hovered_block, ActiveBlock::Library);

        app.hovered_block = ActiveBlock::Home;
        handler(Key::Left, &mut app);
        assert_eq!(app.hovered_block, ActiveBlock::Library);

        app.hovered_block = ActiveBlock::SongTable;
        handler(Key::Left, &mut app);
        assert_eq!(app.hovered_block, ActiveBlock::Library);
    }

    #[test]
    fn on_right_press() {
        let mut app = App::new();
        app.active_block = ActiveBlock::Empty;
        app.hovered_block = ActiveBlock::Library;

        app.navigation_stack.push(Routes::Album);
        handler(Key::Right, &mut app);
        assert_eq!(app.active_block, ActiveBlock::Album);
        assert_eq!(app.hovered_block, ActiveBlock::Album);

        app.hovered_block = ActiveBlock::MyPlaylists;
        app.navigation_stack.push(Routes::Search);
        handler(Key::Right, &mut app);
        assert_eq!(app.active_block, ActiveBlock::SearchResultBlock);
        assert_eq!(app.hovered_block, ActiveBlock::SearchResultBlock);

        app.hovered_block = ActiveBlock::Library;
        app.navigation_stack.push(Routes::SongTable);
        handler(Key::Right, &mut app);
        assert_eq!(app.active_block, ActiveBlock::SongTable);
        assert_eq!(app.hovered_block, ActiveBlock::SongTable);

        app.navigation_stack = vec![];
        app.active_block = ActiveBlock::Empty;
        app.hovered_block = ActiveBlock::Library;
        handler(Key::Right, &mut app);
        assert_eq!(app.active_block, ActiveBlock::Empty);
        assert_eq!(app.hovered_block, ActiveBlock::Home);
    }
}
