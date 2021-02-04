use super::{
  super::app::{ActiveBlock, App, RouteId, LIBRARY_OPTIONS},
  common_key_events,
};
use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
    k if common_key_events::down_event(k) => {
      let next_index = common_key_events::on_down_press_handler(
        &LIBRARY_OPTIONS,
        Some(app.library.selected_index),
      );
      app.library.selected_index = next_index;
    }
    k if common_key_events::up_event(k) => {
      let next_index =
        common_key_events::on_up_press_handler(&LIBRARY_OPTIONS, Some(app.library.selected_index));
      app.library.selected_index = next_index;
    }
    k if common_key_events::high_event(k) => {
      let next_index = common_key_events::on_high_press_handler();
      app.library.selected_index = next_index;
    }
    k if common_key_events::middle_event(k) => {
      let next_index = common_key_events::on_middle_press_handler(&LIBRARY_OPTIONS);
      app.library.selected_index = next_index;
    }
    k if common_key_events::low_event(k) => {
      let next_index = common_key_events::on_low_press_handler(&LIBRARY_OPTIONS);
      app.library.selected_index = next_index
    }
    // `library` should probably be an array of structs with enums rather than just using indexes
    // like this
    Key::Enter => match app.library.selected_index {
      // Made For You,
      0 => {
        app.get_made_for_you();
        app.push_navigation_stack(RouteId::MadeForYou, ActiveBlock::MadeForYou);
      }
      // Recently Played,
      1 => {
        app.dispatch(IoEvent::GetRecentlyPlayed);
        app.push_navigation_stack(RouteId::RecentlyPlayed, ActiveBlock::RecentlyPlayed);
      }
      // Liked Songs,
      2 => {
        app.dispatch(IoEvent::GetCurrentSavedTracks(None));
        app.push_navigation_stack(RouteId::TrackTable, ActiveBlock::TrackTable);
      }
      // Albums,
      3 => {
        app.dispatch(IoEvent::GetCurrentUserSavedAlbums(None));
        app.push_navigation_stack(RouteId::AlbumList, ActiveBlock::AlbumList);
      }
      //  Artists,
      4 => {
        app.dispatch(IoEvent::GetFollowedArtists(None));
        app.push_navigation_stack(RouteId::Artists, ActiveBlock::Artists);
      }
      // Podcasts,
      5 => {
        app.dispatch(IoEvent::GetCurrentUserSavedShows(None));
        app.push_navigation_stack(RouteId::Podcasts, ActiveBlock::Podcasts);
      }
      // This is required because Rust can't tell if this pattern in exhaustive
      _ => {}
    },
    _ => (),
  };
}
