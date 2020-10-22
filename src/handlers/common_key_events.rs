use super::super::app::{ActiveBlock, App, RouteId};
use crate::event::Key;
pub fn down_event(key: Key) -> bool {
  match key {
    Key::Down | Key::Char('j') | Key::Ctrl('n') => true,
    _ => false,
  }
}

pub fn up_event(key: Key) -> bool {
  match key {
    Key::Up | Key::Char('k') | Key::Ctrl('p') => true,
    _ => false,
  }
}

pub fn left_event(key: Key) -> bool {
  match key {
    Key::Left | Key::Char('h') | Key::Ctrl('b') => true,
    _ => false,
  }
}

pub fn right_event(key: Key) -> bool {
  match key {
    Key::Right | Key::Char('l') | Key::Ctrl('f') => true,
    _ => false,
  }
}

pub fn high_event(key: Key) -> bool {
  match key {
    Key::Char('H') => true,
    _ => false,
  }
}

pub fn middle_event(key: Key) -> bool {
  match key {
    Key::Char('M') => true,
    _ => false,
  }
}

pub fn low_event(key: Key) -> bool {
  match key {
    Key::Char('L') => true,
    _ => false,
  }
}

pub fn on_down_press_handler<T>(selection_data: &[T], selection_index: Option<usize>) -> usize {
  if let Some(selection_index) = selection_index {
    if selection_data.is_empty() {
      0
    } else {
      (selection_index + 1) % selection_data.len()
    }
  } else {
    0
  }
}

pub fn on_up_press_handler<T>(selection_data: &[T], selection_index: Option<usize>) -> usize {
  match selection_index {
    Some(selection_index) => {
      if !selection_data.is_empty() {
        if selection_index > 0 {
          return selection_index - 1;
        } else {
          return selection_data.len() - 1;
        }
      }
      0
    }
    None => 0,
  }
}

pub fn on_high_press_handler() -> usize {
  0
}

pub fn on_middle_press_handler<T>(selection_data: &[T]) -> usize {
  let mut index = selection_data.len() / 2;
  if selection_data.len() % 2 == 0 {
    index -= 1;
  }
  index
}

pub fn on_low_press_handler<T>(selection_data: &[T]) -> usize {
  selection_data.len() - 1
}

pub fn handle_right_event(app: &mut App) {
  match app.get_current_route().hovered_block {
    ActiveBlock::MyPlaylists | ActiveBlock::Library => match app.get_current_route().id {
      RouteId::AlbumTracks => {
        app.set_current_route_state(
          Some(ActiveBlock::AlbumTracks),
          Some(ActiveBlock::AlbumTracks),
        );
      }
      RouteId::TrackTable => {
        app.set_current_route_state(Some(ActiveBlock::TrackTable), Some(ActiveBlock::TrackTable));
      }
      RouteId::Podcasts => {
        app.set_current_route_state(Some(ActiveBlock::Podcasts), Some(ActiveBlock::Podcasts));
      }
      RouteId::Recommendations => {
        app.set_current_route_state(Some(ActiveBlock::TrackTable), Some(ActiveBlock::TrackTable));
      }
      RouteId::AlbumList => {
        app.set_current_route_state(Some(ActiveBlock::AlbumList), Some(ActiveBlock::AlbumList));
      }
      RouteId::PodcastEpisodes => {
        app.set_current_route_state(
          Some(ActiveBlock::EpisodeTable),
          Some(ActiveBlock::EpisodeTable),
        );
      }
      RouteId::MadeForYou => {
        app.set_current_route_state(Some(ActiveBlock::MadeForYou), Some(ActiveBlock::MadeForYou));
      }
      RouteId::Artists => {
        app.set_current_route_state(Some(ActiveBlock::Artists), Some(ActiveBlock::Artists));
      }
      RouteId::RecentlyPlayed => {
        app.set_current_route_state(
          Some(ActiveBlock::RecentlyPlayed),
          Some(ActiveBlock::RecentlyPlayed),
        );
      }
      RouteId::Search => {
        app.set_current_route_state(
          Some(ActiveBlock::SearchResultBlock),
          Some(ActiveBlock::SearchResultBlock),
        );
      }
      RouteId::Artist => app.set_current_route_state(
        Some(ActiveBlock::ArtistBlock),
        Some(ActiveBlock::ArtistBlock),
      ),
      RouteId::Home => {
        app.set_current_route_state(Some(ActiveBlock::Home), Some(ActiveBlock::Home));
      }
      RouteId::SelectedDevice => {}
      RouteId::Error => {}
      RouteId::Analysis => {}
      RouteId::BasicView => {}
    },
    _ => {}
  };
}

pub fn handle_left_event(app: &mut App) {
  // TODO: This should send you back to either library or playlist based on last selection
  app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));
}

pub fn is_common_key_event(key: Key) -> bool {
    down_event(key)
    || up_event(key)
    || high_event(key)
    || middle_event(key)
    || low_event(key)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_on_down_press_handler() {
    let data = vec!["Choice 1", "Choice 2", "Choice 3"];

    let index = 0;
    let next_index = on_down_press_handler(&data, Some(index));

    assert_eq!(next_index, 1);

    // Selection wrap if on last item
    let index = data.len() - 1;
    let next_index = on_down_press_handler(&data, Some(index));
    assert_eq!(next_index, 0);
  }

  #[test]
  fn test_on_up_press_handler() {
    let data = vec!["Choice 1", "Choice 2", "Choice 3"];

    let index = data.len() - 1;
    let next_index = on_up_press_handler(&data, Some(index));

    assert_eq!(next_index, index - 1);

    // Selection wrap if on first item
    let index = 0;
    let next_index = on_up_press_handler(&data, Some(index));
    assert_eq!(next_index, data.len() - 1);
  }
}
