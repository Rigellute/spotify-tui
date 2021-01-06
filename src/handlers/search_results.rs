use super::{
  super::app::{
    ActiveBlock, App, DialogContext, RecommendationsContext, RouteId, SearchResultBlock,
    TrackTableContext,
  },
  common_key_events,
};
use crate::event::Key;
use crate::network::IoEvent;

fn handle_down_press_on_selected_block(app: &mut App) {
  // Start selecting within the selected block
  match app.search_results.selected_block {
    SearchResultBlock::AlbumSearch => {
      if let Some(result) = &app.search_results.albums {
        let next_index = common_key_events::on_down_press_handler(
          &result.items,
          app.search_results.selected_album_index,
        );
        app.search_results.selected_album_index = Some(next_index);
      }
    }
    SearchResultBlock::SongSearch => {
      if let Some(result) = &app.search_results.tracks {
        let next_index = common_key_events::on_down_press_handler(
          &result.items,
          app.search_results.selected_tracks_index,
        );
        app.search_results.selected_tracks_index = Some(next_index);
      }
    }
    SearchResultBlock::ArtistSearch => {
      if let Some(result) = &app.search_results.artists {
        let next_index = common_key_events::on_down_press_handler(
          &result.items,
          app.search_results.selected_artists_index,
        );
        app.search_results.selected_artists_index = Some(next_index);
      }
    }
    SearchResultBlock::PlaylistSearch => {
      if let Some(result) = &app.search_results.playlists {
        let next_index = common_key_events::on_down_press_handler(
          &result.items,
          app.search_results.selected_playlists_index,
        );
        app.search_results.selected_playlists_index = Some(next_index);
      }
    }
    SearchResultBlock::ShowSearch => {
      if let Some(result) = &app.search_results.shows {
        let next_index = common_key_events::on_down_press_handler(
          &result.items,
          app.search_results.selected_shows_index,
        );
        app.search_results.selected_shows_index = Some(next_index);
      }
    }
    SearchResultBlock::Empty => {}
  }
}

fn handle_down_press_on_hovered_block(app: &mut App) {
  match app.search_results.hovered_block {
    SearchResultBlock::AlbumSearch => {
      app.search_results.hovered_block = SearchResultBlock::ShowSearch;
    }
    SearchResultBlock::SongSearch => {
      app.search_results.hovered_block = SearchResultBlock::AlbumSearch;
    }
    SearchResultBlock::ArtistSearch => {
      app.search_results.hovered_block = SearchResultBlock::PlaylistSearch;
    }
    SearchResultBlock::PlaylistSearch => {
      app.search_results.hovered_block = SearchResultBlock::ShowSearch;
    }
    SearchResultBlock::ShowSearch => {
      app.search_results.hovered_block = SearchResultBlock::SongSearch;
    }
    SearchResultBlock::Empty => {}
  }
}

fn handle_up_press_on_selected_block(app: &mut App) {
  // Start selecting within the selected block
  match app.search_results.selected_block {
    SearchResultBlock::AlbumSearch => {
      if let Some(result) = &app.search_results.albums {
        let next_index = common_key_events::on_up_press_handler(
          &result.items,
          app.search_results.selected_album_index,
        );
        app.search_results.selected_album_index = Some(next_index);
      }
    }
    SearchResultBlock::SongSearch => {
      if let Some(result) = &app.search_results.tracks {
        let next_index = common_key_events::on_up_press_handler(
          &result.items,
          app.search_results.selected_tracks_index,
        );
        app.search_results.selected_tracks_index = Some(next_index);
      }
    }
    SearchResultBlock::ArtistSearch => {
      if let Some(result) = &app.search_results.artists {
        let next_index = common_key_events::on_up_press_handler(
          &result.items,
          app.search_results.selected_artists_index,
        );
        app.search_results.selected_artists_index = Some(next_index);
      }
    }
    SearchResultBlock::PlaylistSearch => {
      if let Some(result) = &app.search_results.playlists {
        let next_index = common_key_events::on_up_press_handler(
          &result.items,
          app.search_results.selected_playlists_index,
        );
        app.search_results.selected_playlists_index = Some(next_index);
      }
    }
    SearchResultBlock::ShowSearch => {
      if let Some(result) = &app.search_results.shows {
        let next_index = common_key_events::on_up_press_handler(
          &result.items,
          app.search_results.selected_shows_index,
        );
        app.search_results.selected_shows_index = Some(next_index);
      }
    }
    SearchResultBlock::Empty => {}
  }
}

fn handle_up_press_on_hovered_block(app: &mut App) {
  match app.search_results.hovered_block {
    SearchResultBlock::AlbumSearch => {
      app.search_results.hovered_block = SearchResultBlock::SongSearch;
    }
    SearchResultBlock::SongSearch => {
      app.search_results.hovered_block = SearchResultBlock::ShowSearch;
    }
    SearchResultBlock::ArtistSearch => {
      app.search_results.hovered_block = SearchResultBlock::ShowSearch;
    }
    SearchResultBlock::PlaylistSearch => {
      app.search_results.hovered_block = SearchResultBlock::ArtistSearch;
    }
    SearchResultBlock::ShowSearch => {
      app.search_results.hovered_block = SearchResultBlock::AlbumSearch;
    }
    SearchResultBlock::Empty => {}
  }
}

fn handle_high_press_on_selected_block(app: &mut App) {
  match app.search_results.selected_block {
    SearchResultBlock::AlbumSearch => {
      if let Some(_result) = &app.search_results.albums {
        let next_index = common_key_events::on_high_press_handler();
        app.search_results.selected_album_index = Some(next_index);
      }
    }
    SearchResultBlock::SongSearch => {
      if let Some(_result) = &app.search_results.tracks {
        let next_index = common_key_events::on_high_press_handler();
        app.search_results.selected_tracks_index = Some(next_index);
      }
    }
    SearchResultBlock::ArtistSearch => {
      if let Some(_result) = &app.search_results.artists {
        let next_index = common_key_events::on_high_press_handler();
        app.search_results.selected_artists_index = Some(next_index);
      }
    }
    SearchResultBlock::PlaylistSearch => {
      if let Some(_result) = &app.search_results.playlists {
        let next_index = common_key_events::on_high_press_handler();
        app.search_results.selected_playlists_index = Some(next_index);
      }
    }
    SearchResultBlock::ShowSearch => {
      if let Some(_result) = &app.search_results.shows {
        let next_index = common_key_events::on_high_press_handler();
        app.search_results.selected_shows_index = Some(next_index);
      }
    }
    SearchResultBlock::Empty => {}
  }
}

fn handle_middle_press_on_selected_block(app: &mut App) {
  match app.search_results.selected_block {
    SearchResultBlock::AlbumSearch => {
      if let Some(result) = &app.search_results.albums {
        let next_index = common_key_events::on_middle_press_handler(&result.items);
        app.search_results.selected_album_index = Some(next_index);
      }
    }
    SearchResultBlock::SongSearch => {
      if let Some(result) = &app.search_results.tracks {
        let next_index = common_key_events::on_middle_press_handler(&result.items);
        app.search_results.selected_tracks_index = Some(next_index);
      }
    }
    SearchResultBlock::ArtistSearch => {
      if let Some(result) = &app.search_results.artists {
        let next_index = common_key_events::on_middle_press_handler(&result.items);
        app.search_results.selected_artists_index = Some(next_index);
      }
    }
    SearchResultBlock::PlaylistSearch => {
      if let Some(result) = &app.search_results.playlists {
        let next_index = common_key_events::on_middle_press_handler(&result.items);
        app.search_results.selected_playlists_index = Some(next_index);
      }
    }
    SearchResultBlock::ShowSearch => {
      if let Some(result) = &app.search_results.shows {
        let next_index = common_key_events::on_middle_press_handler(&result.items);
        app.search_results.selected_shows_index = Some(next_index);
      }
    }
    SearchResultBlock::Empty => {}
  }
}

fn handle_low_press_on_selected_block(app: &mut App) {
  match app.search_results.selected_block {
    SearchResultBlock::AlbumSearch => {
      if let Some(result) = &app.search_results.albums {
        let next_index = common_key_events::on_low_press_handler(&result.items);
        app.search_results.selected_album_index = Some(next_index);
      }
    }
    SearchResultBlock::SongSearch => {
      if let Some(result) = &app.search_results.tracks {
        let next_index = common_key_events::on_low_press_handler(&result.items);
        app.search_results.selected_tracks_index = Some(next_index);
      }
    }
    SearchResultBlock::ArtistSearch => {
      if let Some(result) = &app.search_results.artists {
        let next_index = common_key_events::on_low_press_handler(&result.items);
        app.search_results.selected_artists_index = Some(next_index);
      }
    }
    SearchResultBlock::PlaylistSearch => {
      if let Some(result) = &app.search_results.playlists {
        let next_index = common_key_events::on_low_press_handler(&result.items);
        app.search_results.selected_playlists_index = Some(next_index);
      }
    }
    SearchResultBlock::ShowSearch => {
      if let Some(result) = &app.search_results.shows {
        let next_index = common_key_events::on_low_press_handler(&result.items);
        app.search_results.selected_shows_index = Some(next_index);
      }
    }
    SearchResultBlock::Empty => {}
  }
}

fn handle_add_item_to_queue(app: &mut App) {
  match &app.search_results.selected_block {
    SearchResultBlock::SongSearch => {
      if let (Some(index), Some(tracks)) = (
        app.search_results.selected_tracks_index,
        &app.search_results.tracks,
      ) {
        if let Some(track) = tracks.items.get(index) {
          let uri = track.uri.clone();
          app.dispatch(IoEvent::AddItemToQueue(uri));
        }
      }
    }
    SearchResultBlock::ArtistSearch => {}
    SearchResultBlock::PlaylistSearch => {}
    SearchResultBlock::AlbumSearch => {}
    SearchResultBlock::ShowSearch => {}
    SearchResultBlock::Empty => {}
  };
}

fn handle_enter_event_on_selected_block(app: &mut App) {
  match &app.search_results.selected_block {
    SearchResultBlock::AlbumSearch => {
      if let (Some(index), Some(albums_result)) = (
        &app.search_results.selected_album_index,
        &app.search_results.albums,
      ) {
        if let Some(album) = albums_result.items.get(index.to_owned()).cloned() {
          app.track_table.context = Some(TrackTableContext::AlbumSearch);
          app.dispatch(IoEvent::GetAlbumTracks(Box::new(album)));
        };
      }
    }
    SearchResultBlock::SongSearch => {
      let index = app.search_results.selected_tracks_index;
      let tracks = app.search_results.tracks.clone();
      let track_uris = tracks.map(|tracks| {
        tracks
          .items
          .into_iter()
          .map(|track| track.uri)
          .collect::<Vec<String>>()
      });
      app.dispatch(IoEvent::StartPlayback(None, track_uris, index));
    }
    SearchResultBlock::ArtistSearch => {
      if let Some(index) = &app.search_results.selected_artists_index {
        if let Some(result) = app.search_results.artists.clone() {
          if let Some(artist) = result.items.get(index.to_owned()) {
            app.get_artist(artist.id.clone(), artist.name.clone());
            app.push_navigation_stack(RouteId::Artist, ActiveBlock::ArtistBlock);
          };
        };
      };
    }
    SearchResultBlock::PlaylistSearch => {
      if let (Some(index), Some(playlists_result)) = (
        app.search_results.selected_playlists_index,
        &app.search_results.playlists,
      ) {
        if let Some(playlist) = playlists_result.items.get(index) {
          // Go to playlist tracks table
          app.track_table.context = Some(TrackTableContext::PlaylistSearch);
          let playlist_id = playlist.id.to_owned();
          app.dispatch(IoEvent::GetPlaylistTracks(playlist_id, app.playlist_offset));
        };
      }
    }
    SearchResultBlock::ShowSearch => {
      if let (Some(index), Some(shows_result)) = (
        app.search_results.selected_shows_index,
        &app.search_results.shows,
      ) {
        if let Some(show) = shows_result.items.get(index).cloned() {
          // Go to show tracks table
          app.dispatch(IoEvent::GetShowEpisodes(Box::new(show)));
        };
      }
    }
    SearchResultBlock::Empty => {}
  };
}

fn handle_enter_event_on_hovered_block(app: &mut App) {
  match app.search_results.hovered_block {
    SearchResultBlock::AlbumSearch => {
      let next_index = app.search_results.selected_album_index.unwrap_or(0);

      app.search_results.selected_album_index = Some(next_index);
      app.search_results.selected_block = SearchResultBlock::AlbumSearch;
    }
    SearchResultBlock::SongSearch => {
      let next_index = app.search_results.selected_tracks_index.unwrap_or(0);

      app.search_results.selected_tracks_index = Some(next_index);
      app.search_results.selected_block = SearchResultBlock::SongSearch;
    }
    SearchResultBlock::ArtistSearch => {
      let next_index = app.search_results.selected_artists_index.unwrap_or(0);

      app.search_results.selected_artists_index = Some(next_index);
      app.search_results.selected_block = SearchResultBlock::ArtistSearch;
    }
    SearchResultBlock::PlaylistSearch => {
      let next_index = app.search_results.selected_playlists_index.unwrap_or(0);

      app.search_results.selected_playlists_index = Some(next_index);
      app.search_results.selected_block = SearchResultBlock::PlaylistSearch;
    }
    SearchResultBlock::ShowSearch => {
      let next_index = app.search_results.selected_shows_index.unwrap_or(0);

      app.search_results.selected_shows_index = Some(next_index);
      app.search_results.selected_block = SearchResultBlock::ShowSearch;
    }
    SearchResultBlock::Empty => {}
  };
}

fn handle_recommended_tracks(app: &mut App) {
  match app.search_results.selected_block {
    SearchResultBlock::AlbumSearch => {}
    SearchResultBlock::SongSearch => {
      if let Some(index) = &app.search_results.selected_tracks_index {
        if let Some(result) = app.search_results.tracks.clone() {
          if let Some(track) = result.items.get(index.to_owned()) {
            let track_id_list: Option<Vec<String>> = match &track.id {
              Some(id) => Some(vec![id.to_string()]),
              None => None,
            };
            app.recommendations_context = Some(RecommendationsContext::Song);
            app.recommendations_seed = track.name.clone();
            app.get_recommendations_for_seed(None, track_id_list, Some(track.clone()));
          };
        };
      };
    }
    SearchResultBlock::ArtistSearch => {
      if let Some(index) = &app.search_results.selected_artists_index {
        if let Some(result) = app.search_results.artists.clone() {
          if let Some(artist) = result.items.get(index.to_owned()) {
            let artist_id_list: Option<Vec<String>> = Some(vec![artist.id.clone()]);
            app.recommendations_context = Some(RecommendationsContext::Artist);
            app.recommendations_seed = artist.name.clone();
            app.get_recommendations_for_seed(artist_id_list, None, None);
          };
        };
      };
    }
    SearchResultBlock::PlaylistSearch => {}
    SearchResultBlock::ShowSearch => {}
    SearchResultBlock::Empty => {}
  }
}

pub fn handler(key: Key, app: &mut App) {
  match key {
    Key::Esc => {
      app.search_results.selected_block = SearchResultBlock::Empty;
    }
    k if common_key_events::down_event(k) => {
      if app.search_results.selected_block != SearchResultBlock::Empty {
        handle_down_press_on_selected_block(app);
      } else {
        handle_down_press_on_hovered_block(app);
      }
    }
    k if common_key_events::up_event(k) => {
      if app.search_results.selected_block != SearchResultBlock::Empty {
        handle_up_press_on_selected_block(app);
      } else {
        handle_up_press_on_hovered_block(app);
      }
    }
    k if common_key_events::left_event(k) => {
      app.search_results.selected_block = SearchResultBlock::Empty;
      match app.search_results.hovered_block {
        SearchResultBlock::AlbumSearch => {
          common_key_events::handle_left_event(app);
        }
        SearchResultBlock::SongSearch => {
          common_key_events::handle_left_event(app);
        }
        SearchResultBlock::ArtistSearch => {
          app.search_results.hovered_block = SearchResultBlock::SongSearch;
        }
        SearchResultBlock::PlaylistSearch => {
          app.search_results.hovered_block = SearchResultBlock::AlbumSearch;
        }
        SearchResultBlock::ShowSearch => {
          common_key_events::handle_left_event(app);
        }
        SearchResultBlock::Empty => {}
      }
    }
    k if common_key_events::right_event(k) => {
      app.search_results.selected_block = SearchResultBlock::Empty;
      match app.search_results.hovered_block {
        SearchResultBlock::AlbumSearch => {
          app.search_results.hovered_block = SearchResultBlock::PlaylistSearch;
        }
        SearchResultBlock::SongSearch => {
          app.search_results.hovered_block = SearchResultBlock::ArtistSearch;
        }
        SearchResultBlock::ArtistSearch => {
          app.search_results.hovered_block = SearchResultBlock::SongSearch;
        }
        SearchResultBlock::PlaylistSearch => {
          app.search_results.hovered_block = SearchResultBlock::AlbumSearch;
        }
        SearchResultBlock::ShowSearch => {}
        SearchResultBlock::Empty => {}
      }
    }
    k if common_key_events::high_event(k) => {
      if app.search_results.selected_block != SearchResultBlock::Empty {
        handle_high_press_on_selected_block(app);
      }
    }
    k if common_key_events::middle_event(k) => {
      if app.search_results.selected_block != SearchResultBlock::Empty {
        handle_middle_press_on_selected_block(app);
      }
    }
    k if common_key_events::low_event(k) => {
      if app.search_results.selected_block != SearchResultBlock::Empty {
        handle_low_press_on_selected_block(app)
      }
    }
    // Handle pressing enter when block is selected to start playing track
    Key::Enter => match app.search_results.selected_block {
      SearchResultBlock::Empty => handle_enter_event_on_hovered_block(app),
      SearchResultBlock::PlaylistSearch => {
        app.playlist_offset = 0;
        handle_enter_event_on_selected_block(app);
      }
      _ => handle_enter_event_on_selected_block(app),
    },
    Key::Char('w') => match app.search_results.selected_block {
      SearchResultBlock::AlbumSearch => {
        app.current_user_saved_album_add(ActiveBlock::SearchResultBlock)
      }
      SearchResultBlock::SongSearch => {}
      SearchResultBlock::ArtistSearch => app.user_follow_artists(ActiveBlock::SearchResultBlock),
      SearchResultBlock::PlaylistSearch => {
        app.user_follow_playlist();
      }
      SearchResultBlock::ShowSearch => app.user_follow_show(ActiveBlock::SearchResultBlock),
      SearchResultBlock::Empty => {}
    },
    Key::Char('D') => match app.search_results.selected_block {
      SearchResultBlock::AlbumSearch => {
        app.current_user_saved_album_delete(ActiveBlock::SearchResultBlock)
      }
      SearchResultBlock::SongSearch => {}
      SearchResultBlock::ArtistSearch => app.user_unfollow_artists(ActiveBlock::SearchResultBlock),
      SearchResultBlock::PlaylistSearch => {
        if let (Some(playlists), Some(selected_index)) = (
          &app.search_results.playlists,
          app.search_results.selected_playlists_index,
        ) {
          let selected_playlist = &playlists.items[selected_index].name;
          app.dialog = Some(selected_playlist.clone());
          app.confirm = false;

          let route = app.get_current_route().id.clone();
          app.push_navigation_stack(route, ActiveBlock::Dialog(DialogContext::PlaylistSearch));
        }
      }
      SearchResultBlock::ShowSearch => app.user_unfollow_show(ActiveBlock::SearchResultBlock),
      SearchResultBlock::Empty => {}
    },
    Key::Char('r') => handle_recommended_tracks(app),
    _ if key == app.user_config.keys.add_item_to_queue => handle_add_item_to_queue(app),
    // Add `s` to "see more" on each option
    _ => {}
  }
}
