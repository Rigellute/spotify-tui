use crate::app::{
  ActiveBlock, AlbumTableContext, App, Artist, ArtistBlock, EpisodeTableContext, RouteId,
  ScrollableResultPages, SelectedAlbum, SelectedFullAlbum, SelectedFullShow, SelectedShow,
  TrackTableContext,
};
use crate::config::ClientConfig;
use anyhow::anyhow;
use rspotify::{
  client::Spotify,
  model::{
    album::SimplifiedAlbum,
    artist::FullArtist,
    offset::for_position,
    page::Page,
    playlist::{PlaylistTrack, SimplifiedPlaylist},
    recommend::Recommendations,
    search::SearchResult,
    show::SimplifiedShow,
    track::FullTrack,
    PlayingItem,
  },
  oauth2::{SpotifyClientCredentials, SpotifyOAuth, TokenInfo},
  senum::{AdditionalType, Country, RepeatState, SearchType},
  util::get_token,
};
use serde_json::{map::Map, Value};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::{
  sync::Arc,
  time::{Duration, Instant, SystemTime},
};
use tokio::sync::Mutex;
use tokio::try_join;

#[derive(Debug)]
pub enum IoEvent {
  GetCurrentPlayback,
  RefreshAuthentication,
  GetPlaylists,
  GetDevices,
  GetSearchResults(String, Option<Country>),
  SetTracksToTable(Vec<FullTrack>),
  GetMadeForYouPlaylistTracks(String, u32),
  GetPlaylistTracks(String, u32),
  GetCurrentSavedTracks(Option<u32>),
  StartPlayback(Option<String>, Option<Vec<String>>, Option<usize>),
  UpdateSearchLimits(u32, u32),
  Seek(u32),
  NextTrack,
  PreviousTrack,
  Shuffle(bool),
  Repeat(RepeatState),
  PausePlayback,
  ChangeVolume(u8),
  GetArtist(String, String, Option<Country>),
  GetAlbumTracks(Box<SimplifiedAlbum>),
  GetRecommendationsForSeed(
    Option<Vec<String>>,
    Option<Vec<String>>,
    Box<Option<FullTrack>>,
    Option<Country>,
  ),
  GetCurrentUserSavedAlbums(Option<u32>),
  CurrentUserSavedAlbumsContains(Vec<String>),
  CurrentUserSavedAlbumDelete(String),
  CurrentUserSavedAlbumAdd(String),
  UserUnfollowArtists(Vec<String>),
  UserFollowArtists(Vec<String>),
  UserFollowPlaylist(String, String, Option<bool>),
  UserUnfollowPlaylist(String, String),
  MadeForYouSearchAndAdd(String, Option<Country>),
  GetAudioAnalysis(String),
  GetUser,
  ToggleSaveTrack(String),
  GetRecommendationsForTrackId(String, Option<Country>),
  GetRecentlyPlayed,
  GetFollowedArtists(Option<String>),
  SetArtistsToTable(Vec<FullArtist>),
  UserArtistFollowCheck(Vec<String>),
  GetAlbum(String),
  TransferPlaybackToDevice(String),
  GetAlbumForTrack(String),
  CurrentUserSavedTracksContains(Vec<String>),
  GetCurrentUserSavedShows(Option<u32>),
  CurrentUserSavedShowsContains(Vec<String>),
  CurrentUserSavedShowDelete(String),
  CurrentUserSavedShowAdd(String),
  GetShowEpisodes(Box<SimplifiedShow>),
  GetShow(String),
  GetCurrentShowEpisodes(String, Option<u32>),
  AddItemToQueue(String),
  PlaylistNew(String, String, Option<bool>, String),
  PlaylistImport(String, String, String, PathBuf),
  PlaylistFork(String, String, PathBuf),
  PlaylistsUpdate(String, PathBuf),
}

pub fn get_spotify(token_info: TokenInfo) -> (Spotify, SystemTime) {
  let token_expiry = {
    if let Some(expires_at) = token_info.expires_at {
      SystemTime::UNIX_EPOCH
        + Duration::from_secs(expires_at as u64)
        // Set 10 seconds early
        - Duration::from_secs(10)
    } else {
      SystemTime::now()
    }
  };

  let client_credential = SpotifyClientCredentials::default()
    .token_info(token_info)
    .build();

  let spotify = Spotify::default()
    .client_credentials_manager(client_credential)
    .build();

  (spotify, token_expiry)
}

#[derive(Clone)]
pub struct Network<'a> {
  oauth: SpotifyOAuth,
  pub spotify: Spotify,
  large_search_limit: u32,
  small_search_limit: u32,
  pub client_config: ClientConfig,
  pub app: &'a Arc<Mutex<App>>,
}

impl<'a> Network<'a> {
  pub fn new(
    oauth: SpotifyOAuth,
    spotify: Spotify,
    client_config: ClientConfig,
    app: &'a Arc<Mutex<App>>,
  ) -> Self {
    Network {
      oauth,
      spotify,
      large_search_limit: 20,
      small_search_limit: 4,
      client_config,
      app,
    }
  }

  #[allow(clippy::cognitive_complexity)]
  pub async fn handle_network_event(&mut self, io_event: IoEvent) {
    match io_event {
      IoEvent::RefreshAuthentication => {
        self.refresh_authentication().await;
      }
      IoEvent::GetPlaylists => {
        self.get_current_user_playlists().await;
      }
      IoEvent::GetUser => {
        self.get_user().await;
      }
      IoEvent::GetDevices => {
        self.get_devices().await;
      }
      IoEvent::GetCurrentPlayback => {
        self.get_current_playback().await;
      }
      IoEvent::SetTracksToTable(full_tracks) => {
        self.set_tracks_to_table(full_tracks).await;
      }
      IoEvent::GetSearchResults(search_term, country) => {
        self.get_search_results(search_term, country).await;
      }
      IoEvent::GetMadeForYouPlaylistTracks(playlist_id, made_for_you_offset) => {
        self
          .get_made_for_you_playlist_tracks(playlist_id, made_for_you_offset)
          .await;
      }
      IoEvent::GetPlaylistTracks(playlist_id, playlist_offset) => {
        self.get_playlist_tracks(playlist_id, playlist_offset).await;
      }
      IoEvent::GetCurrentSavedTracks(offset) => {
        self.get_current_user_saved_tracks(offset).await;
      }
      IoEvent::StartPlayback(context_uri, uris, offset) => {
        self.start_playback(context_uri, uris, offset).await;
      }
      IoEvent::UpdateSearchLimits(large_search_limit, small_search_limit) => {
        self.large_search_limit = large_search_limit;
        self.small_search_limit = small_search_limit;
      }
      IoEvent::Seek(position_ms) => {
        self.seek(position_ms).await;
      }
      IoEvent::NextTrack => {
        self.next_track().await;
      }
      IoEvent::PreviousTrack => {
        self.previous_track().await;
      }
      IoEvent::Repeat(repeat_state) => {
        self.repeat(repeat_state).await;
      }
      IoEvent::PausePlayback => {
        self.pause_playback().await;
      }
      IoEvent::ChangeVolume(volume) => {
        self.change_volume(volume).await;
      }
      IoEvent::GetArtist(artist_id, input_artist_name, country) => {
        self.get_artist(artist_id, input_artist_name, country).await;
      }
      IoEvent::GetAlbumTracks(album) => {
        self.get_album_tracks(album).await;
      }
      IoEvent::GetRecommendationsForSeed(seed_artists, seed_tracks, first_track, country) => {
        self
          .get_recommendations_for_seed(seed_artists, seed_tracks, first_track, country)
          .await;
      }
      IoEvent::GetCurrentUserSavedAlbums(offset) => {
        self.get_current_user_saved_albums(offset).await;
      }
      IoEvent::CurrentUserSavedAlbumsContains(album_ids) => {
        self.current_user_saved_albums_contains(album_ids).await;
      }
      IoEvent::CurrentUserSavedAlbumDelete(album_id) => {
        self.current_user_saved_album_delete(album_id).await;
      }
      IoEvent::CurrentUserSavedAlbumAdd(album_id) => {
        self.current_user_saved_album_add(album_id).await;
      }
      IoEvent::UserUnfollowArtists(artist_ids) => {
        self.user_unfollow_artists(artist_ids).await;
      }
      IoEvent::UserFollowArtists(artist_ids) => {
        self.user_follow_artists(artist_ids).await;
      }
      IoEvent::UserFollowPlaylist(playlist_owner_id, playlist_id, is_public) => {
        self
          .user_follow_playlist(playlist_owner_id, playlist_id, is_public)
          .await;
      }
      IoEvent::UserUnfollowPlaylist(user_id, playlist_id) => {
        self.user_unfollow_playlist(user_id, playlist_id).await;
      }
      IoEvent::MadeForYouSearchAndAdd(search_term, country) => {
        self.made_for_you_search_and_add(search_term, country).await;
      }
      IoEvent::GetAudioAnalysis(uri) => {
        self.get_audio_analysis(uri).await;
      }
      IoEvent::ToggleSaveTrack(track_id) => {
        self.toggle_save_track(track_id).await;
      }
      IoEvent::GetRecommendationsForTrackId(track_id, country) => {
        self
          .get_recommendations_for_track_id(track_id, country)
          .await;
      }
      IoEvent::GetRecentlyPlayed => {
        self.get_recently_played().await;
      }
      IoEvent::GetFollowedArtists(after) => {
        self.get_followed_artists(after).await;
      }
      IoEvent::SetArtistsToTable(full_artists) => {
        self.set_artists_to_table(full_artists).await;
      }
      IoEvent::UserArtistFollowCheck(artist_ids) => {
        self.user_artist_check_follow(artist_ids).await;
      }
      IoEvent::GetAlbum(album_id) => {
        self.get_album(album_id).await;
      }
      IoEvent::TransferPlaybackToDevice(device_id) => {
        self.transfert_playback_to_device(device_id).await;
      }
      IoEvent::GetAlbumForTrack(track_id) => {
        self.get_album_for_track(track_id).await;
      }
      IoEvent::Shuffle(shuffle_state) => {
        self.shuffle(shuffle_state).await;
      }
      IoEvent::CurrentUserSavedTracksContains(track_ids) => {
        self.current_user_saved_tracks_contains(track_ids).await;
      }
      IoEvent::GetCurrentUserSavedShows(offset) => {
        self.get_current_user_saved_shows(offset).await;
      }
      IoEvent::CurrentUserSavedShowsContains(show_ids) => {
        self.current_user_saved_shows_contains(show_ids).await;
      }
      IoEvent::CurrentUserSavedShowDelete(show_id) => {
        self.current_user_saved_shows_delete(show_id).await;
      }
      IoEvent::CurrentUserSavedShowAdd(show_id) => {
        self.current_user_saved_shows_add(show_id).await;
      }
      IoEvent::GetShowEpisodes(show) => {
        self.get_show_episodes(show).await;
      }
      IoEvent::GetShow(show_id) => {
        self.get_show(show_id).await;
      }
      IoEvent::GetCurrentShowEpisodes(show_id, offset) => {
        self.get_current_show_episodes(show_id, offset).await;
      }
      IoEvent::AddItemToQueue(item) => {
        self.add_item_to_queue(item).await;
      }

      IoEvent::PlaylistNew(userid, name, public, description) => {
        self
          .playlist_new(userid, name, public, Some(description))
          .await;
      }
      IoEvent::PlaylistImport(user_id, import_from, import_to, import_file) => {
        self
          .playlist_import(user_id, import_from, import_to, import_file)
          .await;
      }
      IoEvent::PlaylistFork(user_id, playlist_id, import_dir) => {
        self.playlist_fork(user_id, playlist_id, import_dir).await;
      }
      IoEvent::PlaylistsUpdate(user_id, import_dir) => {
        self.playlists_update(user_id, import_dir).await;
      }
    };

    let mut app = self.app.lock().await;
    app.is_loading = false;
  }

  async fn handle_error(&mut self, e: anyhow::Error) {
    let mut app = self.app.lock().await;
    app.handle_error(e);
  }

  async fn get_user(&mut self) {
    match self.spotify.current_user().await {
      Ok(user) => {
        let mut app = self.app.lock().await;
        app.user = Some(user);
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn get_devices(&mut self) {
    if let Ok(result) = self.spotify.device().await {
      let mut app = self.app.lock().await;
      app.push_navigation_stack(RouteId::SelectedDevice, ActiveBlock::SelectDevice);
      if !result.devices.is_empty() {
        app.devices = Some(result);
        // Select the first device in the list
        app.selected_device_index = Some(0);
      }
    }
  }

  async fn get_current_playback(&mut self) {
    let context = self
      .spotify
      .current_playback(
        None,
        Some(vec![AdditionalType::Episode, AdditionalType::Track]),
      )
      .await;

    match context {
      Ok(Some(c)) => {
        let mut app = self.app.lock().await;
        app.current_playback_context = Some(c.clone());
        app.instant_since_last_current_playback_poll = Instant::now();

        if let Some(item) = c.item {
          match item {
            PlayingItem::Track(track) => {
              if let Some(track_id) = track.id {
                app.dispatch(IoEvent::CurrentUserSavedTracksContains(vec![track_id]));
              };
            }
            PlayingItem::Episode(_episode) => { /*should map this to following the podcast show*/ }
          }
        };
      }
      Ok(None) => {
        let mut app = self.app.lock().await;
        app.instant_since_last_current_playback_poll = Instant::now();
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }

    let mut app = self.app.lock().await;
    app.seek_ms.take();
    app.is_fetching_current_playback = false;
  }

  async fn current_user_saved_tracks_contains(&mut self, ids: Vec<String>) {
    match self.spotify.current_user_saved_tracks_contains(&ids).await {
      Ok(is_saved_vec) => {
        let mut app = self.app.lock().await;
        for (i, id) in ids.iter().enumerate() {
          if let Some(is_liked) = is_saved_vec.get(i) {
            if *is_liked {
              app.liked_song_ids_set.insert(id.to_string());
            } else {
              // The song is not liked, so check if it should be removed
              if app.liked_song_ids_set.contains(id) {
                app.liked_song_ids_set.remove(id);
              }
            }
          };
        }
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn get_playlist_tracks(&mut self, playlist_id: String, playlist_offset: u32) {
    if let Ok(playlist_tracks) = self
      .spotify
      .user_playlist_tracks(
        "spotify",
        &playlist_id,
        None,
        Some(self.large_search_limit),
        Some(playlist_offset),
        None,
      )
      .await
    {
      self.set_playlist_tracks_to_table(&playlist_tracks).await;

      let mut app = self.app.lock().await;
      app.playlist_tracks = Some(playlist_tracks);
      app.push_navigation_stack(RouteId::TrackTable, ActiveBlock::TrackTable);
    };
  }

  async fn set_playlist_tracks_to_table(&mut self, playlist_track_page: &Page<PlaylistTrack>) {
    self
      .set_tracks_to_table(
        playlist_track_page
          .items
          .clone()
          .into_iter()
          .filter_map(|item| item.track)
          .collect::<Vec<FullTrack>>(),
      )
      .await;
  }

  async fn set_tracks_to_table(&mut self, tracks: Vec<FullTrack>) {
    let mut app = self.app.lock().await;
    app.track_table.tracks = tracks.clone();

    // Send this event round (don't block here)
    app.dispatch(IoEvent::CurrentUserSavedTracksContains(
      tracks
        .into_iter()
        .filter_map(|item| item.id)
        .collect::<Vec<String>>(),
    ));
  }

  async fn set_artists_to_table(&mut self, artists: Vec<FullArtist>) {
    let mut app = self.app.lock().await;
    app.artists = artists;
  }

  async fn get_made_for_you_playlist_tracks(
    &mut self,
    playlist_id: String,
    made_for_you_offset: u32,
  ) {
    if let Ok(made_for_you_tracks) = self
      .spotify
      .user_playlist_tracks(
        "spotify",
        &playlist_id,
        None,
        Some(self.large_search_limit),
        Some(made_for_you_offset),
        None,
      )
      .await
    {
      self
        .set_playlist_tracks_to_table(&made_for_you_tracks)
        .await;

      let mut app = self.app.lock().await;
      app.made_for_you_tracks = Some(made_for_you_tracks);
      if app.get_current_route().id != RouteId::TrackTable {
        app.push_navigation_stack(RouteId::TrackTable, ActiveBlock::TrackTable);
      }
    }
  }

  async fn get_current_user_saved_shows(&mut self, offset: Option<u32>) {
    match self
      .spotify
      .get_saved_show(self.large_search_limit, offset)
      .await
    {
      Ok(saved_shows) => {
        // not to show a blank page
        if !saved_shows.items.is_empty() {
          let mut app = self.app.lock().await;
          app.library.saved_shows.add_pages(saved_shows);
        }
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn current_user_saved_shows_contains(&mut self, show_ids: Vec<String>) {
    if let Ok(are_followed) = self
      .spotify
      .check_users_saved_shows(show_ids.to_owned())
      .await
    {
      let mut app = self.app.lock().await;
      show_ids.iter().enumerate().for_each(|(i, id)| {
        if are_followed[i] {
          app.saved_show_ids_set.insert(id.to_owned());
        } else {
          app.saved_show_ids_set.remove(id);
        }
      })
    }
  }

  async fn get_show_episodes(&mut self, show: Box<SimplifiedShow>) {
    match self
      .spotify
      .get_shows_episodes(show.id.clone(), self.large_search_limit, 0, None)
      .await
    {
      Ok(episodes) => {
        if !episodes.items.is_empty() {
          let mut app = self.app.lock().await;
          app.library.show_episodes = ScrollableResultPages::new();
          app.library.show_episodes.add_pages(episodes);

          app.selected_show_simplified = Some(SelectedShow { show: *show });

          app.episode_table_context = EpisodeTableContext::Simplified;

          app.push_navigation_stack(RouteId::PodcastEpisodes, ActiveBlock::EpisodeTable);
        }
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn get_show(&mut self, show_id: String) {
    match self.spotify.get_a_show(show_id, None).await {
      Ok(show) => {
        let selected_show = SelectedFullShow { show };

        let mut app = self.app.lock().await;

        app.selected_show_full = Some(selected_show);

        app.episode_table_context = EpisodeTableContext::Full;
        app.push_navigation_stack(RouteId::PodcastEpisodes, ActiveBlock::EpisodeTable);
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn get_current_show_episodes(&mut self, show_id: String, offset: Option<u32>) {
    match self
      .spotify
      .get_shows_episodes(show_id, self.large_search_limit, offset, None)
      .await
    {
      Ok(episodes) => {
        if !episodes.items.is_empty() {
          let mut app = self.app.lock().await;
          app.library.show_episodes.add_pages(episodes);
        }
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn get_search_results(&mut self, search_term: String, country: Option<Country>) {
    let search_track = self.spotify.search(
      &search_term,
      SearchType::Track,
      self.small_search_limit,
      0,
      country,
      None,
    );

    let search_artist = self.spotify.search(
      &search_term,
      SearchType::Artist,
      self.small_search_limit,
      0,
      country,
      None,
    );

    let search_album = self.spotify.search(
      &search_term,
      SearchType::Album,
      self.small_search_limit,
      0,
      country,
      None,
    );

    let search_playlist = self.spotify.search(
      &search_term,
      SearchType::Playlist,
      self.small_search_limit,
      0,
      country,
      None,
    );

    let search_show = self.spotify.search(
      &search_term,
      SearchType::Show,
      self.small_search_limit,
      0,
      country,
      None,
    );

    // Run the futures concurrently
    match try_join!(
      search_track,
      search_artist,
      search_album,
      search_playlist,
      search_show
    ) {
      Ok((
        SearchResult::Tracks(track_results),
        SearchResult::Artists(artist_results),
        SearchResult::Albums(album_results),
        SearchResult::Playlists(playlist_results),
        SearchResult::Shows(show_results),
      )) => {
        let mut app = self.app.lock().await;

        let artist_ids = album_results
          .items
          .iter()
          .filter_map(|item| item.id.to_owned())
          .collect();

        // Check if these artists are followed
        app.dispatch(IoEvent::UserArtistFollowCheck(artist_ids));

        let album_ids = album_results
          .items
          .iter()
          .filter_map(|album| album.id.to_owned())
          .collect();

        // Check if these albums are saved
        app.dispatch(IoEvent::CurrentUserSavedAlbumsContains(album_ids));

        let show_ids = show_results
          .items
          .iter()
          .map(|show| show.id.to_owned())
          .collect();

        // check if these shows are saved
        app.dispatch(IoEvent::CurrentUserSavedShowsContains(show_ids));

        app.search_results.tracks = Some(track_results);
        app.search_results.artists = Some(artist_results);
        app.search_results.albums = Some(album_results);
        app.search_results.playlists = Some(playlist_results);
        app.search_results.shows = Some(show_results);
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
      _ => {}
    };
  }

  async fn get_current_user_saved_tracks(&mut self, offset: Option<u32>) {
    match self
      .spotify
      .current_user_saved_tracks(self.large_search_limit, offset)
      .await
    {
      Ok(saved_tracks) => {
        let mut app = self.app.lock().await;
        app.track_table.tracks = saved_tracks
          .items
          .clone()
          .into_iter()
          .map(|item| item.track)
          .collect::<Vec<FullTrack>>();

        saved_tracks.items.iter().for_each(|item| {
          if let Some(track_id) = &item.track.id {
            app.liked_song_ids_set.insert(track_id.to_string());
          }
        });

        app.library.saved_tracks.add_pages(saved_tracks);
        app.track_table.context = Some(TrackTableContext::SavedTracks);
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn start_playback(
    &mut self,
    context_uri: Option<String>,
    uris: Option<Vec<String>>,
    offset: Option<usize>,
  ) {
    let (uris, context_uri) = if context_uri.is_some() {
      (None, context_uri)
    } else if uris.is_some() {
      (uris, None)
    } else {
      (None, None)
    };

    let offset = offset.and_then(|o| for_position(o as u32));

    let result = match &self.client_config.device_id {
      Some(device_id) => {
        match self
          .spotify
          .start_playback(
            Some(device_id.to_string()),
            context_uri.clone(),
            uris.clone(),
            offset.clone(),
            None,
          )
          .await
        {
          Ok(()) => Ok(()),
          Err(e) => Err(anyhow!(e)),
        }
      }
      None => Err(anyhow!("No device_id selected")),
    };

    match result {
      Ok(()) => {
        let mut app = self.app.lock().await;
        app.song_progress_ms = 0;
        app.dispatch(IoEvent::GetCurrentPlayback);
      }
      Err(e) => {
        self.handle_error(e).await;
      }
    }
  }

  async fn seek(&mut self, position_ms: u32) {
    if let Some(device_id) = &self.client_config.device_id {
      match self
        .spotify
        .seek_track(position_ms, Some(device_id.to_string()))
        .await
      {
        Ok(()) => {
          // Wait between seek and status query.
          // Without it, the Spotify API may return the old progress.
          tokio::time::delay_for(Duration::from_millis(1000)).await;
          self.get_current_playback().await;
        }
        Err(e) => {
          self.handle_error(anyhow!(e)).await;
        }
      };
    }
  }

  async fn next_track(&mut self) {
    match self
      .spotify
      .next_track(self.client_config.device_id.clone())
      .await
    {
      Ok(()) => {
        self.get_current_playback().await;
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    };
  }

  async fn previous_track(&mut self) {
    match self
      .spotify
      .previous_track(self.client_config.device_id.clone())
      .await
    {
      Ok(()) => {
        self.get_current_playback().await;
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    };
  }

  async fn shuffle(&mut self, shuffle_state: bool) {
    match self
      .spotify
      .shuffle(!shuffle_state, self.client_config.device_id.clone())
      .await
    {
      Ok(()) => {
        // Update the UI eagerly (otherwise the UI will wait until the next 5 second interval
        // due to polling playback context)
        let mut app = self.app.lock().await;
        if let Some(current_playback_context) = &mut app.current_playback_context {
          current_playback_context.shuffle_state = !shuffle_state;
        };
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    };
  }

  async fn repeat(&mut self, repeat_state: RepeatState) {
    let next_repeat_state = match repeat_state {
      RepeatState::Off => RepeatState::Context,
      RepeatState::Context => RepeatState::Track,
      RepeatState::Track => RepeatState::Off,
    };
    match self
      .spotify
      .repeat(next_repeat_state, self.client_config.device_id.clone())
      .await
    {
      Ok(()) => {
        let mut app = self.app.lock().await;
        if let Some(current_playback_context) = &mut app.current_playback_context {
          current_playback_context.repeat_state = next_repeat_state;
        };
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    };
  }

  async fn pause_playback(&mut self) {
    match self
      .spotify
      .pause_playback(self.client_config.device_id.clone())
      .await
    {
      Ok(()) => {
        self.get_current_playback().await;
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    };
  }

  async fn change_volume(&mut self, volume_percent: u8) {
    match self
      .spotify
      .volume(volume_percent, self.client_config.device_id.clone())
      .await
    {
      Ok(()) => {
        let mut app = self.app.lock().await;
        if let Some(current_playback_context) = &mut app.current_playback_context {
          current_playback_context.device.volume_percent = volume_percent.into();
        };
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    };
  }

  async fn get_artist(
    &mut self,
    artist_id: String,
    input_artist_name: String,
    country: Option<Country>,
  ) {
    let albums = self.spotify.artist_albums(
      &artist_id,
      None,
      country,
      Some(self.large_search_limit),
      Some(0),
    );
    let artist_name = if input_artist_name.is_empty() {
      self
        .spotify
        .artist(&artist_id)
        .await
        .map(|full_artist| full_artist.name)
        .unwrap_or_default()
    } else {
      input_artist_name
    };
    let top_tracks = self.spotify.artist_top_tracks(&artist_id, country);
    let related_artist = self.spotify.artist_related_artists(&artist_id);

    if let Ok((albums, top_tracks, related_artist)) = try_join!(albums, top_tracks, related_artist)
    {
      let mut app = self.app.lock().await;

      app.dispatch(IoEvent::CurrentUserSavedAlbumsContains(
        albums
          .items
          .iter()
          .filter_map(|item| item.id.to_owned())
          .collect(),
      ));

      app.artist = Some(Artist {
        artist_name,
        albums,
        related_artists: related_artist.artists,
        top_tracks: top_tracks.tracks,
        selected_album_index: 0,
        selected_related_artist_index: 0,
        selected_top_track_index: 0,
        artist_hovered_block: ArtistBlock::TopTracks,
        artist_selected_block: ArtistBlock::Empty,
      });
    }
  }

  async fn get_album_tracks(&mut self, album: Box<SimplifiedAlbum>) {
    if let Some(album_id) = &album.id {
      match self
        .spotify
        .album_track(&album_id.clone(), self.large_search_limit, 0)
        .await
      {
        Ok(tracks) => {
          let track_ids = tracks
            .items
            .iter()
            .filter_map(|item| item.id.clone())
            .collect::<Vec<String>>();

          let mut app = self.app.lock().await;
          app.selected_album_simplified = Some(SelectedAlbum {
            album: *album,
            tracks,
            selected_index: 0,
          });

          app.album_table_context = AlbumTableContext::Simplified;
          app.push_navigation_stack(RouteId::AlbumTracks, ActiveBlock::AlbumTracks);
          app.dispatch(IoEvent::CurrentUserSavedTracksContains(track_ids));
        }
        Err(e) => {
          self.handle_error(anyhow!(e)).await;
        }
      }
    }
  }

  async fn get_recommendations_for_seed(
    &mut self,
    seed_artists: Option<Vec<String>>,
    seed_tracks: Option<Vec<String>>,
    first_track: Box<Option<FullTrack>>,
    country: Option<Country>,
  ) {
    let empty_payload: Map<String, Value> = Map::new();

    match self
      .spotify
      .recommendations(
        seed_artists,            // artists
        None,                    // genres
        seed_tracks,             // tracks
        self.large_search_limit, // adjust playlist to screen size
        country,                 // country
        &empty_payload,          // payload
      )
      .await
    {
      Ok(result) => {
        if let Some(mut recommended_tracks) = self.extract_recommended_tracks(&result).await {
          //custom first track
          if let Some(track) = *first_track {
            recommended_tracks.insert(0, track);
          }

          let track_ids = recommended_tracks
            .iter()
            .map(|x| x.uri.clone())
            .collect::<Vec<String>>();

          self.set_tracks_to_table(recommended_tracks.clone()).await;

          let mut app = self.app.lock().await;
          app.recommended_tracks = recommended_tracks;
          app.track_table.context = Some(TrackTableContext::RecommendedTracks);

          if app.get_current_route().id != RouteId::Recommendations {
            app.push_navigation_stack(RouteId::Recommendations, ActiveBlock::TrackTable);
          };

          app.dispatch(IoEvent::StartPlayback(None, Some(track_ids), Some(0)));
        }
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn extract_recommended_tracks(
    &mut self,
    recommendations: &Recommendations,
  ) -> Option<Vec<FullTrack>> {
    let tracks = recommendations
      .clone()
      .tracks
      .into_iter()
      .map(|item| item.uri)
      .collect::<Vec<String>>();
    if let Ok(result) = self
      .spotify
      .tracks(tracks.iter().map(|x| &x[..]).collect::<Vec<&str>>(), None)
      .await
    {
      return Some(result.tracks);
    }

    None
  }

  async fn get_recommendations_for_track_id(&mut self, id: String, country: Option<Country>) {
    if let Ok(track) = self.spotify.track(&id).await {
      let track_id_list = track.id.as_ref().map(|id| vec![id.to_string()]);
      self
        .get_recommendations_for_seed(None, track_id_list, Box::new(Some(track)), country)
        .await;
    }
  }

  async fn toggle_save_track(&mut self, track_id: String) {
    match self
      .spotify
      .current_user_saved_tracks_contains(&[track_id.clone()])
      .await
    {
      Ok(saved) => {
        if saved.first() == Some(&true) {
          match self
            .spotify
            .current_user_saved_tracks_delete(&[track_id.clone()])
            .await
          {
            Ok(()) => {
              let mut app = self.app.lock().await;
              app.liked_song_ids_set.remove(&track_id);
            }
            Err(e) => {
              self.handle_error(anyhow!(e)).await;
            }
          }
        } else {
          match self
            .spotify
            .current_user_saved_tracks_add(&[track_id.clone()])
            .await
          {
            Ok(()) => {
              // TODO: This should ideally use the same logic as `self.current_user_saved_tracks_contains`
              let mut app = self.app.lock().await;
              app.liked_song_ids_set.insert(track_id);
            }
            Err(e) => {
              self.handle_error(anyhow!(e)).await;
            }
          }
        }
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    };
  }

  async fn get_followed_artists(&mut self, after: Option<String>) {
    match self
      .spotify
      .current_user_followed_artists(self.large_search_limit, after)
      .await
    {
      Ok(saved_artists) => {
        let mut app = self.app.lock().await;
        app.artists = saved_artists.artists.items.to_owned();
        app.library.saved_artists.add_pages(saved_artists.artists);
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    };
  }

  async fn user_artist_check_follow(&mut self, artist_ids: Vec<String>) {
    if let Ok(are_followed) = self.spotify.user_artist_check_follow(&artist_ids).await {
      let mut app = self.app.lock().await;
      artist_ids.iter().enumerate().for_each(|(i, id)| {
        if are_followed[i] {
          app.followed_artist_ids_set.insert(id.to_owned());
        } else {
          app.followed_artist_ids_set.remove(id);
        }
      });
    }
  }

  async fn get_current_user_saved_albums(&mut self, offset: Option<u32>) {
    match self
      .spotify
      .current_user_saved_albums(self.large_search_limit, offset)
      .await
    {
      Ok(saved_albums) => {
        // not to show a blank page
        if !saved_albums.items.is_empty() {
          let mut app = self.app.lock().await;
          app.library.saved_albums.add_pages(saved_albums);
        }
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    };
  }

  async fn current_user_saved_albums_contains(&mut self, album_ids: Vec<String>) {
    if let Ok(are_followed) = self
      .spotify
      .current_user_saved_albums_contains(&album_ids)
      .await
    {
      let mut app = self.app.lock().await;
      album_ids.iter().enumerate().for_each(|(i, id)| {
        if are_followed[i] {
          app.saved_album_ids_set.insert(id.to_owned());
        } else {
          app.saved_album_ids_set.remove(id);
        }
      });
    }
  }

  pub async fn current_user_saved_album_delete(&mut self, album_id: String) {
    match self
      .spotify
      .current_user_saved_albums_delete(&[album_id.to_owned()])
      .await
    {
      Ok(_) => {
        self.get_current_user_saved_albums(None).await;
        let mut app = self.app.lock().await;
        app.saved_album_ids_set.remove(&album_id.to_owned());
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    };
  }

  async fn current_user_saved_album_add(&mut self, album_id: String) {
    match self
      .spotify
      .current_user_saved_albums_add(&[album_id.to_owned()])
      .await
    {
      Ok(_) => {
        let mut app = self.app.lock().await;
        app.saved_album_ids_set.insert(album_id.to_owned());
      }
      Err(e) => self.handle_error(anyhow!(e)).await,
    }
  }

  async fn current_user_saved_shows_delete(&mut self, show_id: String) {
    match self
      .spotify
      .remove_users_saved_shows(vec![show_id.to_owned()], None)
      .await
    {
      Ok(_) => {
        self.get_current_user_saved_shows(None).await;
        let mut app = self.app.lock().await;
        app.saved_show_ids_set.remove(&show_id.to_owned());
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn current_user_saved_shows_add(&mut self, show_id: String) {
    match self.spotify.save_shows(vec![show_id.to_owned()]).await {
      Ok(_) => {
        self.get_current_user_saved_shows(None).await;
        let mut app = self.app.lock().await;
        app.saved_show_ids_set.insert(show_id.to_owned());
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn user_unfollow_artists(&mut self, artist_ids: Vec<String>) {
    match self.spotify.user_unfollow_artists(&artist_ids).await {
      Ok(_) => {
        self.get_followed_artists(None).await;
        let mut app = self.app.lock().await;
        artist_ids.iter().for_each(|id| {
          app.followed_artist_ids_set.remove(&id.to_owned());
        });
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn user_follow_artists(&mut self, artist_ids: Vec<String>) {
    match self.spotify.user_follow_artists(&artist_ids).await {
      Ok(_) => {
        self.get_followed_artists(None).await;
        let mut app = self.app.lock().await;
        artist_ids.iter().for_each(|id| {
          app.followed_artist_ids_set.insert(id.to_owned());
        });
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn user_follow_playlist(
    &mut self,
    playlist_owner_id: String,
    playlist_id: String,
    is_public: Option<bool>,
  ) {
    match self
      .spotify
      .user_playlist_follow_playlist(&playlist_owner_id, &playlist_id, is_public)
      .await
    {
      Ok(_) => {
        self.get_current_user_playlists().await;
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn user_unfollow_playlist(&mut self, user_id: String, playlist_id: String) {
    match self
      .spotify
      .user_playlist_unfollow(&user_id, &playlist_id)
      .await
    {
      Ok(_) => {
        self.get_current_user_playlists().await;
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn made_for_you_search_and_add(&mut self, search_string: String, country: Option<Country>) {
    const SPOTIFY_ID: &str = "spotify";

    match self
      .spotify
      .search(
        &search_string,
        SearchType::Playlist,
        self.large_search_limit,
        0,
        country,
        None,
      )
      .await
    {
      Ok(SearchResult::Playlists(mut search_playlists)) => {
        let mut filtered_playlists = search_playlists
          .items
          .iter()
          .filter(|playlist| playlist.owner.id == SPOTIFY_ID && playlist.name == search_string)
          .map(|playlist| playlist.to_owned())
          .collect::<Vec<SimplifiedPlaylist>>();

        let mut app = self.app.lock().await;
        if !app.library.made_for_you_playlists.pages.is_empty() {
          app
            .library
            .made_for_you_playlists
            .get_mut_results(None)
            .unwrap()
            .items
            .append(&mut filtered_playlists);
        } else {
          search_playlists.items = filtered_playlists;
          app
            .library
            .made_for_you_playlists
            .add_pages(search_playlists);
        }
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
      _ => {}
    }
  }

  async fn get_audio_analysis(&mut self, uri: String) {
    match self.spotify.audio_analysis(&uri).await {
      Ok(result) => {
        let mut app = self.app.lock().await;
        app.audio_analysis = Some(result);
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn get_current_user_playlists(&mut self) {
    let playlists = self
      .spotify
      .current_user_playlists(self.large_search_limit, None)
      .await;

    match playlists {
      Ok(p) => {
        let mut app = self.app.lock().await;
        app.playlists = Some(p);
        // Select the first playlist
        app.selected_playlist_index = Some(0);
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    };
  }

  async fn get_recently_played(&mut self) {
    match self
      .spotify
      .current_user_recently_played(self.large_search_limit)
      .await
    {
      Ok(result) => {
        let track_ids = result
          .items
          .iter()
          .filter_map(|item| item.track.id.clone())
          .collect::<Vec<String>>();

        self.current_user_saved_tracks_contains(track_ids).await;

        let mut app = self.app.lock().await;

        app.recently_played.result = Some(result.clone());
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn get_album(&mut self, album_id: String) {
    match self.spotify.album(&album_id).await {
      Ok(album) => {
        let selected_album = SelectedFullAlbum {
          album,
          selected_index: 0,
        };

        let mut app = self.app.lock().await;

        app.selected_album_full = Some(selected_album);
        app.album_table_context = AlbumTableContext::Full;
        app.push_navigation_stack(RouteId::AlbumTracks, ActiveBlock::AlbumTracks);
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn get_album_for_track(&mut self, track_id: String) {
    match self.spotify.track(&track_id).await {
      Ok(track) => {
        // It is unclear when the id can ever be None, but perhaps a track can be album-less. If
        // so, there isn't much to do here anyways, since we're looking for the parent album.
        let album_id = match track.album.id {
          Some(id) => id,
          None => return,
        };

        if let Ok(album) = self.spotify.album(&album_id).await {
          // The way we map to the UI is zero-indexed, but Spotify is 1-indexed.
          let zero_indexed_track_number = track.track_number - 1;
          let selected_album = SelectedFullAlbum {
            album,
            // Overflow should be essentially impossible here, so we prefer the cleaner 'as'.
            selected_index: zero_indexed_track_number as usize,
          };

          let mut app = self.app.lock().await;

          app.selected_album_full = Some(selected_album.clone());
          app.saved_album_tracks_index = selected_album.selected_index;
          app.album_table_context = AlbumTableContext::Full;
          app.push_navigation_stack(RouteId::AlbumTracks, ActiveBlock::AlbumTracks);
        }
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  async fn transfert_playback_to_device(&mut self, device_id: String) {
    match self.spotify.transfer_playback(&device_id, true).await {
      Ok(()) => {
        self.get_current_playback().await;
      }
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
        return;
      }
    };

    match self.client_config.set_device_id(device_id) {
      Ok(()) => {
        let mut app = self.app.lock().await;
        app.pop_navigation_stack();
      }
      Err(e) => {
        self.handle_error(e).await;
      }
    };
  }

  async fn refresh_authentication(&mut self) {
    if let Some(new_token_info) = get_token(&mut self.oauth).await {
      let (new_spotify, new_token_expiry) = get_spotify(new_token_info);
      self.spotify = new_spotify;
      let mut app = self.app.lock().await;
      app.spotify_token_expiry = new_token_expiry;
    } else {
      println!("\nFailed to refresh authentication token");
      // TODO panic!
    }
  }

  async fn add_item_to_queue(&mut self, item: String) {
    match self
      .spotify
      .add_item_to_queue(item, self.client_config.device_id.clone())
      .await
    {
      Ok(()) => (),
      Err(e) => {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }

  // Just creates new playlist
  async fn playlist_new(
    &mut self,
    user_id: String,
    name: String,
    public: Option<bool>,
    mut description: Option<String>,
  ) {
    if description.is_none() {
      description = Some("".to_string());
    }

    if let Err(e) = self
      .spotify
      .user_playlist_create(user_id.as_str(), name.as_str(), public, description)
      .await
    {
      self.handle_error(anyhow!(e)).await;
    }
  }

  // If there has been no previous import -
  // Get import_to playlist and get its current tracks
  // Get import_from playlist tracks
  // Write tracks to import dir under import_to/import_from
  // Add all tracks to import_to playlist
  //
  // If already imported -
  // Check the difference in import vs old import
  // Add new tracks, prompt for deletion of deleted tracks
  async fn playlist_import(
    &mut self,
    user_id: String,
    mut import_from: String,
    mut import_to: String,
    import_file: PathBuf,
  ) {
    let import_to_playlist = self
      .spotify
      .user_playlist(
        user_id.as_str(),
        Some(import_to.as_mut_str()),
        None,
        Some(Country::UnitedStates),
      )
      .await;

    if import_to_playlist.is_err() {
      self
        .handle_error(anyhow!(
          "'import_to' playlist ({}) does not exist.",
          import_to
        ))
        .await;
      return;
    }

    let mut import_to_tracks = HashSet::new();
    for track in import_to_playlist.unwrap().tracks.items {
      import_to_tracks.insert(track.track.unwrap().id.unwrap());
    }

    let mut ids = Vec::new();

    if let Ok(items) = self
      .spotify
      .user_playlist(
        user_id.as_str(),
        Some(import_from.as_mut_str()),
        None,
        Some(Country::UnitedStates),
      )
      .await
    {
      for track in items.tracks.items {
        let name = track.track.as_ref().unwrap().name.to_owned();
        let id = track.track.as_ref().unwrap().id.to_owned().unwrap();
        ids.push(format!("{}:{}", id, name));
      }

      // Calculate hash of all track ids
      let mut hasher = DefaultHasher::new();
      ids.hash(&mut hasher);
      let hash = hasher.finish();

      println!("Importing {} into {}...", import_from, import_to);

      if !import_file.exists() {
        {
          // Create import_to dir
          if fs::create_dir_all(import_file.parent().unwrap()).is_err() {
            self
              .handle_error(anyhow!(
                "Could not write to import file at {}",
                import_file.as_os_str().to_str().unwrap()
              ))
              .await;
            return;
          }

          // Write hash to top of file
          let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&import_file)
            .unwrap();

          let mut hash_str = hash.to_string();
          hash_str.push_str("\n");
          if writeln!(file, "{}", hash_str).is_err() {
            self
              .handle_error(anyhow!(
                "Could not write to import file at {}",
                import_file.as_os_str().to_str().unwrap()
              ))
              .await;
            return;
          }
        }

        // Writing ids:name to import file
        let mut file = OpenOptions::new().append(true).open(&import_file).unwrap();
        println!("Copying {} tracks into {}...", ids.len(), import_to);
        for track in ids.iter() {
          let id = track.split(':').next().unwrap().to_string();
          if writeln!(file, "{}", track).is_err() {
            self
              .handle_error(anyhow!(
                "Could not write to import file at {}",
                import_file.as_os_str().to_str().unwrap()
              ))
              .await;
            return;
          }

          // if track not in import_to playlist, add it
          if !import_to_tracks.contains(&id) {
            if let Err(e) = self
              .spotify
              .user_playlist_add_tracks(&user_id, &import_to, &[id], Some(0))
              .await
            {
              self.handle_error(anyhow!(e)).await;
            }
          }
        }
      } else {

        // Comapare hash, if no change, nothing
        let file = OpenOptions::new().read(true).open(&import_file).unwrap();
        let reader = BufReader::new(file);

        let mut iter = reader.lines();
        let old_hash = iter.next().unwrap().unwrap().trim().to_string();

        println!("Comparing changes from last import...");

        if !old_hash.eq(&hash.to_string()) {
          // Need to rewrite the file now
          if let Err(e) = fs::remove_file(&import_file) {
            self.handle_error(anyhow!(e)).await;
          }

          {
            let mut file = OpenOptions::new()
              .write(true)
              .create(true)
              .open(&import_file)
              .unwrap();

            let mut hash_str = hash.to_string();
            hash_str.push_str("\n");
            if writeln!(file, "{}", hash_str).is_err() {
              self
                .handle_error(anyhow!(
                  "Could not write to import file at {}",
                  import_file.as_os_str().to_str().unwrap()
                ))
                .await;
              return;
            }
          }

          let mut file = OpenOptions::new().append(true).open(import_file).unwrap();

          for track in ids.iter() {
            if let Err(e) = writeln!(file, "{}", track) {
              self.handle_error(anyhow!(e)).await;
            }
          }

          // If change, find ids which do not match
          // Load in ids from file, get new ids, compare
          // add all new ids
          let mut old_ids: HashSet<String> = HashSet::new();
          let mut new_ids: HashSet<String> = HashSet::new();

          for id in ids {
            new_ids.insert(id);
          }

          // Consume empty line
          iter.next();

          // Add all ids from file to old_ids
          while let Some(Ok(id)) = iter.next() {
            old_ids.insert(id.trim().to_string());
          }

          // Will start with old_ids, removing ids that are in both.
          // At the end, we should have deleted ids in the old_ids hashset
          let deleted_tracks = old_ids.difference(&new_ids);
          for track in deleted_tracks {
            let id = track.split(':').next().unwrap().to_string();
            let mut input = String::new();
            print!(
              "The imported playlist has deleted {}, would you like to as well? [y/n]: ",
              track
            );

            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim().eq(&String::from("y")) || input.trim().eq(&String::from("Y")) {
              if let Err(e) = self
                .spotify
                .user_playlist_remove_all_occurrences_of_tracks(
                  user_id.as_str(),
                  &import_to,
                  &[id],
                  None,
                )
                .await
              {
                self.handle_error(anyhow!(e)).await;
              }
            }
          }

          // ids to add will be in the new_ids hashset
          let new_tracks = new_ids.difference(&old_ids);

          println!("Adding {} new tracks...", new_tracks.to_owned().count());
          for track in new_tracks {
            let id = track.split(':').next().unwrap().to_string();

            if !import_to_tracks.contains(&id) {
              if let Err(e) = self
                .spotify
                .user_playlist_add_tracks(&user_id, &import_to, &[id], Some(0))
                .await
              {
                self.handle_error(anyhow!(e)).await;
              }
            }
          }
        } else {
          println!("No changes.");
        }
      }
    } else {
      self
        .handle_error(anyhow!(
          "'import_from' playlist ({}) does not exist.",
          import_from
        ))
        .await;
      return;
    }
  }

  async fn playlist_fork(&mut self, user_id: String, mut playlist_id: String, import_dir: PathBuf) {
    // Get playlist info from id
    // Copy all info for making
    // make the new playlist
    // import to the new playlist

    let playlist_req = self
      .spotify
      .user_playlist(
        user_id.as_str(),
        Some(playlist_id.as_mut_str()),
        None,
        Some(Country::UnitedStates),
      )
      .await;

    if playlist_req.is_err() {
      self
        .handle_error(anyhow!("playlist ({}) does not exist.", playlist_id))
        .await;
      return;
    }
    let playlist = playlist_req.ok().unwrap();

    let forked_response = self
      .spotify
      .user_playlist_create(
        user_id.as_str(),
        &playlist.name,
        playlist.public,
        Some(playlist.description),
      )
      .await;

    if let Err(e) = forked_response {
      self.handle_error(anyhow!(e)).await;
      return;
    }

    let forked = forked_response.unwrap();

    println!(
      "Forking '{}' into playlist id {}...",
      playlist.name, forked.id
    );

    // Need to add copied playlist picture but it comes with the updated rspotify
    let forked_dir = import_dir.join(forked.id.to_owned());

    self
      .playlist_import(
        user_id,
        playlist_id.to_owned(),
        forked.id.to_owned(),
        forked_dir.join(playlist_id),
      )
      .await;
  }

  async fn playlists_update(&mut self, user_id: String, import_dir: PathBuf) {
    // go through each imported file and do the import

    // Directories in import dir
    let imported_dirs = fs::read_dir(import_dir.to_owned());
    if let Err(e) = imported_dirs {
      self.handle_error(anyhow!(e)).await;
      return;
    }

    // Getting all ids of user followed playlists
    let user_playlists = self
      .spotify
      .current_user_playlists(self.large_search_limit, None)
      .await;
    let mut ids = HashSet::new();
    for pl in user_playlists.unwrap().items {
      ids.insert(pl.id.to_owned());
    }

    println!("Updating playlists...\n");

    // Deleting any imports that are not in user playlists list
    let mut playlists_to_delete = Vec::new();

    // for each dir in the imported_dirs
    for playlist in imported_dirs.unwrap() {
      let playlist_id = playlist
        .as_ref()
        .unwrap()
        .file_name()
        .into_string()
        .unwrap();

      let playlist_req = self
        .spotify
        .user_playlist(
          user_id.as_str(),
          Some(playlist_id.to_owned().as_mut_str()),
          None,
          Some(Country::UnitedStates),
        )
        .await;

      // If playlist request fails or the playlist is not in user followed playlists
      // add it to vec that will be used to delete playlists
      if playlist_req.is_err() {
        playlists_to_delete.push(playlist.unwrap());
        continue;
      }

      let playlist_r = playlist_req.unwrap();

      if !ids.contains(&playlist_r.id) {
        playlists_to_delete.push(playlist.unwrap());
        continue;
      }
      println!("Updating {}...", playlist_r.name);

      // Files in import dirs
      let imported_playlists = fs::read_dir(playlist.unwrap().path());
      let mut imports_to_delete = Vec::new();

      for imported in imported_playlists.unwrap() {
        let imported_id = imported
          .as_ref()
          .unwrap()
          .file_name()
          .into_string()
          .unwrap();
        let imported_req = self
          .spotify
          .user_playlist(
            user_id.as_str(),
            Some(imported_id.to_owned().as_mut_str()),
            None,
            Some(Country::UnitedStates),
          )
          .await;

        // If import fails, delete import
        if imported_req.is_err() {
          imports_to_delete.push(imported.unwrap());
          continue;
        }

        println!("Importing {}", imported_req.unwrap().name);

        // Use playlist dir and import file name to import again, updating the playlist
        self
          .playlist_import(
            user_id.to_owned(),
            imported_id.to_owned(),
            playlist_id.to_owned(),
            imported.unwrap().path(),
          )
          .await;

        println!("");
      }

      for import in imports_to_delete.iter() {
        if let Err(e) = fs::remove_file(import.path()) {
          self.handle_error(anyhow!(e)).await;
        }
      }
    }

    for playlist in playlists_to_delete.iter() {
      if let Err(e) = fs::remove_dir_all(playlist.path()) {
        self.handle_error(anyhow!(e)).await;
      }
    }
  }
}
