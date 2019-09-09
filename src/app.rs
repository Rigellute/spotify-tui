use failure::err_msg;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::model::album::{SavedAlbum, SimplifiedAlbum};
use rspotify::spotify::model::context::FullPlayingContext;
use rspotify::spotify::model::device::DevicePayload;
use rspotify::spotify::model::offset::for_position;
use rspotify::spotify::model::page::Page;
use rspotify::spotify::model::playlist::{PlaylistTrack, SimplifiedPlaylist};
use rspotify::spotify::model::search::{
    SearchAlbums, SearchArtists, SearchPlaylists, SearchTracks,
};
use rspotify::spotify::model::track::{FullTrack, SavedTrack, SimplifiedTrack};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

pub const LIBRARY_OPTIONS: [&str; 6] = [
    "Made For You",
    "Recently Played",
    "Liked Songs",
    "Albums",
    "Artists",
    "Podcasts",
];

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

    pub fn get_saved_tracks(&self, at_index: Option<usize>) -> Option<&T> {
        match at_index {
            Some(index) => self.pages.get(index),
            None => self.pages.get(self.index),
        }
    }

    pub fn add_ages(&mut self, new_pages: T) {
        self.pages.push(new_pages);
        // Whenever a new page is added, set the active index to the end of the vector
        self.index = self.pages.len() - 1;
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Clone)]
pub struct Library {
    pub selected_index: usize,
    pub saved_tracks: ScrollableResultPages<Page<SavedTrack>>,
    pub saved_albums: Option<Page<SavedAlbum>>,
}

#[derive(Clone)]
pub struct PlaybackParams {
    context_uri: Option<String>,
    uris: Option<Vec<String>>,
    offset: Option<usize>,
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
    Album,
    Empty,
    Error,
    HelpMenu,
    Home,
    Input,
    Library,
    MyPlaylists,
    SearchResultBlock,
    SelectDevice,
    SongTable,
    Artist,
}

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    Home,
    SelectedDevice,
    Error,
    HelpMenu,
    Search,
    Album,
    Artist,
    SongTable,
}

pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
    pub hovered_block: ActiveBlock,
}

// Is it possible to compose enums?
#[derive(PartialEq, Debug)]
pub enum SongTableContext {
    MyPlaylists,
    AlbumSearch,
    SongSearch,
    ArtistSearch,
    PlaylistSearch,
    SavedTracks,
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
    pub context: Option<SongTableContext>,
}

#[derive(Clone)]
pub struct SelectedAlbum {
    pub album: SimplifiedAlbum,
    pub tracks: Page<SimplifiedTrack>,
    pub selected_index: Option<usize>,
}

pub struct App {
    instant_since_last_current_playback_poll: Instant,
    navigation_stack: Vec<Route>,
    path_to_cached_device_id: PathBuf,
    pub api_error: String,
    pub current_playback_context: Option<FullPlayingContext>,
    pub device_id: Option<String>,
    pub devices: Option<DevicePayload>,
    pub input: String,
    pub input_cursor_position: u16,
    pub large_search_limit: u32,
    pub library: Library,
    pub playback_params: PlaybackParams,
    pub playlist_tracks: Vec<PlaylistTrack>,
    pub playlists: Option<Page<SimplifiedPlaylist>>,
    pub search_results: SearchResult,
    pub selected_album: Option<SelectedAlbum>,
    pub selected_device_index: Option<usize>,
    pub selected_playlist_index: Option<usize>,
    pub small_search_limit: u32,
    pub song_progress_ms: u128,
    pub spotify: Option<Spotify>,
    pub track_table: TrackTable,
}

impl App {
    pub fn new() -> App {
        App {
            selected_album: None,
            library: Library {
                saved_tracks: ScrollableResultPages::new(),
                saved_albums: None,
                selected_index: 0,
            },
            navigation_stack: vec![Route {
                id: RouteId::Home,
                active_block: ActiveBlock::Empty,
                hovered_block: ActiveBlock::Library,
            }],
            large_search_limit: 20,
            small_search_limit: 4,

            api_error: String::new(),
            current_playback_context: None,
            device_id: None,
            devices: None,
            input: String::new(),
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
            path_to_cached_device_id: PathBuf::from(".cached_device_id.txt"),
            instant_since_last_current_playback_poll: Instant::now(),
        }
    }

    // Perhaps this should be a yaml/json file for more cached options (e.g. locale data?)
    pub fn get_cached_device_token(&self) -> Result<String, failure::Error> {
        let input = fs::read_to_string(&self.path_to_cached_device_id)?;
        Ok(input)
    }

    pub fn set_cached_device_token(&self, device_token: String) -> Result<(), failure::Error> {
        let mut output = fs::File::create(&self.path_to_cached_device_id)?;
        write!(output, "{}", device_token)?;

        Ok(())
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
                    self.current_playback_context = Some(c);
                    self.instant_since_last_current_playback_poll = Instant::now();
                }
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

    pub fn pause_playback(&mut self) {
        if let (Some(spotify), Some(device_id)) = (&self.spotify, &self.device_id) {
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

    pub fn handle_error(&mut self, e: failure::Error) {
        self.push_navigation_stack(RouteId::Error, ActiveBlock::Error);
        self.api_error = e.to_string();
    }

    pub fn toggle_playback(&mut self) {
        if let Some(current_playback_context) = &self.current_playback_context {
            if current_playback_context.is_playing {
                self.pause_playback();
            } else {
                // Ideally I should be able to pass in the `progress_ms` to start_playback, but
                // this does not yet work https://github.com/ramsayleung/rspotify/issues/51
                if let Some(_progress_ms) = current_playback_context.progress_ms {
                    let PlaybackParams {
                        context_uri,
                        uris,
                        offset,
                    } = &self.playback_params.clone();

                    self.start_playback(context_uri.to_owned(), uris.to_owned(), offset.to_owned());
                }
            }
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
        } else {
            (uris, None)
        };

        let offset = match offset {
            Some(o) => o,
            None => 0,
        };

        let result = match &self.device_id {
            Some(device_id) => match &self.spotify {
                Some(spotify) => spotify.start_playback(
                    Some(device_id.to_string()),
                    context_uri.clone(),
                    uris.clone(),
                    for_position(offset as u32),
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
                    offset: Some(offset),
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
                    self.track_table.tracks = playlist_tracks
                        .items
                        .clone()
                        .into_iter()
                        .map(|item| item.track)
                        .collect::<Vec<FullTrack>>();

                    self.playlist_tracks = playlist_tracks.items;
                    self.push_navigation_stack(RouteId::SongTable, ActiveBlock::SongTable);
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
        self.navigation_stack.pop()
    }

    pub fn get_current_route(&self) -> &Route {
        // There should always be at least one route. But there must be better way of handling
        // this? `unwrap` seems too dangerous
        self.navigation_stack.last().unwrap()
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
        match (active_block, hovered_block) {
            (Some(active), Some(hovered)) => {
                current_route.active_block = active;
                current_route.hovered_block = hovered;
            }
            (Some(active), None) => {
                current_route.active_block = active;
            }
            (None, Some(hovered)) => {
                current_route.hovered_block = hovered;
            }
            (None, None) => {}
        }
    }

    pub fn get_current_user_saved_tracks(&mut self, offset: Option<u32>) {
        if let Some(spotify) = &self.spotify {
            match spotify.current_user_saved_tracks(self.large_search_limit, offset) {
                Ok(saved_tracks) => {
                    self.track_table.tracks = saved_tracks
                        .items
                        .clone()
                        .into_iter()
                        .map(|item| item.track)
                        .collect::<Vec<FullTrack>>();

                    self.library.saved_tracks.add_ages(saved_tracks);
                    self.track_table.context = Some(SongTableContext::SavedTracks);
                    self.push_navigation_stack(RouteId::SongTable, ActiveBlock::SongTable);
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
            .get_saved_tracks(Some(self.library.saved_tracks.index + 1))
        {
            Some(saved_tracks) => {
                self.track_table.tracks = saved_tracks
                    .items
                    .clone()
                    .into_iter()
                    .map(|item| item.track)
                    .collect::<Vec<FullTrack>>();
                self.library.saved_tracks.index += 1
            }
            None => {
                if let Some(saved_tracks) = &self.library.saved_tracks.get_saved_tracks(None) {
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

        if let Some(saved_tracks) = &self.library.saved_tracks.get_saved_tracks(None) {
            self.track_table.tracks = saved_tracks
                .items
                .clone()
                .into_iter()
                .map(|item| item.track)
                .collect::<Vec<FullTrack>>();
        }
    }
}
