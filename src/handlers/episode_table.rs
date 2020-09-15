use super::{
  super::app::{App, RecommendationsContext, EpisodeTable, EpisodeTableContext},
  common_key_events,
};
use crate::event::Key;
use crate::network::IoEvent;
use rand::{thread_rng, Rng};
use serde_json::from_value;

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
    Key::Ctrl('d') => {
      match &app.episode_table.context {
        Some(context) => match context {
          EpisodeTableContext::ShowSearch => {},
          EpisodeTableContext::MyShows => {},
        },
        None => {}
      };
    }
    // Scroll up
    Key::Ctrl('u') => {
      match &app.episode_table.context {
        Some(context) => match context {
          EpisodeTableContext::ShowSearch => {},
          EpisodeTableContext::MyShows => {},
        },
        None => {}
      };
    }
    Key::Char('s') => {}, // TODO: handle saving the show
    Key::Ctrl('e') => jump_to_end(app),
    Key::Ctrl('a') => jump_to_start(app),
    _ => {}
  }
}

fn jump_to_end(app: &mut App) {
  match &app.episode_table.context {
      EpisodeTableContext::ShowSearch => {}
      EpisodeTableContext::MyShows => {}
    },
    None => {}
  }
}

fn on_enter(app: &mut App) {
  let EpisodeTable {
    context,
    selected_index,
    episodes,
  } = &app.episode_table;
  match &context {
    Some(context) => match context {
      EpisodeTableContext::ShowSearch => {}
      EpisodeTableContext::MyShows => {}
    None => {}
  };
}

fn jump_to_start(app: &mut App) {
  match &app.episode_table.context {
    Some(context) => match context {
      EpisodeTableContext::ShowSearch => {}
      EpisodeTableContext::MyShows => {}
    },
    None => {}
  }
}
