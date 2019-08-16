extern crate rspotify;

use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};
use cursive::Cursive;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::model::offset::for_position;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::spotify::senum::Country;
use rspotify::spotify::util::get_token;

fn main() {
    // Set client_id and client_secret in .env file or
    // export CLIENT_ID="your client_id"
    // export CLIENT_SECRET="secret"
    // export REDIRECT_URI=your-direct-uri

    // Or set client_id, client_secret,redirect_uri explictly
    // let oauth = SpotifyOAuth::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .redirect_uri("http://localhost:8888/callback")
    //     .build();

    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state user-read-playback-state user-read-private")
        .build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            // Or set client_id and client_secret explictly
            // let client_credential = SpotifyClientCredentials::default()
            //     .client_id("this-is-my-client-id")
            //     .client_secret("this-is-my-client-secret")
            //     .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let devices = spotify.device();
            println!("{:?}", devices);

            let device_id = String::from("2577b0ea0b00e3d2c0d276d8f9629dde8645e3d8");
            let uris = vec!["spotify:track:3ZFwuJwUpIl0GeXsvF1ELf".to_owned()];

            let query = "abba";
            let tracks = match spotify.search_track(query, 10, 0, Some(Country::UnitedKingdom)) {
                Ok(tracks) => tracks,
                Err(_) => return (),
            };

            // match spotify.start_playback(Some(device_id), None, Some(uris), for_position(0)) {
            //     Ok(_) => println!("start playback successful"),
            //     Err(e) => eprintln!("start playback failed {}", e),
            // }
            let mut select = SelectView::new()
                // Center the text horizontally
                .h_align(HAlign::Center)
                // Use keyboard to jump to the pressed letters
                .autojump();

            for item in &tracks.tracks.items {
                select.add_item(&item.name, &item.uri);
            }

            // Sets the callback for when "Enter" is pressed.
            select.set_on_submit(show_next_window);

            // Let's override the `j` and `k` keys for navigation
            let select = OnEventView::new(select)
                .on_pre_event_inner('k', |s, _| {
                    s.select_up(1);
                    Some(EventResult::Consumed(None))
                })
                .on_pre_event_inner('j', |s, _| {
                    s.select_down(1);
                    Some(EventResult::Consumed(None))
                });

            let mut siv = Cursive::default();

            // Let's add a BoxView to keep the list at a reasonable size
            // (it can scroll anyway).
            siv.add_layer(
                Dialog::around(select.scrollable().fixed_size((20, 10)))
                    .title("What is the name of the track?"),
            );

            siv.run();
        }
        None => println!("auth failed"),
    };
}

// Let's put the callback in a separate function to keep it clean,
// but it's not required.
fn show_next_window(siv: &mut Cursive, song_id: &str) {
    siv.pop_layer();
    let text = format!("Playing {}!", song_id);
    siv.add_layer(Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()));
}
