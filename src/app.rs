use failure::err_msg;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::model::context::SimplifiedPlayingContext;
use rspotify::spotify::model::device::DevicePayload;
use rspotify::spotify::model::offset::for_position;
use rspotify::spotify::model::page::Page;
use rspotify::spotify::model::playlist::{PlaylistTrack, SimplifiedPlaylist};
use rspotify::spotify::model::search::{
    SearchAlbums, SearchArtists, SearchPlaylists, SearchTracks,
};
use rspotify::spotify::model::track::FullTrack;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Clone)]
pub struct PlaybackParams {
    context_uri: Option<String>,
    uris: Option<Vec<String>>,
    offset: Option<usize>,
}

#[derive(PartialEq, Debug)]
pub enum Routes {
    Search,
    Album(String /* album id */),
    Artist(String /* artist id */),
    TrackInfo(String /* track id*/),
    SongTable,
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
    Error,
    HelpMenu,
    Home,
    Input,
    MyPlaylists,
    SelectDevice,
    SearchResultBlock,
    SongTable,
}

// Is it possible to compose enums?
#[derive(PartialEq, Debug)]
pub enum SongTableContext {
    MyPlaylists,
    AlbumSearch,
    SongSearch,
    ArtistSearch,
    PlaylistSearch,
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

pub struct App {
    pub large_search_limit: u32,
    pub navigation_stack: Vec<Routes>,
    pub small_search_limit: u32,
    pub active_block: ActiveBlock,
    pub api_error: String,
    pub current_playing_context: Option<SimplifiedPlayingContext>,
    pub device_id: Option<String>,
    pub devices: Option<DevicePayload>,
    pub input: String,
    pub playlist_tracks: Vec<PlaylistTrack>,
    pub playlists: Option<Page<SimplifiedPlaylist>>,
    pub search_results: SearchResult,
    pub song_table_context: Option<SongTableContext>,
    pub select_song_index: usize,
    pub selected_device_index: Option<usize>,
    pub selected_playlist_index: Option<usize>,
    pub songs_for_table: Vec<FullTrack>,
    pub song_progress_ms: u128,
    pub spotify: Option<Spotify>,
    path_to_cached_device_id: PathBuf,
    instant_since_last_currently_playing_poll: Instant,
    pub playback_params: PlaybackParams,
}

impl App {
    pub fn new() -> App {
        App {
            large_search_limit: 20,
            navigation_stack: vec![],
            small_search_limit: 4,
            active_block: ActiveBlock::MyPlaylists,
            api_error: String::new(),
            current_playing_context: None,
            device_id: None,
            devices: None,
            input: String::new(),
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
            select_song_index: 0,
            song_table_context: None,
            song_progress_ms: 0,
            selected_device_index: None,
            selected_playlist_index: None,
            songs_for_table: vec![],
            spotify: None,
            playback_params: PlaybackParams {
                context_uri: None,
                uris: None,
                offset: None,
            },
            path_to_cached_device_id: PathBuf::from(".cached_device_id.txt"),
            instant_since_last_currently_playing_poll: Instant::now(),
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
                self.active_block = ActiveBlock::SelectDevice;
                if !result.devices.is_empty() {
                    self.devices = Some(result);
                    // Select the first device in the list
                    self.selected_device_index = Some(0);
                }
            }
        }
    }

    pub fn get_currently_playing(&mut self) {
        if let Some(spotify) = &self.spotify {
            let context = spotify.current_playing(None);
            if let Ok(ctx) = context {
                if let Some(c) = ctx {
                    self.current_playing_context = Some(c);
                    self.instant_since_last_currently_playing_poll = Instant::now();
                }
            };
        }
    }

    fn poll_currently_playing(&mut self) {
        // Poll every 5 seconds
        let poll_interval_ms = 5_000;

        let elapsed = self
            .instant_since_last_currently_playing_poll
            .elapsed()
            .as_millis();

        if elapsed >= poll_interval_ms {
            self.get_currently_playing();
        }
    }

    pub fn update_on_tick(&mut self) {
        self.poll_currently_playing();
        if let Some(current_playing_context) = &self.current_playing_context {
            if let (Some(track), Some(progress_ms)) = (
                &current_playing_context.item,
                current_playing_context.progress_ms,
            ) {
                if current_playing_context.is_playing {
                    let elapsed = self
                        .instant_since_last_currently_playing_poll
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

    pub fn get_current_route(&self) -> Option<&Routes> {
        self.navigation_stack.last()
    }

    pub fn pause_playback(&mut self) {
        if let (Some(spotify), Some(device_id)) = (&self.spotify, &self.device_id) {
            match spotify.pause_playback(Some(device_id.to_string())) {
                Ok(()) => {
                    self.get_currently_playing();
                }
                Err(e) => {
                    self.active_block = ActiveBlock::Error;
                    self.api_error = e.to_string();
                }
            };
        }
    }

    pub fn toggle_playback(&mut self) {
        if let Some(current_playing_context) = &self.current_playing_context {
            if current_playing_context.is_playing {
                self.pause_playback();
            } else {
                // Ideally I should be able to pass in the `progress_ms` to start_playback, but
                // this does not yet work https://github.com/ramsayleung/rspotify/issues/51
                if let Some(_progress_ms) = current_playing_context.progress_ms {
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
                self.get_currently_playing();
                self.playback_params = PlaybackParams {
                    context_uri: context_uri,
                    uris,
                    offset: Some(offset),
                }
            }
            Err(e) => {
                self.active_block = ActiveBlock::Error;
                self.api_error = e.to_string();
            }
        }
    }
}
