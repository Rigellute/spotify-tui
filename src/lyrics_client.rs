//! Client to Lyrics.ovh API endpoint
use chrono::prelude::*;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use reqwest::Method;
use reqwest::StatusCode;
use serde::de::Deserialize;
use serde_json::map::Map;
use serde_json::Value;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::string::String;

use super::model::album::{FullAlbum, FullAlbums, PageSimpliedAlbums, SavedAlbum, SimplifiedAlbum};
use super::model::artist::{CursorPageFullArtists, FullArtist, FullArtists};
use super::model::audio::{AudioAnalysis, AudioFeatures, AudioFeaturesPayload};
use super::model::category::PageCategory;
use super::model::context::{CurrentlyPlaybackContext, CurrentlyPlayingContext};
use super::model::cud_result::CUDResult;
use super::model::device::DevicePayload;
use super::model::page::{CursorBasedPage, Page};
use super::model::playing::{PlayHistory, Playing};
use super::model::playlist::{FeaturedPlaylists, FullPlaylist, PlaylistTrack, SimplifiedPlaylist};
use super::model::recommend::Recommendations;
use super::model::search::SearchResult;
use super::model::show::{
    FullEpisode, FullShow, SeveralEpisodes, SeversalSimplifiedShows, Show, SimplifiedEpisode,
};
use super::model::track::{FullTrack, FullTracks, SavedTrack, SimplifiedTrack};
use super::model::user::{PrivateUser, PublicUser};
use super::oauth2::SpotifyClientCredentials;
use super::senum::{
    AdditionalType, AlbumType, Country, IncludeExternal, RepeatState, SearchType, TimeRange, Type,
};
use super::util::convert_map_to_string;
lazy_static! {
    /// HTTP Client
    pub static ref CLIENT: Client = Client::new();
}


// new: Lyrics.ovh API object 
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LyricsOvh {
    pub prefix: String,
}
impl LyricsOvh {

    pub fn default() -> LyricsOvh {
        LyricsOvh {
            prefix: "https://api.lyrics.ovh/v1/",
        }
    }

    pub fn prefix(mut self, prefix: &str) -> LyricsOvh {
        self.prefix = prefix.to_owned();
        self
    }

    ///send get request
    async fn get(
        &self,
        url: &str,
        params: &mut HashMap<String, String>,
    ) -> Result<String, failure::Error> {
        if !params.is_empty() {
            let param: String = convert_map_to_string(params);
            let mut url_with_params = url.to_owned();
            url_with_params.push('?');
            url_with_params.push_str(&param);
            self.internal_call(Method::GET, &url_with_params, None)
                .await
        } else {
            self.internal_call(Method::GET, url, None).await
        }
    }

    async fn internal_call(
        &self,
        method: Method,
        url: &str,
        payload: Option<&Value>,
    ) -> Result<String, failure::Error> {
        let mut url: Cow<str> = url.into();
        if !url.starts_with("http") {
            url = ["https://api.lyrics.ovh/v1/", &url].concat().into();
        }

        let mut headers = HeaderMap::new();
        // headers.insert(AUTHORIZATION, self.auth_headers().await.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let response = {
            let builder = CLIENT.request(method, &url.into_owned()).headers(headers);
    
            // never add body i think
            // let builder = if let Some(json) = payload {
            //     builder.json(json)
            // } else {
            //     builder
            // };

            builder.send().await?
        };

        if response.status().is_success() {
            match response.text().await {
                Ok(text) => Ok(text),
                Err(e) => Err(failure::err_msg(format!(
                    "Error getting text out of response {}",
                    e
                ))),
            }
        } else {
            Err(failure::Error::from(
                ApiError::from_response(response).await,
            ))
        }
    }

    ///[get album](https://developer.spotify.com/web-api/get-album/)
    ///returns a single album given the album's ID, URIs or URL
    ///Parameters:
    ///- album_id - the album ID, URI or URL
    pub async fn lyrics(&self, artist: &str, song: &str) -> Result<String, failure::Error> {
        // let trid = self.get_id(Type::Album, album_id);
        let url = format!("/{}/{}", artist, song);
        let result = self.get(&url, &mut HashMap::new()).await?;
        // self.convert_result::<FullAlbum>(&result)
        result
    }

    // copypasted from spotify convert result
    // pub fn convert_result<'a, T: Deserialize<'a>>(
    //     &self,
    //     input: &'a str,
    // ) -> Result<T, failure::Error> {
    //     let result = serde_json::from_str::<T>(input).map_err(|e| {
    //         format_err!(
    //             "convert result failed, reason: {:?}; content: [{:?}]",
    //             e,
    //             input
    //         )
    //     })?;
    //     Ok(result)
    // }

}








// /// Describes API errors
// #[derive(Debug, Deserialize)]
// pub enum ApiError {
//     Unauthorized,
//     RateLimited(Option<usize>),
//     #[serde(alias = "error")]
//     RegularError {
//         status: u16,
//         message: String,
//     },
//     #[serde(alias = "error")]
//     PlayerError {
//         status: u16,
//         message: String,
//         reason: String,
//     },
//     Other(u16),
// }
// impl failure::Fail for ApiError {}
// impl fmt::Display for ApiError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             ApiError::Unauthorized => write!(f, "Unauthorized request to API"),
//             ApiError::RateLimited(e) => {
//                 if let Some(d) = e {
//                     write!(f, "Exceeded API request limit - please wait {} seconds", d)
//                 } else {
//                     write!(f, "Exceeded API request limit")
//                 }
//             }
//             ApiError::RegularError { status, message } => {
//                 write!(f, "Spotify API error code {}: {}", status, message)
//             }
//             ApiError::PlayerError {
//                 status,
//                 message,
//                 reason,
//             } => write!(
//                 f,
//                 "Spotify API error code {} {}: {}",
//                 status, reason, message
//             ),
//             ApiError::Other(s) => write!(f, "Spotify API reported error code {}", s),
//         }
//     }
// }
// impl ApiError {
//     async fn from_response(response: reqwest::Response) -> Self {
//         match response.status() {
//             StatusCode::UNAUTHORIZED => ApiError::Unauthorized,
//             StatusCode::TOO_MANY_REQUESTS => {
//                 if let Ok(duration) = response.headers()[reqwest::header::RETRY_AFTER].to_str() {
//                     ApiError::RateLimited(duration.parse::<usize>().ok())
//                 } else {
//                     ApiError::RateLimited(None)
//                 }
//             }
//             status @ StatusCode::FORBIDDEN | status @ StatusCode::NOT_FOUND => {
//                 if let Ok(reason) = response.json::<ApiError>().await {
//                     reason
//                 } else {
//                     ApiError::Other(status.as_u16())
//                 }
//             }
//             status => ApiError::Other(status.as_u16()),
//         }
//     }
// }