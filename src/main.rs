use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::event::{Event, Key};
use cursive::theme::{Color, PaletteColor, Theme};
use cursive::traits::*;
use cursive::views::{CircularFocus, Dialog, EditView, OnEventView, Panel, SelectView, TextView};
use cursive::Cursive;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::model::artist::SimplifiedArtist;
use rspotify::spotify::model::offset::for_position;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::spotify::senum::Country;
use rspotify::spotify::util::get_token;
mod table;

struct Data {
    spotify: Spotify,
    device_id: String,
}

fn main() {
    // Initialize the cursive logger.
    cursive::logger::init();
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state user-read-playback-state user-read-private user-read-currently-playing")
        .build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();

            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let devices = spotify.device();
            println!("{:?}", devices);

            let context = spotify.current_playing(None);

            // TODO: Create a step for selecting which device to play
            let device_id = String::from("2577b0ea0b00e3d2c0d276d8f9629dde8645e3d8");

            let mut siv = Cursive::default();

            let theme = custom_theme_from_cursive(&siv);
            siv.set_theme(theme);

            // We can quit by pressing `q`
            siv.add_global_callback('q', Cursive::quit);

            if let Ok(ctx) = context {
                if let Some(c) = ctx {
                    if let Some(item) = c.item {
                        if c.is_playing {
                            siv.add_layer(
                                Dialog::around(Panel::new(
                                    TextView::new(format!(
                                        "Playing {} - {}",
                                        item.name,
                                        create_arist_string(&item.artists)
                                    ))
                                    .scrollable(),
                                ))
                                .title("Unicode and wide-character support")
                                // This is the alignment for the button
                                .h_align(HAlign::Center),
                            )
                        }
                    }
                }
            };

            siv.set_user_data(Data { spotify, device_id });
            siv.add_layer(
                // Most views can be configured in a chainable way
                CircularFocus::wrap_tab(
                    Dialog::around(TextView::new("Use ctrl-f to search"))
                        .title("Spotify TUI")
                        .button("Ok", |s| {
                            s.pop_layer();
                        }),
                ),
            );

            siv.add_global_callback('q', cursive::Cursive::quit);

            siv.add_global_callback(Event::CtrlChar('f'), |s| {
                // When Ctrl-F is pressed, show the Find popup.
                // Pressing the Escape key will discard it.
                s.add_layer(
                    OnEventView::new(
                        Dialog::new()
                            .title("Find")
                            .content(
                                EditView::new()
                                    .on_submit(move |s, text| {
                                        search_for_track(s, &text);
                                    })
                                    .with_id("edit")
                                    .min_width(50),
                            )
                            .button("Ok", |s| {
                                let text = s
                                    .call_on_id("edit", |view: &mut EditView| view.get_content())
                                    .unwrap();
                                search_for_track(s, &text);
                            })
                            .dismiss_button("Cancel"),
                    )
                    .on_event(Event::Key(Key::Esc), |s| {
                        s.pop_layer();
                    }),
                )
            });

            siv.run();
        }
        None => println!("auth failed"),
    };
}

fn search_for_track(siv: &mut Cursive, query: &str) {
    // First, remove the find popup
    siv.pop_layer();

    let data = siv.user_data::<Data>().unwrap();
    let tracks = match data
        .spotify
        .search_track(query, 10, 0, Some(Country::UnitedKingdom))
    {
        Ok(result) => result.tracks,
        Err(_) => return,
    };

    if tracks.items.is_empty() {
        siv.add_layer(
            // Most views can be configured in a chainable way
            CircularFocus::wrap_tab(
                Dialog::around(TextView::new("No tracks found"))
                    .title("Search")
                    .button("Ok", |s| {
                        s.pop_layer();
                    }),
            ),
        );

        return;
    }

    let items: Vec<table::Track> = tracks
        .items
        .iter()
        .map(|item| table::Track {
            artist: create_arist_string(&item.artists),
            name: item.name.clone(),
            album: item.album.name.clone(),
            uri: item.uri.clone(),
        })
        .collect();

    let mut table_view = table::build_tracks_table(siv);
    table_view.set_items(items.clone());
    table_view.set_on_submit(move |siv: &mut Cursive, row: usize, index: usize| {
        play_track(siv, &items[index].uri);
    });
    siv.add_layer(
        Dialog::around(table_view.with_id("table").min_size((100, 40))).title("Table View"),
    );
}

fn play_track(siv: &mut Cursive, song_id: &String) {
    siv.pop_layer();
    let Data { spotify, device_id } = siv.user_data::<Data>().unwrap();

    match spotify.start_playback(
        Some(device_id.to_owned()),
        None,
        Some(vec![song_id.to_owned()]),
        for_position(0),
    ) {
        Ok(_) => println!("start playback successful"),
        Err(e) => eprintln!("start playback failed {}", e),
    }
}

fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    // We'll return the current theme with a small modification.
    let mut theme = siv.current_theme().clone();

    theme.palette[PaletteColor::Background] = Color::TerminalDefault;

    theme
}

fn create_arist_string(artists: &Vec<SimplifiedArtist>) -> String {
    artists
        .iter()
        .fold("".to_string(), |artist_string, artist| {
            artist_string + &artist.name
        })
}
