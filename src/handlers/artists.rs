use super::super::app::App;
use super::common_key_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
        k if common_key_events::down_event(k) => {
            if let Some(artists) = &mut app.library.saved_artists.get_results(None) {
                let next_index = common_key_events::on_down_press_handler(
                    &artists.items,
                    Some(app.artists_list_index),
                );
                app.artists_list_index = next_index;
            }
        }
        k if common_key_events::up_event(k) => {
            if let Some(artists) = &mut app.library.saved_artists.get_results(None) {
                let next_index = common_key_events::on_up_press_handler(
                    &artists.items,
                    Some(app.artists_list_index),
                );
                app.artists_list_index = next_index;
            }
        }
        Key::Char('\n') => {
            let artists = app.artists.to_owned();
            let artist = &artists[app.artists_list_index];
            app.get_artist_albums(&artist.id.to_owned(), &artist.name.to_owned());
        }
        Key::Ctrl('g') => {
            common_key_events::go_first_line(app);
        }
        Key::Char('G') => {
            common_key_events::go_last_line(app);
        }
        _ => {}
    }
}
