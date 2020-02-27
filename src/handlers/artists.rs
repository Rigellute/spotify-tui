use super::common_key_events;
use crate::{
    app::{ActiveBlock, App, RecommendationsContext, RouteId},
    event::Key,
};

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
        k if common_key_events::down_event(k) => {
            if let Some(artists) = &mut app.library.saved_artists.get_results(None) {
                let next_index = common_key_events::on_down_press_handler(
                    &artists.items,
                    Some(app.artists_list_index),
                );
                app.artists_list_index = next_index;
            }
        }
        k if common_key_events::up_event(k) => {
            if let Some(artists) = &mut app.library.saved_artists.get_results(None) {
                let next_index = common_key_events::on_up_press_handler(
                    &artists.items,
                    Some(app.artists_list_index),
                );
                app.artists_list_index = next_index;
            }
        }
        k if common_key_events::high_event(k) => {
            if let Some(_artists) = &mut app.library.saved_artists.get_results(None) {
                let next_index = common_key_events::on_high_press_handler();
                app.artists_list_index = next_index;
            }
        }
        k if common_key_events::middle_event(k) => {
            if let Some(artists) = &mut app.library.saved_artists.get_results(None) {
                let next_index = common_key_events::on_middle_press_handler(&artists.items);
                app.artists_list_index = next_index;
            }
        }
        k if common_key_events::low_event(k) => {
            if let Some(artists) = &mut app.library.saved_artists.get_results(None) {
                let next_index = common_key_events::on_low_press_handler(&artists.items);
                app.artists_list_index = next_index;
            }
        }
        Key::Enter => {
            let artists = app.artists.to_owned();
            let artist = &artists[app.artists_list_index];
            app.get_artist(&artist.id, &artist.name);
            app.push_navigation_stack(RouteId::Artist, ActiveBlock::ArtistBlock);
        }
        Key::Char('D') => app.user_unfollow_artists(),
        Key::Char('e') => {
            let artists = app.artists.to_owned();
            let artist = artists.get(app.artists_list_index);
            if let Some(artist) = artist {
                app.start_playback(Some(artist.uri.to_owned()), None, None);
            }
        }
        Key::Char('r') => {
            let artists = app.artists.to_owned();
            let artist = artists.get(app.artists_list_index);
            if let Some(artist) = artist {
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
