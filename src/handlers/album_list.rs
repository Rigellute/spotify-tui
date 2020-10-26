use super::common_key_events;
use crate::{
  app::{ActiveBlock, AlbumTableContext, App, RouteId, SelectedFullAlbum},
  event::Key,
};

pub fn handler(key: Key, app: &mut App) {
  match key {
    key if common_key_events::is_list_navigation_key_event(key, app) => {
      app.library.saved_albums.selected_index = app
        .library
        .saved_albums
        .handle_list_navigation_event(key, app);
    }
    Key::Enter => {
      if let Some(selected_album) = app.library.saved_albums.get_selected_item() {
        app.selected_album_full = Some(SelectedFullAlbum {
          album: selected_album.album.clone(),
          selected_index: 0,
        });
        app.album_table_context = AlbumTableContext::Full;
        app.push_navigation_stack(RouteId::AlbumTracks, ActiveBlock::AlbumTracks);
      };
    }
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
