use rspotify::spotify::client::Spotify;
use rspotify::spotify::model::device::DevicePayload;
use rspotify::spotify::model::page::Page;
use rspotify::spotify::model::playlist::{PlaylistTrack, SimplifiedPlaylist};
use rspotify::spotify::model::search::{
    SearchAlbums, SearchArtists, SearchPlaylists, SearchTracks,
};
use rspotify::spotify::model::track::FullTrack;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(PartialEq, Debug)]
pub enum EventLoop {
    Exit,
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
    pub small_search_limit: u32,
    pub active_block: ActiveBlock,
    pub api_error: String,
    pub current_playing_song: Option<FullTrack>,
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
    pub spotify: Option<Spotify>,
    path_to_cached_device_id: PathBuf,
}

impl App {
    pub fn new() -> App {
        App {
            large_search_limit: 20,
            small_search_limit: 4,
            active_block: ActiveBlock::MyPlaylists,
            api_error: String::new(),
            current_playing_song: None,
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
            selected_device_index: None,
            selected_playlist_index: None,
            songs_for_table: vec![],
            spotify: None,
            path_to_cached_device_id: PathBuf::from(".cached_device_id.txt"),
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
}
