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
    AlbumSearch,
    ApiError,
    ArtistSearch,
    HelpMenu,
    Home,
    Input,
    MyPlaylists,
    PlaylistSearch,
    SelectDevice,
    SongSearch,
    SongTable,
}

pub struct App {
    pub active_block: ActiveBlock,
    pub api_error: String,
    pub current_playing_song: Option<FullTrack>,
    pub device_id: Option<String>,
    pub devices: Option<DevicePayload>,
    pub input: String,
    pub playlist_tracks: Vec<PlaylistTrack>,
    pub playlists: Option<Page<SimplifiedPlaylist>>,
    pub searched_albums: Option<SearchAlbums>,
    pub searched_artists: Option<SearchArtists>,
    pub searched_playlists: Option<SearchPlaylists>,
    pub searched_tracks: Option<SearchTracks>,
    pub select_song_index: usize,
    pub selected_device_index: Option<usize>,
    pub selected_playlist_index: Option<usize>,
    pub songs_for_table: Vec<FullTrack>,
    pub spotify: Option<Spotify>,
}

impl App {
    pub fn new() -> App {
        App {
            active_block: ActiveBlock::MyPlaylists,
            api_error: String::new(),
            current_playing_song: None,
            device_id: None,
            devices: None,
            input: String::new(),
            playlist_tracks: vec![],
            playlists: None,
            searched_albums: None,
            searched_artists: None,
            searched_playlists: None,
            searched_tracks: None,
            select_song_index: 0,
            selected_device_index: None,
            selected_playlist_index: None,
            songs_for_table: vec![],
            spotify: None,
        }
    }
}
