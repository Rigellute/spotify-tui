use super::common_key_events;
use crate::{
  app::{ActiveBlock, App},
  event::Key,
  network::IoEvent,
};

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
    k if common_key_events::down_event(k) => {
      if let Some(shows) = &mut app.library.saved_shows.get_results(None) {
        let next_index =
          common_key_events::on_down_press_handler(&shows.items, Some(app.shows_list_index));
        app.shows_list_index = next_index;
      }
    }
    k if common_key_events::up_event(k) => {
      if let Some(shows) = &mut app.library.saved_shows.get_results(None) {
        let next_index =
          common_key_events::on_up_press_handler(&shows.items, Some(app.shows_list_index));
        app.shows_list_index = next_index;
      }
    }
    k if common_key_events::high_event(k) => {
      if let Some(_shows) = app.library.saved_shows.get_results(None) {
        let next_index = common_key_events::on_high_press_handler();
        app.shows_list_index = next_index;
      }
    }
    k if common_key_events::middle_event(k) => {
      if let Some(shows) = app.library.saved_shows.get_results(None) {
        let next_index = common_key_events::on_middle_press_handler(&shows.items);
        app.shows_list_index = next_index;
      }
    }
    k if common_key_events::low_event(k) => {
      if let Some(shows) = app.library.saved_shows.get_results(None) {
        let next_index = common_key_events::on_low_press_handler(&shows.items);
        app.shows_list_index = next_index;
      }
    }
    Key::Enter => {
      if let Some(shows) = app.library.saved_shows.get_results(None) {
        if let Some(selected_show) = shows.items.get(app.shows_list_index).cloned() {
          app.dispatch(IoEvent::GetShowEpisodes(Box::new(selected_show.show)));
        };
      }
    }
    k if k == app.user_config.keys.next_page => app.get_current_user_saved_shows_next(),
    k if k == app.user_config.keys.previous_page => app.get_current_user_saved_shows_previous(),
    Key::Char('D') => app.user_unfollow_show(ActiveBlock::Podcasts),
    _ => {}
  }
}
