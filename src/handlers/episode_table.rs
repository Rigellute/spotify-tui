use super::{
  super::app::{App, EpisodeTable},
  common_key_events,
};
use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
    k if common_key_events::down_event(k) => {
      let next_index = common_key_events::on_down_press_handler(
        &app.episode_table.episodes,
        Some(app.episode_table.selected_index),
      );
      app.episode_table.selected_index = next_index;
    }
    k if common_key_events::up_event(k) => {
      let next_index = common_key_events::on_up_press_handler(
        &app.episode_table.episodes,
        Some(app.episode_table.selected_index),
      );
      app.episode_table.selected_index = next_index;
    }
    k if common_key_events::high_event(k) => {
      let next_index = common_key_events::on_high_press_handler();
      app.episode_table.selected_index = next_index;
    }
    k if common_key_events::middle_event(k) => {
      let next_index = common_key_events::on_middle_press_handler(&app.episode_table.episodes);
      app.episode_table.selected_index = next_index;
    }
    k if common_key_events::low_event(k) => {
      let next_index = common_key_events::on_low_press_handler(&app.episode_table.episodes);
      app.episode_table.selected_index = next_index;
    }
    Key::Enter => {
      on_enter(app);
    }
    // Scroll down
    Key::Ctrl('d') => {}
    // Scroll up
    Key::Ctrl('u') => {}
    Key::Char('s') => {} // TODO: handle saving the show
    Key::Ctrl('e') => jump_to_end(app),
    Key::Ctrl('a') => jump_to_start(app),
    _ => {}
  }
}

fn jump_to_end(app: &mut App) {
    let last_idx = &app.episode_table.episodes.len() - 1;
    app.episode_table.selected_index = last_idx;
}

fn on_enter(app: &mut App) {
  let EpisodeTable {
    selected_index: _,
    episodes,
  } = &app.episode_table;
  let episode_uris = episodes
    .iter()
    .map(|episode| episode.uri.to_owned())
    .collect::<Vec<String>>();
  app.dispatch(IoEvent::StartPlayback(
    None,
    Some(episode_uris),
    Some(app.episode_table.selected_index),
  ));
}

fn jump_to_start(app: &mut App) {
    app.episode_table.selected_index = 0;
}
