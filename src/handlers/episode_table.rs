use super::{
  super::app::{App, EpisodeTableContext},
  common_key_events,
};
use crate::app::ActiveBlock;
use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
    k if common_key_events::down_event(k) => {
      if let Some(episodes) = &mut app.library.show_episodes.get_results(None) {
        let next_index =
          common_key_events::on_down_press_handler(&episodes.items, Some(app.episode_list_index));
        app.episode_list_index = next_index;
      }
    }
    k if common_key_events::up_event(k) => {
      if let Some(episodes) = &mut app.library.show_episodes.get_results(None) {
        let next_index =
          common_key_events::on_up_press_handler(&episodes.items, Some(app.episode_list_index));
        app.episode_list_index = next_index;
      }
    }
    k if common_key_events::high_event(k) => {
      if let Some(_episodes) = app.library.show_episodes.get_results(None) {
        let next_index = common_key_events::on_high_press_handler();
        app.episode_list_index = next_index;
      }
    }
    k if common_key_events::middle_event(k) => {
      if let Some(episodes) = app.library.show_episodes.get_results(None) {
        let next_index = common_key_events::on_middle_press_handler(&episodes.items);
        app.episode_list_index = next_index;
      }
    }
    k if common_key_events::low_event(k) => {
      if let Some(episodes) = app.library.show_episodes.get_results(None) {
        let next_index = common_key_events::on_low_press_handler(&episodes.items);
        app.episode_list_index = next_index;
      }
    }
    Key::Enter => {
      on_enter(app);
    }
    // Scroll down
    k if k == app.user_config.keys.next_page => handle_next_event(app),
    // Scroll up
    k if k == app.user_config.keys.previous_page => handle_prev_event(app),
    Key::Char('S') => toggle_sort_by_date(app),
    Key::Char('s') => handle_follow_event(app),
    Key::Char('D') => handle_unfollow_event(app),
    Key::Ctrl('e') => jump_to_end(app),
    Key::Ctrl('a') => jump_to_start(app),
    _ => {}
  }
}

fn jump_to_end(app: &mut App) {
  if let Some(episodes) = app.library.show_episodes.get_results(None) {
    let last_idx = episodes.items.len() - 1;
    app.episode_list_index = last_idx;
  }
}

fn on_enter(app: &mut App) {
  if let Some(episodes) = app.library.show_episodes.get_results(None) {
    let episode_uris = episodes
      .items
      .iter()
      .map(|episode| episode.uri.to_owned())
      .collect::<Vec<String>>();
    app.dispatch(IoEvent::StartPlayback(
      None,
      Some(episode_uris),
      Some(app.episode_list_index),
    ));
  }
}

fn handle_prev_event(app: &mut App) {
  app.get_episode_table_previous();
}

fn handle_next_event(app: &mut App) {
  match app.episode_table_context {
    EpisodeTableContext::Full => {
      if let Some(selected_episode) = app.selected_show_full.clone() {
        let show_id = selected_episode.show.id;
        app.get_episode_table_next(show_id)
      }
    }
    EpisodeTableContext::Simplified => {
      if let Some(selected_episode) = app.selected_show_simplified.clone() {
        let show_id = selected_episode.show.id;
        app.get_episode_table_next(show_id)
      }
    }
  }
}

fn handle_follow_event(app: &mut App) {
  app.user_follow_show(ActiveBlock::EpisodeTable);
}

fn handle_unfollow_event(app: &mut App) {
  app.user_unfollow_show(ActiveBlock::EpisodeTable);
}

fn jump_to_start(app: &mut App) {
  app.episode_list_index = 0;
}

fn toggle_sort_by_date(app: &mut App) {
  //TODO: reverse whole list and not just currently visible episodes
  let selected_id = match app.library.show_episodes.get_results(None) {
    Some(episodes) => episodes
      .items
      .get(app.episode_list_index)
      .map(|e| e.id.clone()),
    None => None,
  };

  if let Some(episodes) = app.library.show_episodes.get_mut_results(None) {
    episodes.items.reverse();
  }

  if let Some(id) = selected_id {
    if let Some(episodes) = app.library.show_episodes.get_results(None) {
      app.episode_list_index = episodes.items.iter().position(|e| e.id == id).unwrap_or(0);
    }
  } else {
    app.episode_list_index = 0;
  }
}
