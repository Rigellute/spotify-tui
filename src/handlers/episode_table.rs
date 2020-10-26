use super::{
  super::app::{App, EpisodeTable},
  common_key_events,
};
use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::is_list_navigation_key_event(k, app) => {
      app.episode_table.episodes.selected_index = app
        .episode_table
        .episodes
        .handle_list_navigation_event(k, app);
    }
    k if common_key_events::left_event(k) => {
      common_key_events::handle_left_event(app);
    }
    k if common_key_events::right_event(k) => {
      common_key_events::handle_right_event(app);
    }
    Key::Enter => {
      on_enter(app);
    }
    Key::Char('S') => toggle_sort_by_date(app),
    Key::Char('s') => {} // TODO: handle saving the show
    _ => {}
  }
}

fn on_enter(app: &mut App) {
  let EpisodeTable {
    show_id: _,
    episodes,
    reversed: _,
  } = &app.episode_table;
  let episode_uris = episodes
    .items
    .iter()
    .map(|episode| episode.uri.to_owned())
    .collect::<Vec<String>>();
  app.dispatch(IoEvent::StartPlayback(
    None,
    Some(episode_uris),
    Some(app.episode_table.episodes.selected_index),
  ));
}

fn toggle_sort_by_date(app: &mut App) {
  let selected_id = app
    .episode_table
    .episodes
    .items
    .get(app.episode_table.episodes.selected_index)
    .map(|e| e.id.clone());
  //app.episode_table.episodes.items.reverse();
  //app.episode_table.reversed ^= true;
  if let Some(id) = selected_id {
    app.episode_table.episodes.selected_index = app
      .episode_table
      .episodes
      .items
      .iter()
      .position(|e| e.id == id)
      .unwrap_or(0);
  } else {
    app.episode_table.episodes.selected_index = 0;
  }
}
