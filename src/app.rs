use super::config::ClientConfig;
use super::user_config::UserConfig;
use failure::err_msg;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::model::album::{FullAlbum, SavedAlbum, SimplifiedAlbum};
use rspotify::spotify::model::artist::FullArtist;
use rspotify::spotify::model::context::FullPlayingContext;
use rspotify::spotify::model::device::DevicePayload;
use rspotify::spotify::model::offset::for_position;
use rspotify::spotify::model::offset::Offset;
use rspotify::spotify::model::page::{CursorBasedPage, Page};
use rspotify::spotify::model::playing::PlayHistory;
use rspotify::spotify::model::playlist::{PlaylistTrack, SimplifiedPlaylist};
use rspotify::spotify::model::search::{
    SearchAlbums, SearchArtists, SearchPlaylists, SearchTracks,
};
use rspotify::spotify::model::track::{FullTrack, SavedTrack, SimplifiedTrack};
use rspotify::spotify::model::user::PrivateUser;
use rspotify::spotify::senum::{Country, RepeatState};
use std::collections::HashSet;
use std::time::Instant;
use tui::layout::Rect;

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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    PlayBar,
    AlbumTracks,
    AlbumList,
    Artist,
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
pub struct ArtistAlbums {
    pub artist_name: String,
    pub albums: Page<SimplifiedAlbum>,
    pub selected_index: usize,
}

pub struct App {
    instant_since_last_current_playback_poll: Instant,
    navigation_stack: Vec<Route>,
    pub home_scroll: u16,
    pub client_config: ClientConfig,
    pub user_config: UserConfig,
    pub artists: Vec<FullArtist>,
    pub artist_albums: Option<ArtistAlbums>,
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
}

impl App {
    pub fn new() -> App {
        App {
            album_table_context: AlbumTableContext::Full,
            album_list_index: 0,
            artists_list_index: 0,
            artists: vec![],
            artist_albums: None,
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
                if track.duration_ms - self.song_progress_ms as u32 > 5000 {
                    self.seek(self.song_progress_ms as u32 + 5000);
                } else {
                    self.next_track();
                }
            }
        }
    }

    pub fn seek_backwards(&mut self) {
        let new_progress = if self.song_progress_ms > 5000 {
            self.song_progress_ms as u32 - 5000
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
                    self.get_current_playback();
                }
                Err(e) => {
                    self.handle_error(e);
                }
            };
        }
    }

    pub fn increase_volume(&mut self) {
        if let Some(context) = self.current_playback_context.clone() {
            let next_volume = context.device.volume_percent as u8 + 10;
            if next_volume <= 100 {
                self.change_volume(next_volume);
            }
        }
    }

    pub fn decrease_volume(&mut self) {
        if let Some(context) = self.current_playback_context.clone() {
            let volume = context.device.volume_percent;
            if volume >= 10 {
                let next_volume = context.device.volume_percent as u8 - 10;
                self.change_volume(next_volume);
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
                    None,
                    None,
                ) {
                    self.set_playlist_tracks_to_table(&playlist_tracks);

                    self.playlist_tracks = playlist_tracks.items;
                    self.push_navigation_stack(RouteId::TrackTable, ActiveBlock::TrackTable);
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
                .clone()
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
                    self.push_navigation_stack(RouteId::TrackTable, ActiveBlock::TrackTable);
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

    pub fn get_artist_albums(&mut self, artist_id: &str, artist_name: &str) {
        if let (Some(spotify), Some(user)) = (&self.spotify, &self.user.to_owned()) {
            match spotify.artist_albums(
                artist_id,
                None,
                Country::from_str(&user.country.to_owned().unwrap_or_else(|| "".to_string())),
                Some(self.large_search_limit),
                Some(0),
            ) {
                Ok(result) => {
                    self.artist_albums = Some(ArtistAlbums {
                        artist_name: artist_name.to_owned(),
                        selected_index: 0,
                        albums: result,
                    });
                    self.push_navigation_stack(RouteId::Artist, ActiveBlock::Artist);
                }
                Err(e) => {
                    self.handle_error(e);
                }
            };
        };
    }

    pub fn get_artists(&mut self, offset: Option<String>) {
        if let Some(spotify) = &self.spotify {
            match spotify.current_user_followed_artists(self.large_search_limit, offset) {
                Ok(saved_artists) => {
                    self.artists = saved_artists.artists.items.to_owned();
                    self.library.saved_artists.add_pages(saved_artists.artists);
                    self.push_navigation_stack(RouteId::Artists, ActiveBlock::Artists);
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
                    if saved_albums.items.len() > 0 {
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
}
