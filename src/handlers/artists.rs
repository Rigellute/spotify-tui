use super::common_key_events;
use crate::{
  app::{ActiveBlock, App, RecommendationsContext, RouteId},
  event::Key,
  network::IoEvent,
};

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::is_list_navigation_key_event(k, app) => {
      app.library.saved_artists.selected_index = app
        .library
        .saved_artists
        .handle_list_navigation_event(k, app);
    }
    Key::Enter => {
      if let Some(artist) = app.library.saved_artists.get_selected_item() {
        app.get_artist(artist.id.clone(), artist.name.clone());
        app.push_navigation_stack(RouteId::Artist, ActiveBlock::ArtistBlock);
      }
    }
    Key::Char('D') => app.user_unfollow_artists(ActiveBlock::AlbumList),
    Key::Char('e') => {
      if let Some(artist) = app.library.saved_artists.get_selected_item() {
        app.dispatch(IoEvent::StartPlayback(
          Some(artist.uri.to_owned()),
          None,
          None,
        ));
      }
    }
    Key::Char('r') => {
      if let Some(artist) = app.library.saved_artists.get_selected_item() {
        let artist_name = artist.name.clone();
        let artist_id_list: Option<Vec<String>> = Some(vec![artist.id.clone()]);

        app.recommendations_context = Some(RecommendationsContext::Artist);
        app.recommendations_seed = artist_name;
        app.get_recommendations_for_seed(artist_id_list, None, None);
      }
    }
    _ => {}
  }
}
