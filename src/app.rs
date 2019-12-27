use super::{config::ClientConfig, user_config::UserConfig};
use failure::{err_msg, format_err};
use rspotify::spotify::{
    client::Spotify,
    model::{
        album::{FullAlbum, SavedAlbum, SimplifiedAlbum},
        artist::FullArtist,
        context::FullPlayingContext,
        device::DevicePayload,
        offset::{for_position, Offset},
        page::{CursorBasedPage, Page},
        playing::PlayHistory,
        playlist::{PlaylistTrack, SimplifiedPlaylist},
        search::{SearchAlbums, SearchArtists, SearchPlaylists, SearchTracks},
        track::{FullTrack, SavedTrack, SimplifiedTrack},
        user::PrivateUser,
    },
    senum::{Country, RepeatState},
};
use std::{
    cmp::{max, min},
    collections::HashSet,
    time::Instant,
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
pub struct ScrollableResultPages<T> {
    index: usize,
    pages: Vec<T>,
}

impl<T> ScrollableResultPages<T> {
    pub fn new() -> ScrollableResultPages<T> {
        ScrollableResultPages {
            index: 0,
            pages: vec![],
        }
    }

    pub fn get_results(&self, at_index: Option<usize>) -> Option<&T> {
        match at_index {
            Some(index) => self.pages.get(index),
            None => self.pages.get(self.index),
        }
    }

    pub fn add_pages(&mut self, new_pages: T) {
        self.pages.push(new_pages);
        // Whenever a new page is added, set the active index to the end of the vector
        self.index = self.pages.len() - 1;
    }
}

#[derive(Default)]
pub struct SpotifyResultAndSelectedIndex<T> {
    pub index: usize,
    pub result: T,
}

#[derive(Clone)]
pub struct Library {
    pub selected_index: usize,
    pub saved_tracks: ScrollableResultPages<Page<SavedTrack>>,
    pub saved_albums: ScrollableResultPages<Page<SavedAlbum>>,
    pub saved_artists: ScrollableResultPages<CursorBasedPage<FullArtist>>,
}

#[derive(Clone)]
pub struct PlaybackParams {
    context_uri: Option<String>,
    uris: Option<Vec<String>>,
    offset: Option<Offset>,
}

#[derive(PartialEq, Debug)]
pub enum SearchResultBlock {
    AlbumSearch,
    SongSearch,
    ArtistSearch,
    PlaylistSearch,
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
pub enum ActiveBlock {
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
    RecentlyPlayed,
    SearchResultBlock,
    SelectDevice,
    TrackTable,
    MadeForYou,
    Artists,
}

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    AlbumTracks,
    AlbumList,
    Artist,
    Error,
    Home,
    RecentlyPlayed,
    Search,
    SelectedDevice,
    TrackTable,
    MadeForYou,
    Artists,
    Podcasts,
}

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
}

#[derive(Clone, PartialEq, Debug)]
pub enum AlbumTableContext {
    Simplified,
    Full,
}

pub struct SearchResult {
    pub albums: Option<SearchAlbums>,
    pub artists: Option<SearchArtists>,
    pub playlists: Option<SearchPlaylists>,
    pub selected_album_index: Option<usize>,
    pub selected_artists_index: Option<usize>,
    pub selected_playlists_index: Option<usize>,
    pub selected_tracks_index: Option<usize>,
    pub tracks: Option<SearchTracks>,
    pub hovered_block: SearchResultBlock,
    pub selected_block: SearchResultBlock,
}

#[derive(Default)]
pub struct TrackTable {
    pub tracks: Vec<FullTrack>,
    pub selected_index: usize,
    pub context: Option<TrackTableContext>,
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
    instant_since_last_current_playback_poll: Instant,
    navigation_stack: Vec<Route>,
    pub home_scroll: u16,
    pub client_config: ClientConfig,
    pub user_config: UserConfig,
    pub artists: Vec<FullArtist>,
    pub artist: Option<Artist>,
    pub album_table_context: AlbumTableContext,
    pub saved_album_tracks_index: usize,
    pub api_error: String,
    pub current_playback_context: Option<FullPlayingContext>,
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
    pub large_search_limit: u32,
    pub library: Library,
    pub playlist_offset: u32,
    pub playback_params: PlaybackParams,
    pub playlist_tracks: Vec<PlaylistTrack>,
    pub playlists: Option<Page<SimplifiedPlaylist>>,
    pub recently_played: SpotifyResultAndSelectedIndex<Option<CursorBasedPage<PlayHistory>>>,
    pub search_results: SearchResult,
    pub selected_album: Option<SelectedAlbum>,
    pub selected_album_full: Option<SelectedFullAlbum>,
    pub selected_device_index: Option<usize>,
    pub selected_playlist_index: Option<usize>,
    pub size: Rect,
    pub small_search_limit: u32,
    pub song_progress_ms: u128,
    pub spotify: Option<Spotify>,
    pub track_table: TrackTable,
    pub user: Option<PrivateUser>,
    pub album_list_index: usize,
    pub artists_list_index: usize,
    pub clipboard_context: Option<ClipboardContext>,
}

impl App {
    pub fn new() -> App {
        App {
            album_table_context: AlbumTableContext::Full,
            album_list_index: 0,
            artists_list_index: 0,
            artists: vec![],
            artist: None,
            user_config: UserConfig::new(),
            client_config: Default::default(),
            saved_album_tracks_index: 0,
            recently_played: Default::default(),
            size: Rect::default(),
            selected_album: None,
            selected_album_full: None,
            home_scroll: 0,
            library: Library {
                saved_tracks: ScrollableResultPages::new(),
                saved_albums: ScrollableResultPages::new(),
                saved_artists: ScrollableResultPages::new(),
                selected_index: 0,
            },
            liked_song_ids_set: HashSet::new(),
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
            playlist_tracks: vec![],
            playlists: None,
            search_results: SearchResult {
                hovered_block: SearchResultBlock::SongSearch,
                selected_block: SearchResultBlock::Empty,
                albums: None,
                artists: None,
                playlists: None,
                selected_album_index: None,
                selected_artists_index: None,
                selected_playlists_index: None,
                selected_tracks_index: None,
                tracks: None,
            },
            song_progress_ms: 0,
            selected_device_index: None,
            selected_playlist_index: None,
            spotify: None,
            track_table: Default::default(),
            playback_params: PlaybackParams {
                context_uri: None,
                uris: None,
                offset: None,
            },
            user: None,
            instant_since_last_current_playback_poll: Instant::now(),
            clipboard_context: None,
        }
    }

    pub fn get_user(&mut self) {
        if let Some(spotify) = &self.spotify {
            match spotify.current_user() {
                Ok(user) => {
                    self.user = Some(user);
                }
                Err(e) => {
                    self.handle_error(e);
                }
            }
        }
    }

    pub fn handle_get_devices(&mut self) {
        if let Some(spotify) = &self.spotify {
            if let Ok(result) = spotify.device() {
                self.push_navigation_stack(RouteId::SelectedDevice, ActiveBlock::SelectDevice);
                if !result.devices.is_empty() {
                    self.devices = Some(result);
                    // Select the first device in the list
                    self.selected_device_index = Some(0);
                }
            }
        }
    }

    pub fn get_current_playback(&mut self) {
        if let Some(spotify) = &self.spotify {
            let context = spotify.current_playback(None);
            if let Ok(ctx) = context {
                if let Some(c) = ctx {
                    self.current_playback_context = Some(c.clone());
                    self.instant_since_last_current_playback_poll = Instant::now();

                    if let Some(track) = c.item {
                        if let Some(track_id) = track.id {
                            self.current_user_saved_tracks_contains(vec![track_id]);
                        }
                    }
                }
            };
        }
    }

    pub fn current_user_saved_tracks_contains(&mut self, ids: Vec<String>) {
        if let Some(spotify) = &self.spotify {
            match spotify.current_user_saved_tracks_contains(&ids) {
                Ok(is_saved_vec) => {
                    for (i, id) in ids.iter().enumerate() {
                        if let Some(is_liked) = is_saved_vec.get(i) {
                            if *is_liked {
                                self.liked_song_ids_set.insert(id.to_string());
                            } else {
                                // The song is not liked, so check if it should be removed
                                if self.liked_song_ids_set.contains(id) {
                                    self.liked_song_ids_set.remove(id);
                                }
                            }
                        };
                    }
                }
                Err(e) => {
                    self.handle_error(e);
                }
            }
        }
    }

    fn poll_current_playback(&mut self) {
        // Poll every 5 seconds
        let poll_interval_ms = 5_000;

        let elapsed = self
            .instant_since_last_current_playback_poll
            .elapsed()
            .as_millis();

        if elapsed >= poll_interval_ms {
            self.get_current_playback();
        }
    }

    pub fn update_on_tick(&mut self) {
        self.poll_current_playback();
        if let Some(current_playback_context) = &self.current_playback_context {
            if let (Some(track), Some(progress_ms)) = (
                &current_playback_context.item,
                current_playback_context.progress_ms,
            ) {
                if current_playback_context.is_playing {
                    let elapsed = self
                        .instant_since_last_current_playback_poll
                        .elapsed()
                        .as_millis()
                        + u128::from(progress_ms);

                    if elapsed < u128::from(track.duration_ms) {
                        self.song_progress_ms = elapsed;
                    } else {
                        self.song_progress_ms = track.duration_ms.into();
                    }
                }
            }
        }
    }

    fn seek(&mut self, position_ms: u32) {
        if let (Some(spotify), Some(device_id)) = (&self.spotify, &self.client_config.device_id) {
            match spotify.seek_track(position_ms, Some(device_id.to_string())) {
                Ok(()) => {
                    self.get_current_playback();
                }
                Err(e) => {
                    self.handle_error(e);
                }
            };
        }
    }

    pub fn seek_forwards(&mut self) {
        if let Some(current_playback_context) = &self.current_playback_context {
            if let Some(track) = &current_playback_context.item {
                if track.duration_ms - self.song_progress_ms as u32
                    > self.user_config.behavior.seek_milliseconds
                {
                    self.seek(
                        self.song_progress_ms as u32 + self.user_config.behavior.seek_milliseconds,
                    );
                } else {
                    self.next_track();
                }
            }
        }
    }

    pub fn seek_backwards(&mut self) {
        let new_progress =
            if self.song_progress_ms as u32 > self.user_config.behavior.seek_milliseconds {
                self.song_progress_ms as u32 - self.user_config.behavior.seek_milliseconds
            } else {
                0u32
            };
        self.seek(new_progress);
    }

    pub fn pause_playback(&mut self) {
        if let (Some(spotify), Some(device_id)) = (&self.spotify, &self.client_config.device_id) {
            match spotify.pause_playback(Some(device_id.to_string())) {
                Ok(()) => {
                    self.get_current_playback();
                }
                Err(e) => {
                    self.handle_error(e);
                }
            };
        }
    }

    fn change_volume(&mut self, volume_percent: u8) {
        if let (Some(spotify), Some(device_id), Some(context)) = (
            &self.spotify,
            &self.client_config.device_id,
            &mut self.current_playback_context,
        ) {
            match spotify.volume(volume_percent, Some(device_id.to_string())) {
                Ok(()) => {
                    context.device.volume_percent = volume_percent.into();
                }
                Err(e) => {
                    self.handle_error(e);
                }
            };
        }
    }

    pub fn increase_volume(&mut self) {
        if let Some(context) = self.current_playback_context.clone() {
            let current_volume = context.device.volume_percent as u8;
            let next_volume = min(current_volume + 10, 100);

            if next_volume != current_volume {
                self.change_volume(next_volume);
            }
        }
    }

    pub fn decrease_volume(&mut self) {
        if let Some(context) = self.current_playback_context.clone() {
            let current_volume = context.device.volume_percent as i8;
            let next_volume = max(current_volume - 10, 0);

            if next_volume != current_volume {
                self.change_volume(next_volume as u8);
            }
        }
    }

    pub fn handle_error(&mut self, e: failure::Error) {
        self.push_navigation_stack(RouteId::Error, ActiveBlock::Error);
        self.api_error = e.to_string();
    }

    pub fn toggle_playback(&mut self) {
        if let Some(current_playback_context) = &self.current_playback_context {
            if current_playback_context.is_playing {
                self.pause_playback();
            } else {
                // When no offset or uris are passed, spotify will resume current playback
                self.start_playback(None, None, None);
            }
        }
    }

    pub fn next_track(&mut self) {
        if let (Some(spotify), Some(device_id)) = (&self.spotify, &self.client_config.device_id) {
            match spotify.next_track(Some(device_id.to_string())) {
                Ok(()) => {
                    self.get_current_playback();
                }
                Err(e) => {
                    self.handle_error(e);
                }
            };
        }
    }

    pub fn previous_track(&mut self) {
        if let (Some(spotify), Some(device_id)) = (&self.spotify, &self.client_config.device_id) {
            match spotify.previous_track(Some(device_id.to_string())) {
                Ok(()) => {
                    self.get_current_playback();
                }
                Err(e) => {
                    self.handle_error(e);
                }
            };
        }
    }

    pub fn start_playback(
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
            Some(device_id) => match &self.spotify {
                Some(spotify) => spotify.start_playback(
                    Some(device_id.to_string()),
                    context_uri.clone(),
                    uris.clone(),
                    offset.clone(),
                    None,
                ),
                None => Err(err_msg("Spotify is not ready to be used".to_string())),
            },
            None => Err(err_msg("No device_id selected")),
        };

        match result {
            Ok(()) => {
                self.get_current_playback();
                self.song_progress_ms = 0;
                self.playback_params = PlaybackParams {
                    context_uri,
                    uris,
                    offset,
                }
            }
            Err(e) => {
                self.handle_error(e);
            }
        }
    }

    pub fn get_playlist_tracks(&mut self, playlist_id: String) {
        match &self.spotify {
            Some(spotify) => {
                if let Ok(playlist_tracks) = spotify.user_playlist_tracks(
                    "spotify",
                    &playlist_id,
                    None,
                    Some(self.large_search_limit),
                    Some(self.playlist_offset),
                    None,
                ) {
                    self.set_playlist_tracks_to_table(&playlist_tracks);

                    self.playlist_tracks = playlist_tracks.items;
                    if self.get_current_route().id != RouteId::TrackTable {
                        self.push_navigation_stack(RouteId::TrackTable, ActiveBlock::TrackTable);
                    };
                };
            }
            None => {}
        }
    }

    // The navigation_stack actually only controls the large block to the right of `library` and
    // `playlists`
    pub fn push_navigation_stack(
        &mut self,
        next_route_id: RouteId,
        next_active_block: ActiveBlock,
    ) {
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
        match self.navigation_stack.last() {
            Some(route) => route,
            None => &DEFAULT_ROUTE, // if for some reason there is no route return the default
        }
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

        if let Some(FullPlayingContext {
            item: Some(FullTrack { id: Some(id), .. }),
            ..
        }) = &self.current_playback_context
        {
            if let Err(e) = clipboard.set_contents(format!("https://open.spotify.com/track/{}", id))
            {
                self.handle_error(format_err!("failed to set clipboard content: {}", e));
            }
        }
    }

    fn set_saved_tracks_to_table(&mut self, saved_track_page: &Page<SavedTrack>) {
        self.set_tracks_to_table(
            saved_track_page
                .items
                .clone()
                .into_iter()
                .map(|item| item.track)
                .collect::<Vec<FullTrack>>(),
        );
    }

    fn set_playlist_tracks_to_table(&mut self, playlist_track_page: &Page<PlaylistTrack>) {
        self.set_tracks_to_table(
            playlist_track_page
                .items
                .clone()
                .into_iter()
                .map(|item| item.track)
                .collect::<Vec<FullTrack>>(),
        );
    }

    pub fn set_tracks_to_table(&mut self, tracks: Vec<FullTrack>) {
        self.track_table.tracks = tracks.clone();

        self.current_user_saved_tracks_contains(
            tracks
                .into_iter()
                .filter_map(|item| item.id)
                .collect::<Vec<String>>(),
        );
    }

    pub fn get_current_user_saved_tracks(&mut self, offset: Option<u32>) {
        if let Some(spotify) = &self.spotify {
            match spotify.current_user_saved_tracks(self.large_search_limit, offset) {
                Ok(saved_tracks) => {
                    self.set_saved_tracks_to_table(&saved_tracks);

                    self.library.saved_tracks.add_pages(saved_tracks);
                    self.track_table.context = Some(TrackTableContext::SavedTracks);
                }
                Err(e) => {
                    self.handle_error(e);
                }
            }
        }
    }

    pub fn get_current_user_saved_tracks_next(&mut self) {
        // Before fetching the next tracks, check if we have already fetched them
        match self
            .library
            .saved_tracks
            .get_results(Some(self.library.saved_tracks.index + 1))
            .cloned()
        {
            Some(saved_tracks) => {
                self.set_saved_tracks_to_table(&saved_tracks);
                self.library.saved_tracks.index += 1
            }
            None => {
                if let Some(saved_tracks) = &self.library.saved_tracks.get_results(None) {
                    let offset = Some(saved_tracks.offset + saved_tracks.limit);
                    self.get_current_user_saved_tracks(offset);
                }
            }
        }
    }

    pub fn get_current_user_saved_tracks_previous(&mut self) {
        if self.library.saved_tracks.index > 0 {
            self.library.saved_tracks.index -= 1;
        }

        if let Some(saved_tracks) = &self.library.saved_tracks.get_results(None).cloned() {
            self.set_saved_tracks_to_table(&saved_tracks);
        }
    }

    pub fn get_album_tracks(&mut self, album: SimplifiedAlbum) {
        if let Some(album_id) = &album.id {
            if let Some(spotify) = &self.spotify {
                match spotify.album_track(&album_id.clone(), self.large_search_limit, 0) {
                    Ok(tracks) => {
                        self.selected_album = Some(SelectedAlbum {
                            album,
                            tracks: tracks.clone(),
                            selected_index: 0,
                        });

                        self.current_user_saved_tracks_contains(
                            tracks
                                .items
                                .into_iter()
                                .filter_map(|item| item.id)
                                .collect::<Vec<String>>(),
                        );

                        self.album_table_context = AlbumTableContext::Simplified;
                        self.push_navigation_stack(RouteId::AlbumTracks, ActiveBlock::AlbumTracks);
                    }
                    Err(e) => {
                        self.handle_error(e);
                    }
                }
            }
        }
    }

    pub fn toggle_save_track(&mut self, track_id: String) {
        if let Some(spotify) = &self.spotify {
            match spotify.current_user_saved_tracks_contains(&[track_id.clone()]) {
                Ok(saved) => {
                    if saved.first() == Some(&true) {
                        match spotify.current_user_saved_tracks_delete(&[track_id.clone()]) {
                            Ok(()) => {
                                self.liked_song_ids_set.remove(&track_id);
                            }
                            Err(e) => {
                                self.handle_error(e);
                            }
                        }
                    } else {
                        match spotify.current_user_saved_tracks_add(&[track_id.clone()]) {
                            Ok(()) => {
                                // TODO: This should ideally use the same logic as `self.current_user_saved_tracks_contains`
                                self.liked_song_ids_set.insert(track_id);
                            }
                            Err(e) => {
                                self.handle_error(e);
                            }
                        }
                    }
                }
                Err(e) => {
                    self.handle_error(e);
                }
            }
        };
    }

    pub fn shuffle(&mut self) {
        if let (Some(spotify), Some(context)) = (&self.spotify, &mut self.current_playback_context)
        {
            match spotify.shuffle(!context.shuffle_state, self.client_config.device_id.clone()) {
                Ok(()) => {
                    // Update the UI eagerly (otherwise the UI will wait until the next 5 second interval
                    // due to polling playback context)
                    context.shuffle_state = !context.shuffle_state;
                }
                Err(e) => {
                    self.handle_error(e);
                }
            }
        };
    }

    pub fn repeat(&mut self) {
        if let (Some(spotify), Some(context)) = (&self.spotify, &mut self.current_playback_context)
        {
            let next_repeat_state = match context.repeat_state {
                RepeatState::Off => RepeatState::Context,
                RepeatState::Context => RepeatState::Track,
                RepeatState::Track => RepeatState::Off,
            };
            match spotify.repeat(next_repeat_state, self.client_config.device_id.clone()) {
                Ok(()) => {
                    // Update the UI eagerly (otherwise the UI will wait until the next 5 second interval
                    // due to polling playback context)
                    context.repeat_state = next_repeat_state;
                }
                Err(e) => {
                    self.handle_error(e);
                }
            }
        }
    }

    pub fn get_artist(&mut self, artist_id: &str, artist_name: &str) {
        if let (Some(spotify), Some(user)) = (&self.spotify, &self.user.to_owned()) {
            let user_country =
                Country::from_str(&user.country.to_owned().unwrap_or_else(|| "".to_string()));
            let albums = spotify.artist_albums(
                artist_id,
                None,
                Country::from_str(&user.country.to_owned().unwrap_or_else(|| "".to_string())),
                Some(self.large_search_limit),
                Some(0),
            );
            let top_tracks = spotify.artist_top_tracks(artist_id, user_country);
            let related_artist = spotify.artist_related_artists(artist_id);

            if let (Ok(albums), Ok(top_tracks), Ok(related_artist)) =
                (albums, top_tracks, related_artist)
            {
                self.artist = Some(Artist {
                    artist_name: artist_name.to_owned(),
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
    }

    pub fn get_artists(&mut self, offset: Option<String>) {
        if let Some(spotify) = &self.spotify {
            match spotify.current_user_followed_artists(self.large_search_limit, offset) {
                Ok(saved_artists) => {
                    self.artists = saved_artists.artists.items.to_owned();
                    self.library.saved_artists.add_pages(saved_artists.artists);
                }
                Err(e) => {
                    self.handle_error(e);
                }
            };
        };
    }

    pub fn get_current_user_saved_albums(&mut self, offset: Option<u32>) {
        if let Some(spotify) = &self.spotify {
            match spotify.current_user_saved_albums(self.large_search_limit, offset) {
                Ok(saved_albums) => {
                    // not to show a blank page
                    if !saved_albums.items.is_empty() {
                        self.library.saved_albums.add_pages(saved_albums);
                    }
                }
                Err(e) => {
                    self.handle_error(e);
                }
            }
        }
    }

    pub fn get_current_user_saved_albums_next(&mut self) {
        match self
            .library
            .saved_albums
            .get_results(Some(self.library.saved_albums.index + 1))
            .cloned()
        {
            Some(_) => self.library.saved_albums.index += 1,
            None => {
                if let Some(saved_albums) = &self.library.saved_albums.get_results(None) {
                    let offset = Some(saved_albums.offset + saved_albums.limit);
                    self.get_current_user_saved_albums(offset);
                }
            }
        }
    }

    pub fn get_current_user_saved_albums_previous(&mut self) {
        if self.library.saved_albums.index > 0 {
            self.library.saved_albums.index -= 1;
        }
    }

    pub fn current_user_saved_album_delete(&mut self) {
        if let Some(albums) = self.library.saved_albums.get_results(None) {
            if let Some(selected_album) = albums.items.get(self.album_list_index) {
                if let Some(spotify) = &mut self.spotify {
                    let album_id = &selected_album.album.id;
                    match spotify.current_user_saved_albums_delete(&[album_id.to_owned()]) {
                        Ok(_) => self.get_current_user_saved_albums(None),
                        Err(e) => self.handle_error(e),
                    }
                }
            }
        }
    }

    pub fn current_user_saved_album_add(&mut self) {
        if let Some(albums) = &self.search_results.albums {
            if let Some(selected_index) = self.search_results.selected_album_index {
                if let Some(spotify) = &self.spotify {
                    let selected_album = &albums.albums.items[selected_index];
                    if let Some(artist_id) = &selected_album.id {
                        if let Err(e) =
                            spotify.current_user_saved_albums_add(&[artist_id.to_owned()])
                        {
                            self.handle_error(e);
                        }
                    }
                }
            }
        }
    }

    pub fn user_unfollow_artists(&mut self) {
        if let Some(artists) = self.library.saved_artists.get_results(None) {
            if let Some(selected_artist) = artists.items.get(self.artists_list_index) {
                if let Some(spotify) = &mut self.spotify {
                    let artist_id = &selected_artist.id;
                    match spotify.user_unfollow_artists(&[artist_id.to_owned()]) {
                        Ok(_) => self.get_artists(None),
                        Err(e) => self.handle_error(e),
                    }
                }
            }
        }
    }

    pub fn user_follow_artists(&mut self) {
        if let Some(artists) = &self.search_results.artists {
            if let Some(selected_index) = self.search_results.selected_artists_index {
                if let Some(spotify) = &mut self.spotify {
                    let selected_artist: &FullArtist = &artists.artists.items[selected_index];
                    let artist_id = &selected_artist.id;
                    if let Err(e) = spotify.user_follow_artists(&[artist_id.to_owned()]) {
                        self.handle_error(e);
                    }
                }
            }
        }
    }

    pub fn user_follow_playlists(&mut self) {
        if let (Some(playlists), Some(selected_index), Some(spotify)) = (
            &self.search_results.playlists,
            self.search_results.selected_playlists_index,
            &self.spotify,
        ) {
            let selected_playlist: &SimplifiedPlaylist = &playlists.playlists.items[selected_index];
            let selected_id = &selected_playlist.id;
            let selected_public = selected_playlist.public;
            let selected_owner_id = &selected_playlist.owner.id;
            if let Err(e) = spotify.user_playlist_follow_playlist(
                &selected_owner_id,
                &selected_id,
                selected_public,
            ) {
                self.handle_error(e);
            }
        }
    }

    pub fn user_unfollow_playlists(&mut self) {
        if let (Some(playlists), Some(selected_index), Some(user), Some(spotify)) = (
            &self.playlists,
            self.selected_playlist_index,
            &self.user,
            &self.spotify,
        ) {
            let selected_playlist = &playlists.items[selected_index];
            let selected_id = &selected_playlist.id;
            if let Err(e) = spotify.user_playlist_unfollow(&user.id, &selected_id) {
                self.handle_error(e);
            }
        }
    }
}
