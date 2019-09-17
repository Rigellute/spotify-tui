use super::util::get_help;
use failure::err_msg;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::model::album::{FullAlbum, SavedAlbum, SimplifiedAlbum};
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
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
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

#[derive(Serialize, Deserialize)]
pub struct HelpItem {
    pub key: String,
    pub desc: String,
}

#[derive(Serialize, Deserialize)]
pub struct Help {
    general: Vec<HelpItem>,
    selected_block: Vec<HelpItem>,
    search_input: Vec<HelpItem>,
    pagination: Vec<HelpItem>,
}

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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Clone)]
pub struct Library {
    pub selected_index: usize,
    pub saved_tracks: ScrollableResultPages<Page<SavedTrack>>,
    pub saved_albums: ScrollableResultPages<Page<SavedAlbum>>,
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
    RecentlyPlayed,
    SearchResultBlock,
    SelectDevice,
    TrackTable,
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
    pub context: Option<SongTableContext>,
}

#[derive(Clone)]
pub struct SelectedAlbum {
    pub album: SimplifiedAlbum,
    pub tracks: Page<SimplifiedTrack>,
    pub selected_index: Option<usize>,
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
    path_to_cached_device_id: PathBuf,
    pub artist_albums: Option<ArtistAlbums>,
    pub album_table_context: AlbumTableContext,
    pub saved_album_tracks_index: usize,
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
    pub recently_played: SpotifyResultAndSelectedIndex<Option<CursorBasedPage<PlayHistory>>>,
    pub search_results: SearchResult,
    pub selected_album: Option<SelectedAlbum>,
    pub selected_album_full: Option<SelectedFullAlbum>,
    pub selected_device_index: Option<usize>,
    pub selected_playlist_index: Option<usize>,
    pub size: Rect,
    pub help_rows: Vec<Vec<String>>,
    pub small_search_limit: u32,
    pub song_progress_ms: u128,
    pub spotify: Option<Spotify>,
    pub track_table: TrackTable,
    pub album_list_index: usize,
}

impl App {
    pub fn new() -> App {
        App {
            album_table_context: AlbumTableContext::Full,
            album_list_index: 0,
            artist_albums: None,
            saved_album_tracks_index: 0,
            recently_played: Default::default(),
            size: Rect::default(),
            selected_album: None,
            selected_album_full: None,
            library: Library {
                saved_tracks: ScrollableResultPages::new(),
                saved_albums: ScrollableResultPages::new(),
                selected_index: 0,
            },
            navigation_stack: vec![Route {
                id: RouteId::Home,
                active_block: ActiveBlock::Empty,
                hovered_block: ActiveBlock::Library,
            }],
            large_search_limit: 20,
            small_search_limit: 4,
            help_rows: vec![], // This needs to get loading in form the `help.yml` file
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

    pub fn load_help_items(&mut self) {
        if self.help_rows.is_empty() {
            match get_help() {
                Ok(help) => {
                    let mut general = help
                        .general
                        .iter()
                        .map(|row| {
                            vec![
                                "General".to_string(),
                                row.key.to_owned(),
                                row.desc.to_owned(),
                            ]
                        })
                        .collect::<Vec<Vec<String>>>();

                    let mut selected_block = help
                        .selected_block
                        .iter()
                        .map(|row| {
                            vec![
                                "Selected Block".to_string(),
                                row.key.to_owned(),
                                row.desc.to_owned(),
                            ]
                        })
                        .collect::<Vec<Vec<String>>>();

                    let mut search_input = help
                        .search_input
                        .iter()
                        .map(|row| {
                            vec![
                                "Search Input".to_string(),
                                row.key.to_owned(),
                                row.desc.to_owned(),
                            ]
                        })
                        .collect::<Vec<Vec<String>>>();

                    let mut pagination = help
                        .pagination
                        .iter()
                        .map(|row| {
                            vec![
                                "Pagination".to_string(),
                                row.key.to_owned(),
                                row.desc.to_owned(),
                            ]
                        })
                        .collect::<Vec<Vec<String>>>();

                    let mut help_rows = vec![];

                    help_rows.append(&mut general);
                    help_rows.append(&mut selected_block);
                    help_rows.append(&mut search_input);
                    help_rows.append(&mut pagination);

                    self.help_rows = help_rows;
                }
                Err(e) => {
                    self.handle_error(e);
                }
            }
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
                // When no offset or uris are passed, spotify will resume current playback
                self.start_playback(None, None, None);
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
        } else if uris.is_some() {
            (uris, None)
        } else {
            (None, None)
        };

        let offset = match offset {
            Some(o) => for_position(o as u32),
            None => None,
        };

        let result = match &self.device_id {
            Some(device_id) => match &self.spotify {
                Some(spotify) => spotify.start_playback(
                    Some(device_id.to_string()),
                    context_uri.clone(),
                    uris.clone(),
                    offset.clone(),
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
                    self.track_table.tracks = playlist_tracks
                        .items
                        .clone()
                        .into_iter()
                        .map(|item| item.track)
                        .collect::<Vec<FullTrack>>();

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

    fn set_saved_tracks_to_table(&mut self, saved_tracks: &Page<SavedTrack>) {
        self.track_table.tracks = saved_tracks
            .items
            .clone()
            .into_iter()
            .map(|item| item.track)
            .collect::<Vec<FullTrack>>();
    }

    pub fn get_current_user_saved_tracks(&mut self, offset: Option<u32>) {
        if let Some(spotify) = &self.spotify {
            match spotify.current_user_saved_tracks(self.large_search_limit, offset) {
                Ok(saved_tracks) => {
                    self.set_saved_tracks_to_table(&saved_tracks);

                    self.library.saved_tracks.add_pages(saved_tracks);
                    self.track_table.context = Some(SongTableContext::SavedTracks);
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
                            tracks,
                            selected_index: Some(0),
                        });

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

    pub fn save_tracks(&mut self, track_ids: Vec<String>) {
        if let Some(spotify) = &self.spotify {
            match spotify.current_user_saved_tracks_add(&track_ids) {
                Ok(()) => {}
                Err(e) => {
                    self.handle_error(e);
                }
            }
        };
    }

    pub fn shuffle(&mut self) {
        if let (Some(spotify), Some(context)) = (&self.spotify, &self.current_playback_context) {
            match spotify.shuffle(!context.shuffle_state, self.device_id.clone()) {
                Ok(()) => {}
                Err(e) => {
                    self.handle_error(e);
                }
            }
        };
    }

    pub fn get_artist_albums(&mut self, artist_id: &str, artist_name: &str) {
        if let Some(spotify) = &self.spotify {
            match spotify.artist_albums(
                artist_id,
                None,
                None,
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
}
