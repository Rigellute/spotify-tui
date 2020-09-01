use super::{super::app::App, common_key_events};
use crate::{app::RecommendationsContext, event::Key, network::IoEvent};

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
    k if common_key_events::down_event(k) => {
      if let Some(recently_played_result) = &app.recently_played.result {
        let next_index = common_key_events::on_down_press_handler(
          &recently_played_result.items,
          Some(app.recently_played.index),
        );
        app.recently_played.index = next_index;
      }
    }
    k if common_key_events::up_event(k) => {
      if let Some(recently_played_result) = &app.recently_played.result {
        let next_index = common_key_events::on_up_press_handler(
          &recently_played_result.items,
          Some(app.recently_played.index),
        );
        app.recently_played.index = next_index;
      }
    }
    k if common_key_events::high_event(k) => {
      if let Some(_recently_played_result) = &app.recently_played.result {
        let next_index = common_key_events::on_high_press_handler();
        app.recently_played.index = next_index;
      }
    }
    k if common_key_events::middle_event(k) => {
      if let Some(recently_played_result) = &app.recently_played.result {
        let next_index = common_key_events::on_middle_press_handler(&recently_played_result.items);
        app.recently_played.index = next_index;
      }
    }
    k if common_key_events::low_event(k) => {
      if let Some(recently_played_result) = &app.recently_played.result {
        let next_index = common_key_events::on_low_press_handler(&recently_played_result.items);
        app.recently_played.index = next_index;
      }
    }
    Key::Char('s') => {
      if let Some(recently_played_result) = &app.recently_played.result.clone() {
        if let Some(selected_track) = recently_played_result.items.get(app.recently_played.index) {
          if let Some(track_id) = &selected_track.track.id {
            app.dispatch(IoEvent::ToggleSaveTrack(track_id.to_string()));
          };
        };
      };
    }
    Key::Enter => {
      if let Some(recently_played_result) = &app.recently_played.result.clone() {
        let track_uris: Vec<String> = recently_played_result
          .items
          .iter()
          .map(|item| item.track.uri.to_owned())
          .collect();

        app.dispatch(IoEvent::StartPlayback(
          None,
          Some(track_uris),
          Some(app.recently_played.index),
        ));
      };
    }
    Key::Char('r') => {
      if let Some(recently_played_result) = &app.recently_played.result.clone() {
        let selected_track_history_item =
          recently_played_result.items.get(app.recently_played.index);

        if let Some(item) = selected_track_history_item {
          if let Some(id) = &item.track.id {
            app.recommendations_context = Some(RecommendationsContext::Song);
            app.recommendations_seed = item.track.name.clone();
            app.get_recommendations_for_track_id(id.to_string());
          }
        }
      }
    }
    _ if key == app.user_config.keys.add_item_to_queue => {
      if let Some(recently_played_result) = &app.recently_played.result.clone() {
        if let Some(history) = recently_played_result.items.get(app.recently_played.index) {
          app.dispatch(IoEvent::AddItemToQueue(history.track.uri.clone()))
        }
      };
    }
    _ => {}
  };
}

#[cfg(test)]
mod tests {
  use super::{super::super::app::ActiveBlock, *};

  #[test]
  fn on_left_press() {
    let mut app = App::default();
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
  fn on_esc() {
    let mut app = App::default();

    handler(Key::Esc, &mut app);

    let current_route = app.get_current_route();
    assert_eq!(current_route.active_block, ActiveBlock::Empty);
  }
}
