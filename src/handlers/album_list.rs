use super::common_key_events;
use crate::{
  app::{ActiveBlock, AlbumTableContext, App, RouteId, SelectedFullAlbum},
  event::Key,
};

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
    k if common_key_events::down_event(k) => {
      if let Some(albums) = &mut app.library.saved_albums.get_results(None) {
        let next_index =
          common_key_events::on_down_press_handler(&albums.items, Some(app.album_list_index));
        app.album_list_index = next_index;
      }
    }
    k if common_key_events::up_event(k) => {
      if let Some(albums) = &mut app.library.saved_albums.get_results(None) {
        let next_index =
          common_key_events::on_up_press_handler(&albums.items, Some(app.album_list_index));
        app.album_list_index = next_index;
      }
    }
    k if common_key_events::high_event(k) => {
      if let Some(_albums) = app.library.saved_albums.get_results(None) {
        let next_index = common_key_events::on_high_press_handler();
        app.album_list_index = next_index;
      }
    }
    k if common_key_events::middle_event(k) => {
      if let Some(albums) = app.library.saved_albums.get_results(None) {
        let next_index = common_key_events::on_middle_press_handler(&albums.items);
        app.album_list_index = next_index;
      }
    }
    k if common_key_events::low_event(k) => {
      if let Some(albums) = app.library.saved_albums.get_results(None) {
        let next_index = common_key_events::on_low_press_handler(&albums.items);
        app.album_list_index = next_index;
      }
    }
    Key::Enter => {
      if let Some(albums) = app.library.saved_albums.get_results(None) {
        if let Some(selected_album) = albums.items.get(app.album_list_index) {
          app.selected_album_full = Some(SelectedFullAlbum {
            album: selected_album.album.clone(),
            selected_index: 0,
          });
          app.album_table_context = AlbumTableContext::Full;
          app.push_navigation_stack(RouteId::AlbumTracks, ActiveBlock::AlbumTracks);
        };
      }
    }
    k if k == app.user_config.keys.next_page => app.get_current_user_saved_albums_next(),
    k if k == app.user_config.keys.previous_page => app.get_current_user_saved_albums_previous(),
    Key::Char('D') => app.current_user_saved_album_delete(ActiveBlock::AlbumList),
    _ => {}
  };
}

#[cfg(test)]
mod tests {
  use super::*;

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
