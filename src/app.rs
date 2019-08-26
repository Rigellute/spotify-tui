use rspotify::spotify::client::Spotify;
use rspotify::spotify::model::device::DevicePayload;
use rspotify::spotify::model::page::Page;
use rspotify::spotify::model::playlist::{PlaylistTrack, SimplifiedPlaylist};
use rspotify::spotify::model::search::{
    SearchAlbums, SearchArtists, SearchPlaylists, SearchTracks,
};
use rspotify::spotify::model::track::FullTrack;

pub const LIMIT: u32 = 20;

#[derive(PartialEq, Debug)]
pub enum EventLoop {
    Exit,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    Input,
    MyPlaylists,
    SongTable,
    HelpMenu,
    ApiError,
    SelectDevice,
    PlaylistSearch,
    SongSearch,
    ArtistSearch,
    AlbumSearch,
}

pub struct App {
    pub active_block: ActiveBlock,
    pub devices: Option<DevicePayload>,
    pub device_id: Option<String>,
    pub current_playing_song: Option<FullTrack>,
    pub input: String,
    pub playlists: Option<Page<SimplifiedPlaylist>>,
    pub playlist_tracks: Vec<PlaylistTrack>,
    pub searched_tracks: Option<SearchTracks>,
    pub searched_albums: Option<SearchAlbums>,
    pub searched_artists: Option<SearchArtists>,
    pub searched_playlists: Option<SearchPlaylists>,
    pub spotify: Option<Spotify>,
    pub songs_for_table: Vec<FullTrack>,
    pub selected_playlist_index: Option<usize>,
    pub select_song_index: usize,
    pub api_error: String,
    pub selected_device_index: Option<usize>,
}

impl App {
    pub fn new() -> App {
        App {
            active_block: ActiveBlock::MyPlaylists,
            devices: None,
            device_id: None,
            api_error: String::new(),
            current_playing_song: None,
            input: String::new(),
            playlists: None,
            playlist_tracks: vec![],
            searched_tracks: None,
            spotify: None,
            songs_for_table: vec![],
            selected_playlist_index: None,
            select_song_index: 0,
            selected_device_index: None,
            searched_albums: None,
            searched_artists: None,
            searched_playlists: None,
        }
    }
}
