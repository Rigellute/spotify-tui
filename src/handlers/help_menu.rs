use crate::{app::App, event::Key};

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Ctrl('d') => {
            app.help_menu_page += 1;
            app.calculate_help_menu_offset();
        }
        Key::Ctrl('u') => {
            if app.help_menu_page > 0 {
                app.help_menu_page -= 1;
                app.calculate_help_menu_offset();
            }
        }
        _ => {}
    };
}
