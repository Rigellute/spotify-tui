use crate::app::App;
use rspotify::{
    client::Spotify,
    oauth2::{SpotifyClientCredentials, SpotifyOAuth, TokenInfo},
    util::get_token,
};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum IoEvent {
    Poll,
    RefreshAuthentication,
    GetPlaylists,
}

pub fn get_spotify(token_info: TokenInfo) -> (Spotify, Instant) {
    let token_expiry = Instant::now()
        + Duration::from_secs(token_info.expires_in.into())
        // Set 10 seconds early
        - Duration::from_secs(10);

    let client_credential = SpotifyClientCredentials::default()
        .token_info(token_info)
        .build();

    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    (spotify, token_expiry)
}

pub struct Network<'a> {
    oauth: &'a mut SpotifyOAuth,
    config_port: u16,
    spotify: Spotify,
    spotify_token_expiry: Instant,
}

impl<'a> Network<'a> {
    pub fn new(
        oauth: &'a mut SpotifyOAuth,
        config_port: u16,
        spotify: Spotify,
        spotify_token_expiry: Instant,
    ) -> Self {
        Network {
            oauth,
            config_port,
            spotify,
            spotify_token_expiry,
        }
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent, app: &Arc<Mutex<App>>) {
        match io_event {
            IoEvent::Poll => {}
            IoEvent::RefreshAuthentication => {
                if let Some(new_token_info) = get_token(self.oauth).await {
                    let (new_spotify, new_token_expiry) = get_spotify(new_token_info);
                    self.spotify = new_spotify;
                    self.spotify_token_expiry = new_token_expiry;
                } else {
                    println!("\nFailed to refresh authentication token");
                    // TODO panic!
                }
            }
            IoEvent::GetPlaylists => {
                let mut app = app.lock().await;
                let playlists = self
                    .spotify
                    .current_user_playlists(app.large_search_limit, None)
                    .await;

                match playlists {
                    Ok(p) => {
                        app.playlists = Some(p);
                        // Select the first playlist
                        app.selected_playlist_index = Some(0);
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                };

                // app.get_user();
            }
        };
    }
}
