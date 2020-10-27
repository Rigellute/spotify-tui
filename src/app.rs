use super::user_config::UserConfig;
use crate::{
  network::IoEvent,
  paging::{MadeForYouPlaylist, NewScrollableResultPages, SavedArtist},
};
use anyhow::anyhow;
use rspotify::{
  model::{
    album::{FullAlbum, SavedAlbum, SimplifiedAlbum},
    artist::FullArtist,
    audio::AudioAnalysis,
    context::CurrentlyPlaybackContext,
    device::DevicePayload,
    page::{CursorBasedPage, Page},
    playing::PlayHistory,
    playlist::{PlaylistTrack, SimplifiedPlaylist},
    show::{SimplifiedEpisode, SimplifiedShow},
    track::{FullTrack, SavedTrack, SimplifiedTrack},
    user::PrivateUser,
    PlayingItem,
  },
  senum::Country,
};
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{
  cmp::{max, min},
  collections::HashSet,
  time::{Instant, SystemTime},
};
use tui::layout::Rect;

use clipboard::{ClipboardContext, ClipboardProvider};

pub const LIBRARY_OPTIONS: [&str; 6] = [
  "Made For You",
  "Recently Played",
  "Liked Songs",
  "Albums",
  "Artists",
  "Podcasts",
];

const DEFAULT_ROUTE: Route = Route {
  id: RouteId::Home,
  active_block: ActiveBlock::Empty,
  hovered_block: ActiveBlock::Library,
};

#[derive(Clone)]
pub struct UIViewWindow {
  pub height: usize,
  pub start_index: usize,
}

pub enum TableUIHeight {
  EpisodeTable(UIViewWindow),
  ArtistTable(UIViewWindow),
  SavedAlbumsView(UIViewWindow),
}

#[derive(Clone)]
//#[deprecated(
//since = "0.22.0",
//note = "This struct is going away in favor of NewScrollableResultPages (which will be renamed once this is gone)"
//)]
pub struct ScrollableResultPages<T> {
  index: usize,
  pub pages: Vec<T>,
}

#[derive(Default)]
pub struct SpotifyResultAndSelectedIndex<T> {
  pub index: usize,
  pub result: T,
}

#[derive(Clone)]
pub struct Library {
  pub selected_index: usize,
  pub saved_tracks: NewScrollableResultPages<SavedTrack>,
  pub made_for_you_playlists: NewScrollableResultPages<MadeForYouPlaylist>,
  pub saved_albums: NewScrollableResultPages<SavedAlbum>,
  pub saved_artists: NewScrollableResultPages<SavedArtist>,
}
impl Default for Library {
  fn default() -> Self {
    Self {
      selected_index: 0,
      saved_tracks: NewScrollableResultPages::new(),
      made_for_you_playlists: NewScrollableResultPages::new(),
      saved_albums: NewScrollableResultPages::new(),
      saved_artists: NewScrollableResultPages::new(),
    }
  }
}

#[derive(PartialEq, Debug)]
pub enum SearchResultBlock {
  AlbumSearch,
  SongSearch,
  ArtistSearch,
  PlaylistSearch,
  ShowSearch,
  Empty,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ArtistBlock {
  TopTracks,
  Albums,
  RelatedArtists,
  Empty,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DialogContext {
  PlaylistWindow,
  PlaylistSearch,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
  Analysis,
  PlayBar,
  AlbumTracks,
  AlbumList,
  ArtistBlock,
  Empty,
  Error,
  HelpMenu,
  Home,
  Input,
  Library,
  MyPlaylists,
  Podcasts,
  EpisodeTable,
  RecentlyPlayed,
  SearchResultBlock,
  SelectDevice,
  TrackTable,
  MadeForYou,
  Artists,
  BasicView,
  Dialog(DialogContext),
}

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
  Analysis,
  AlbumTracks,
  AlbumList,
  Artist,
  BasicView,
  Error,
  Home,
  RecentlyPlayed,
  Search,
  SelectedDevice,
  TrackTable,
  MadeForYou,
  Artists,
  Podcasts,
  PodcastEpisodes,
  Recommendations,
}

#[derive(Debug)]
pub struct Route {
  pub id: RouteId,
  pub active_block: ActiveBlock,
  pub hovered_block: ActiveBlock,
}

// Is it possible to compose enums?
#[derive(PartialEq, Debug)]
pub enum TrackTableContext {
  MyPlaylists,
  AlbumSearch,
  PlaylistSearch,
  SavedTracks,
  RecommendedTracks,
  MadeForYou,
}

// Is it possible to compose enums?
#[derive(Clone, PartialEq, Debug, Copy)]
pub enum AlbumTableContext {
  Simplified,
  Full,
}

#[derive(Clone, PartialEq, Debug)]
pub enum RecommendationsContext {
  Artist,
  Song,
}

pub struct SearchResult {
  pub albums: Option<Page<SimplifiedAlbum>>,
  pub artists: Option<Page<FullArtist>>,
  pub playlists: Option<Page<SimplifiedPlaylist>>,
  pub tracks: Option<Page<FullTrack>>,
  pub shows: Option<Page<SimplifiedShow>>,
  pub selected_album_index: Option<usize>,
  pub selected_artists_index: Option<usize>,
  pub selected_playlists_index: Option<usize>,
  pub selected_tracks_index: Option<usize>,
  pub selected_shows_index: Option<usize>,
  pub hovered_block: SearchResultBlock,
  pub selected_block: SearchResultBlock,
}

#[derive(Default)]
pub struct TrackTable {
  pub tracks: Vec<FullTrack>,
  pub selected_index: usize,
  pub context: Option<TrackTableContext>,
}

pub struct EpisodeTable {
  pub show_id: Option<String>,
  pub episodes: NewScrollableResultPages<SimplifiedEpisode>,
  pub reversed: bool,
}

impl Default for EpisodeTable {
  fn default() -> Self {
    Self {
      show_id: Default::default(),
      episodes: NewScrollableResultPages::new(),
      reversed: Default::default(),
    }
  }
}

#[derive(Clone)]
pub struct SelectedAlbum {
  pub album: SimplifiedAlbum,
  pub tracks: Page<SimplifiedTrack>,
  pub selected_index: usize,
}

#[derive(Clone)]
pub struct SelectedFullAlbum {
  pub album: FullAlbum,
  pub selected_index: usize,
}

#[derive(Clone)]
pub struct Artist {
  pub artist_name: String,
  pub albums: Page<SimplifiedAlbum>,
  pub related_artists: Vec<FullArtist>,
  pub top_tracks: Vec<FullTrack>,
  pub selected_album_index: usize,
  pub selected_related_artist_index: usize,
  pub selected_top_track_index: usize,
  pub artist_hovered_block: ArtistBlock,
  pub artist_selected_block: ArtistBlock,
}

pub struct App {
  pub instant_since_last_current_playback_poll: Instant,
  navigation_stack: Vec<Route>,
  pub audio_analysis: Option<AudioAnalysis>,
  pub home_scroll: u16,
  pub user_config: UserConfig,
  pub artist: Option<Artist>,
  pub album_table_context: AlbumTableContext,
  pub saved_album_tracks_index: usize,
  pub api_error: String,
  pub current_playback_context: Option<CurrentlyPlaybackContext>,
  pub devices: Option<DevicePayload>,
  // Inputs:
  // input is the string for input;
  // input_idx is the index of the cursor in terms of character;
  // input_cursor_position is the sum of the width of charaters preceding the cursor.
  // Reason for this complication is due to non-ASCII characters, they may
  // take more than 1 bytes to store and more than 1 character width to display.
  pub input: Vec<char>,
  pub input_idx: usize,
  pub input_cursor_position: u16,
  pub liked_song_ids_set: HashSet<String>,
  pub followed_artist_ids_set: HashSet<String>,
  pub saved_album_ids_set: HashSet<String>,
  pub large_search_limit: u32,
  pub library: Library,
  pub playlist_offset: u32,
  pub made_for_you_offset: u32,
  pub playlist_tracks: Option<Page<PlaylistTrack>>,
  pub made_for_you_tracks: Option<Page<PlaylistTrack>>,
  pub playlists: Option<Page<SimplifiedPlaylist>>,
  pub recently_played: SpotifyResultAndSelectedIndex<Option<CursorBasedPage<PlayHistory>>>,
  pub recommended_tracks: Vec<FullTrack>,
  pub recommendations_seed: String,
  pub recommendations_context: Option<RecommendationsContext>,
  pub search_results: SearchResult,
  pub selected_album_simplified: Option<SelectedAlbum>,
  pub selected_album_full: Option<SelectedFullAlbum>,
  pub selected_device_index: Option<usize>,
  pub selected_playlist_index: Option<usize>,
  pub size: Rect,
  pub small_search_limit: u32,
  pub song_progress_ms: u128,
  pub track_table: TrackTable,
  pub episode_table: EpisodeTable,
  pub user: Option<PrivateUser>,
  pub clipboard_context: Option<ClipboardContext>,
  pub help_docs_size: u32,
  pub help_menu_page: u32,
  pub help_menu_max_lines: u32,
  pub help_menu_offset: u32,
  pub is_loading: bool,
  io_tx: Option<Sender<IoEvent>>,
  pub ui_tx: Sender<TableUIHeight>,
  pub ui_rx: Receiver<TableUIHeight>,
  pub is_fetching_current_playback: bool,
  pub spotify_token_expiry: SystemTime,
  pub dialog: Option<String>,
  pub confirm: bool,
}

impl Default for App {
  fn default() -> Self {
    let (ui_tx, ui_rx) = channel();
    App {
      audio_analysis: None,
      album_table_context: AlbumTableContext::Full,
      artist: None,
      user_config: UserConfig::new(),
      saved_album_tracks_index: 0,
      recently_played: Default::default(),
      size: Rect::default(),
      selected_album_simplified: None,
      selected_album_full: None,
      home_scroll: 0,
      library: Default::default(),
      liked_song_ids_set: HashSet::new(),
      followed_artist_ids_set: HashSet::new(),
      saved_album_ids_set: HashSet::new(),
      navigation_stack: vec![DEFAULT_ROUTE],
      large_search_limit: 20,
      small_search_limit: 4,
      api_error: String::new(),
      current_playback_context: None,
      devices: None,
      input: vec![],
      input_idx: 0,
      input_cursor_position: 0,
      playlist_offset: 0,
      made_for_you_offset: 0,
      playlist_tracks: None,
      made_for_you_tracks: None,
      playlists: None,
      recommended_tracks: vec![],
      recommendations_context: None,
      recommendations_seed: "".to_string(),
      search_results: SearchResult {
        hovered_block: SearchResultBlock::SongSearch,
        selected_block: SearchResultBlock::Empty,
        albums: None,
        artists: None,
        playlists: None,
        shows: None,
        selected_album_index: None,
        selected_artists_index: None,
        selected_playlists_index: None,
        selected_tracks_index: None,
        selected_shows_index: None,
        tracks: None,
      },
      song_progress_ms: 0,
      selected_device_index: None,
      selected_playlist_index: None,
      track_table: Default::default(),
      episode_table: Default::default(),
      user: None,
      instant_since_last_current_playback_poll: Instant::now(),
      clipboard_context: clipboard::ClipboardProvider::new().ok(),
      help_docs_size: 0,
      help_menu_page: 0,
      help_menu_max_lines: 0,
      help_menu_offset: 0,
      is_loading: false,
      io_tx: None,
      ui_tx,
      ui_rx,
      is_fetching_current_playback: false,
      spotify_token_expiry: SystemTime::now(),
      dialog: None,
      confirm: false,
    }
  }
}

impl App {
  pub fn new(
    io_tx: Sender<IoEvent>,
    user_config: UserConfig,
    spotify_token_expiry: SystemTime,
  ) -> App {
    App {
      io_tx: Some(io_tx),
      user_config,
      spotify_token_expiry,
      ..App::default()
    }
  }

  // Send a network event to the network thread
  pub fn dispatch(&self, action: IoEvent) {
    if let Some(io_tx) = &self.io_tx {
      if let Err(e) = io_tx.send(action) {
        println!("Error from dispatch {}", e);
        // TODO: handle error
      };
    }
  }

  fn poll_current_playback(&mut self) {
    // Poll every 5 seconds
    let poll_interval_ms = 5_000;

    let elapsed = self
      .instant_since_last_current_playback_poll
      .elapsed()
      .as_millis();

    if !self.is_fetching_current_playback && elapsed >= poll_interval_ms {
      self.is_fetching_current_playback = true;
      self.dispatch(IoEvent::GetCurrentPlayback);
    }
  }

  pub fn update_on_tick(&mut self) {
    self.poll_current_playback();
    if let Some(CurrentlyPlaybackContext {
      item: Some(item),
      progress_ms: Some(progress_ms),
      is_playing: true,
      ..
    }) = &self.current_playback_context
    {
      let elapsed = self
        .instant_since_last_current_playback_poll
        .elapsed()
        .as_millis()
        + u128::from(*progress_ms);

      let duration_ms = match item {
        PlayingItem::Track(track) => track.duration_ms,
        PlayingItem::Episode(episode) => episode.duration_ms,
      };

      if elapsed < u128::from(duration_ms) {
        self.song_progress_ms = elapsed;
      } else {
        self.song_progress_ms = duration_ms.into();
      }
    }

    while let Ok(ui_height) = self.ui_rx.try_recv() {
      match ui_height {
        TableUIHeight::EpisodeTable(window) => {
          self.episode_table.episodes.ui_view_height = Some(window);
        }
        TableUIHeight::ArtistTable(window) => {
          self.library.saved_artists.ui_view_height = Some(window);
        }
        TableUIHeight::SavedAlbumsView(window) => {
          self.library.saved_albums.ui_view_height = Some(window);
        }
      }
    }
  }

  pub fn seek_forwards(&mut self) {
    if let Some(CurrentlyPlaybackContext {
      item: Some(item), ..
    }) = &self.current_playback_context
    {
      let duration_ms = match item {
        PlayingItem::Track(track) => track.duration_ms,
        PlayingItem::Episode(episode) => episode.duration_ms,
      };

      let event = if duration_ms - self.song_progress_ms as u32
        > self.user_config.behavior.seek_milliseconds
      {
        IoEvent::Seek(self.song_progress_ms as u32 + self.user_config.behavior.seek_milliseconds)
      } else {
        IoEvent::NextTrack
      };

      self.dispatch(event);
    }
  }

  pub fn seek_backwards(&mut self) {
    let new_progress = if self.song_progress_ms as u32 > self.user_config.behavior.seek_milliseconds
    {
      self.song_progress_ms as u32 - self.user_config.behavior.seek_milliseconds
    } else {
      0u32
    };
    self.dispatch(IoEvent::Seek(new_progress));
  }

  pub fn get_recommendations_for_seed(
    &self,
    seed_artists: Option<Vec<String>>,
    seed_tracks: Option<Vec<String>>,
    first_track: Option<FullTrack>,
  ) {
    let user_country = self.get_user_country();
    self.dispatch(IoEvent::GetRecommendationsForSeed(
      seed_artists,
      seed_tracks,
      Box::new(first_track),
      user_country,
    ));
  }

  pub fn get_recommendations_for_track_id(&self, id: String) {
    let user_country = self.get_user_country();
    self.dispatch(IoEvent::GetRecommendationsForTrackId(id, user_country));
  }

  pub fn increase_volume(&mut self) {
    if let Some(context) = self.current_playback_context.clone() {
      let current_volume = context.device.volume_percent as u8;
      let next_volume = min(
        current_volume + self.user_config.behavior.volume_increment,
        100,
      );

      if next_volume != current_volume {
        self.dispatch(IoEvent::ChangeVolume(next_volume));
      }
    }
  }

  pub fn decrease_volume(&mut self) {
    if let Some(context) = self.current_playback_context.clone() {
      let current_volume = context.device.volume_percent as i8;
      let next_volume = max(
        current_volume - self.user_config.behavior.volume_increment as i8,
        0,
      );

      if next_volume != current_volume {
        self.dispatch(IoEvent::ChangeVolume(next_volume as u8));
      }
    }
  }

  pub fn handle_error(&mut self, e: anyhow::Error) {
    self.push_navigation_stack(RouteId::Error, ActiveBlock::Error);
    self.api_error = e.to_string();
  }

  pub fn toggle_playback(&mut self) {
    if let Some(CurrentlyPlaybackContext {
      is_playing: true, ..
    }) = &self.current_playback_context
    {
      self.dispatch(IoEvent::PausePlayback);
    } else {
      // When no offset or uris are passed, spotify will resume current playback
      self.dispatch(IoEvent::StartPlayback(None, None, None));
    }
  }

  pub fn previous_track(&mut self) {
    if self.song_progress_ms >= 3_000 {
      self.dispatch(IoEvent::Seek(0));
    } else {
      self.dispatch(IoEvent::PreviousTrack);
    }
  }

  // The navigation_stack actually only controls the large block to the right of `library` and
  // `playlists`
  pub fn push_navigation_stack(&mut self, next_route_id: RouteId, next_active_block: ActiveBlock) {
    self.navigation_stack.push(Route {
      id: next_route_id,
      active_block: next_active_block,
      hovered_block: next_active_block,
    });
  }

  pub fn pop_navigation_stack(&mut self) -> Option<Route> {
    if self.navigation_stack.len() == 1 {
      None
    } else {
      self.navigation_stack.pop()
    }
  }

  pub fn get_current_route(&self) -> &Route {
    // if for some reason there is no route return the default
    self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
  }

  fn get_current_route_mut(&mut self) -> &mut Route {
    self.navigation_stack.last_mut().unwrap()
  }

  pub fn set_current_route_state(
    &mut self,
    active_block: Option<ActiveBlock>,
    hovered_block: Option<ActiveBlock>,
  ) {
    let mut current_route = self.get_current_route_mut();
    if let Some(active_block) = active_block {
      current_route.active_block = active_block;
    }
    if let Some(hovered_block) = hovered_block {
      current_route.hovered_block = hovered_block;
    }
  }

  pub fn copy_song_url(&mut self) {
    let clipboard = match &mut self.clipboard_context {
      Some(ctx) => ctx,
      None => return,
    };

    if let Some(CurrentlyPlaybackContext {
      item: Some(item), ..
    }) = &self.current_playback_context
    {
      match item {
        PlayingItem::Track(track) => {
          if let Err(e) = clipboard.set_contents(format!(
            "https://open.spotify.com/track/{}",
            track.id.to_owned().unwrap_or_default()
          )) {
            self.handle_error(anyhow!("failed to set clipboard content: {}", e));
          }
        }
        PlayingItem::Episode(episode) => {
          if let Err(e) = clipboard.set_contents(format!(
            "https://open.spotify.com/episode/{}",
            episode.id.to_owned()
          )) {
            self.handle_error(anyhow!("failed to set clipboard content: {}", e));
          }
        }
      }
    }
  }

  pub fn copy_album_url(&mut self) {
    let clipboard = match &mut self.clipboard_context {
      Some(ctx) => ctx,
      None => return,
    };

    if let Some(CurrentlyPlaybackContext {
      item: Some(item), ..
    }) = &self.current_playback_context
    {
      match item {
        PlayingItem::Track(track) => {
          if let Err(e) = clipboard.set_contents(format!(
            "https://open.spotify.com/album/{}",
            track.id.to_owned().unwrap_or_default()
          )) {
            self.handle_error(anyhow!("failed to set clipboard content: {}", e));
          }
        }
        PlayingItem::Episode(episode) => {
          if let Err(e) = clipboard.set_contents(format!(
            "https://open.spotify.com/show/{}",
            episode.show.id.to_owned()
          )) {
            self.handle_error(anyhow!("failed to set clipboard content: {}", e));
          }
        }
      }
    }
  }

  pub fn shuffle(&mut self) {
    if let Some(context) = &self.current_playback_context.clone() {
      self.dispatch(IoEvent::Shuffle(context.shuffle_state));
    };
  }

  pub fn current_user_saved_album_delete(&mut self, block: ActiveBlock) {
    match block {
      ActiveBlock::SearchResultBlock => {
        if let Some(albums) = &self.search_results.albums {
          if let Some(selected_index) = self.search_results.selected_album_index {
            let selected_album = &albums.items[selected_index];
            if let Some(album_id) = selected_album.id.clone() {
              self.dispatch(IoEvent::CurrentUserSavedAlbumDelete(album_id));
            }
          }
        }
      }
      ActiveBlock::AlbumList => {
        if let Some(selected_album) = self.library.saved_albums.get_selected_item() {
          let album_id = selected_album.album.id.clone();
          self.dispatch(IoEvent::CurrentUserSavedAlbumDelete(album_id));
        }
      }
      ActiveBlock::ArtistBlock => {
        if let Some(artist) = &self.artist {
          if let Some(selected_album) = artist.albums.items.get(artist.selected_album_index) {
            if let Some(album_id) = selected_album.id.clone() {
              self.dispatch(IoEvent::CurrentUserSavedAlbumDelete(album_id));
            }
          }
        }
      }
      _ => (),
    }
  }

  pub fn current_user_saved_album_add(&mut self, block: ActiveBlock) {
    match block {
      ActiveBlock::SearchResultBlock => {
        if let Some(albums) = &self.search_results.albums {
          if let Some(selected_index) = self.search_results.selected_album_index {
            let selected_album = &albums.items[selected_index];
            if let Some(album_id) = selected_album.id.clone() {
              self.dispatch(IoEvent::CurrentUserSavedAlbumAdd(album_id));
            }
          }
        }
      }
      ActiveBlock::ArtistBlock => {
        if let Some(artist) = &self.artist {
          if let Some(selected_album) = artist.albums.items.get(artist.selected_album_index) {
            if let Some(album_id) = selected_album.id.clone() {
              self.dispatch(IoEvent::CurrentUserSavedAlbumAdd(album_id));
            }
          }
        }
      }
      _ => (),
    }
  }

  pub fn user_unfollow_artists(&mut self, block: ActiveBlock) {
    match block {
      ActiveBlock::SearchResultBlock => {
        if let Some(artists) = &self.search_results.artists {
          if let Some(selected_index) = self.search_results.selected_artists_index {
            let selected_artist: &FullArtist = &artists.items[selected_index];
            let artist_id = selected_artist.id.clone();
            self.dispatch(IoEvent::UserUnfollowArtists(vec![artist_id]));
          }
        }
      }
      ActiveBlock::AlbumList => {
        if let Some(selected_artist) = self.library.saved_artists.get_selected_item() {
          let artist_id = selected_artist.id.clone();
          self.dispatch(IoEvent::UserUnfollowArtists(vec![artist_id]));
        }
      }
      ActiveBlock::ArtistBlock => {
        if let Some(artist) = &self.artist {
          let selected_artis = &artist.related_artists[artist.selected_related_artist_index];
          let artist_id = selected_artis.id.clone();
          self.dispatch(IoEvent::UserUnfollowArtists(vec![artist_id]));
        }
      }
      _ => (),
    };
  }

  pub fn user_follow_artists(&mut self, block: ActiveBlock) {
    match block {
      ActiveBlock::SearchResultBlock => {
        if let Some(artists) = &self.search_results.artists {
          if let Some(selected_index) = self.search_results.selected_artists_index {
            let selected_artist: &FullArtist = &artists.items[selected_index];
            let artist_id = selected_artist.id.clone();
            self.dispatch(IoEvent::UserFollowArtists(vec![artist_id]));
          }
        }
      }
      ActiveBlock::ArtistBlock => {
        if let Some(artist) = &self.artist {
          let selected_artis = &artist.related_artists[artist.selected_related_artist_index];
          let artist_id = selected_artis.id.clone();
          self.dispatch(IoEvent::UserFollowArtists(vec![artist_id]));
        }
      }
      _ => (),
    }
  }

  pub fn user_follow_playlist(&mut self) {
    if let SearchResult {
      playlists: Some(ref playlists),
      selected_playlists_index: Some(selected_index),
      ..
    } = self.search_results
    {
      let selected_playlist: &SimplifiedPlaylist = &playlists.items[selected_index];
      let selected_id = selected_playlist.id.clone();
      let selected_public = selected_playlist.public;
      let selected_owner_id = selected_playlist.owner.id.clone();
      self.dispatch(IoEvent::UserFollowPlaylist(
        selected_owner_id,
        selected_id,
        selected_public,
      ));
    }
  }

  pub fn user_unfollow_playlist(&mut self) {
    if let (Some(playlists), Some(selected_index), Some(user)) =
      (&self.playlists, self.selected_playlist_index, &self.user)
    {
      let selected_playlist = &playlists.items[selected_index];
      let selected_id = selected_playlist.id.clone();
      let user_id = user.id.clone();
      self.dispatch(IoEvent::UserUnfollowPlaylist(user_id, selected_id))
    }
  }

  pub fn user_unfollow_playlist_search_result(&mut self) {
    if let (Some(playlists), Some(selected_index), Some(user)) = (
      &self.search_results.playlists,
      self.search_results.selected_playlists_index,
      &self.user,
    ) {
      let selected_playlist = &playlists.items[selected_index];
      let selected_id = selected_playlist.id.clone();
      let user_id = user.id.clone();
      self.dispatch(IoEvent::UserUnfollowPlaylist(user_id, selected_id))
    }
  }

  pub fn user_follow_show(&mut self) {
    unimplemented!();
  }

  pub fn user_unfollow_show(&mut self) {
    unimplemented!();
  }

  pub fn get_made_for_you(&mut self) {
    // TODO: replace searches when relevant endpoint is added
    const DISCOVER_WEEKLY: &str = "Discover Weekly";
    const RELEASE_RADAR: &str = "Release Radar";
    const ON_REPEAT: &str = "On Repeat";
    const REPEAT_REWIND: &str = "Repeat Rewind";
    const YOUR_TOP_SONGS: &str = "Your Top Songs";
    const BEST_OF_THE_DECADE: &str = "Best of the Decade For You";

    if self.library.made_for_you_playlists.items.is_empty() {
      // We shouldn't be fetching all the results immediately - only load the data when the
      // user selects the playlist
      self.made_for_you_search_and_add(DISCOVER_WEEKLY);
      self.made_for_you_search_and_add(RELEASE_RADAR);
      self.made_for_you_search_and_add(ON_REPEAT);
      self.made_for_you_search_and_add(REPEAT_REWIND);
      self.made_for_you_search_and_add(YOUR_TOP_SONGS);
      self.made_for_you_search_and_add(BEST_OF_THE_DECADE);
    }
  }

  fn made_for_you_search_and_add(&mut self, search_string: &str) {
    let user_country = self.get_user_country();
    self.dispatch(IoEvent::MadeForYouSearchAndAdd(
      search_string.to_string(),
      user_country,
    ));
  }

  pub fn get_audio_analysis(&mut self) {
    if let Some(CurrentlyPlaybackContext {
      item: Some(item), ..
    }) = &self.current_playback_context
    {
      match item {
        PlayingItem::Track(track) => {
          if self.get_current_route().id != RouteId::Analysis {
            let uri = track.uri.clone();
            self.dispatch(IoEvent::GetAudioAnalysis(uri));
            self.push_navigation_stack(RouteId::Analysis, ActiveBlock::Analysis);
          }
        }
        PlayingItem::Episode(_episode) => {
          // No audio analysis available for podcast uris, so just default to the empty analysis
          // view to avoid a 400 error code
          self.push_navigation_stack(RouteId::Analysis, ActiveBlock::Analysis);
        }
      }
    }
  }

  pub fn repeat(&mut self) {
    if let Some(context) = &self.current_playback_context.clone() {
      self.dispatch(IoEvent::Repeat(context.repeat_state));
    }
  }

  pub fn get_artist(&self, artist_id: String, input_artist_name: String) {
    let user_country = self.get_user_country();
    self.dispatch(IoEvent::GetArtist(
      artist_id,
      input_artist_name,
      user_country,
    ));
  }

  pub fn get_user_country(&self) -> Option<Country> {
    self
      .user
      .to_owned()
      .and_then(|user| Country::from_str(&user.country.unwrap_or_else(|| "".to_string())).ok())
  }

  pub fn calculate_help_menu_offset(&mut self) {
    let old_offset = self.help_menu_offset;

    if self.help_menu_max_lines < self.help_docs_size {
      self.help_menu_offset = self.help_menu_page * self.help_menu_max_lines;
    }
    if self.help_menu_offset > self.help_docs_size {
      self.help_menu_offset = old_offset;
      self.help_menu_page -= 1;
    }
  }
}
