mod help;
mod util;
use super::app::{
    ActiveBlock, AlbumTableContext, App, RouteId, SearchResultBlock, LIBRARY_OPTIONS,
};
use super::banner::BANNER;
use help::get_help_docs;
use rspotify::spotify::senum::RepeatState;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Gauge, Paragraph, Row, SelectableList, Table, Text, Widget};
use tui::Frame;
use util::{
    create_artist_string, display_track_progress, get_color, get_percentage_width,
    get_search_results_highlight_state, millis_to_minutes,
};

pub struct TableItem {
    id: String,
    format: Vec<String>,
}

pub struct TableHeader<'a> {
    text: &'a str,
    width: u16,
}

pub fn draw_help_menu<B>(f: &mut Frame<B>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(2)
        .split(f.size());

    let white = Style::default().fg(Color::White);
    let gray = Style::default().fg(Color::White);
    let header = ["Context", "Event", "Description"];

    let help_docs = get_help_docs();

    let rows = help_docs
        .iter()
        .map(|item| Row::StyledData(item.iter(), gray));

    Table::new(header.iter(), rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(white)
                .title("Help (press <Esc> to go back)")
                .title_style(gray)
                .border_style(gray),
        )
        .style(Style::default().fg(Color::White))
        .widths(&[20, 40, 50])
        .render(f, chunks[0]);
}

pub fn draw_input_and_help_box<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(layout_chunk);

    let current_route = app.get_current_route();

    let highlight_state = (
        current_route.active_block == ActiveBlock::Input,
        current_route.hovered_block == ActiveBlock::Input,
    );

    Paragraph::new([Text::raw(&app.input)].iter())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Search")
                .title_style(get_color(highlight_state))
                .border_style(get_color(highlight_state)),
        )
        .render(f, chunks[0]);

    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray))
        .title_style(Style::default().fg(Color::Gray));

    Paragraph::new([Text::raw("Type ?")].iter())
        .block(block)
        .style(Style::default().fg(Color::Gray))
        .render(f, chunks[1]);
}

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(6),
            ]
            .as_ref(),
        )
        .margin(2)
        .split(f.size());

    // Search input and help
    draw_input_and_help_box(f, app, parent_layout[0]);

    // Nested main block with potential routes
    draw_routes(f, app, parent_layout[1]);

    // Currently playing
    draw_playing_block(f, app, parent_layout[2]);
}

pub fn draw_routes<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    draw_user_block(f, app, chunks[0]);

    let current_route = app.get_current_route();

    match current_route.id {
        RouteId::Search => {
            draw_search_results(f, app, chunks[1]);
        }
        RouteId::TrackTable => {
            draw_song_table(f, app, chunks[1]);
        }
        RouteId::AlbumTracks => {
            draw_album_table(f, app, chunks[1]);
        }
        RouteId::RecentlyPlayed => {
            draw_recently_played_table(f, app, chunks[1]);
        }
        RouteId::Artist => {
            draw_artist_albums(f, app, chunks[1]);
        }
        RouteId::AlbumList => {
            draw_album_list(f, app, chunks[1]);
        }
        RouteId::Home => {
            draw_home(f, app, chunks[1]);
        }
        RouteId::MadeForYou => {
            draw_not_implemented_yet(f, app, chunks[1], ActiveBlock::MadeForYou, "Made For You");
        }
        RouteId::Artists => {
            draw_not_implemented_yet(f, app, chunks[1], ActiveBlock::Artists, "Artists");
        }
        RouteId::Podcasts => {
            draw_not_implemented_yet(f, app, chunks[1], ActiveBlock::Podcasts, "Podcasts");
        }
        RouteId::Error => {} // This is handled as a "full screen" route in main.rs
        RouteId::SelectedDevice => {} // This is handled as a "full screen" route in main.rs
    };
}

pub fn draw_library_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Library,
        current_route.hovered_block == ActiveBlock::Library,
    );
    draw_selectable_list(
        f,
        layout_chunk,
        "Library",
        &LIBRARY_OPTIONS,
        highlight_state,
        Some(app.library.selected_index),
    );
}

pub fn draw_playlist_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let playlist_items = match &app.playlists {
        Some(p) => p.items.iter().map(|item| item.name.to_owned()).collect(),
        None => vec![],
    };

    let current_route = app.get_current_route();

    let highlight_state = (
        current_route.active_block == ActiveBlock::MyPlaylists,
        current_route.hovered_block == ActiveBlock::MyPlaylists,
    );

    draw_selectable_list(
        f,
        layout_chunk,
        "Playlists",
        &playlist_items,
        highlight_state,
        app.selected_playlist_index,
    );
}

pub fn draw_user_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(layout_chunk);

    draw_library_block(f, app, chunks[0]);
    draw_playlist_block(f, app, chunks[1]);
}

pub fn draw_search_results<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(layout_chunk);

    {
        let song_artist_block = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);

        let songs = match &app.search_results.tracks {
            Some(r) => r
                .tracks
                .items
                .iter()
                // TODO: reuse the function formatting this text for `playing` block
                .map(|item| item.name.to_owned() + " - " + &create_artist_string(&item.artists))
                .collect(),
            None => vec![],
        };

        draw_selectable_list(
            f,
            song_artist_block[0],
            "Songs",
            &songs,
            get_search_results_highlight_state(app, SearchResultBlock::SongSearch),
            app.search_results.selected_tracks_index,
        );

        let artists = match &app.search_results.artists {
            Some(r) => r
                .artists
                .items
                .iter()
                .map(|item| item.name.to_owned())
                .collect(),
            None => vec![],
        };

        draw_selectable_list(
            f,
            song_artist_block[1],
            "Artists",
            &artists,
            get_search_results_highlight_state(app, SearchResultBlock::ArtistSearch),
            app.search_results.selected_artists_index,
        );
    }

    {
        let albums_playlist_block = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[1]);

        let albums = match &app.search_results.albums {
            Some(r) => r
                .albums
                .items
                .iter()
                .map(|item| {
                    format!(
                        "{} - {}",
                        item.name.to_owned(),
                        create_artist_string(&item.artists)
                    )
                })
                .collect(),
            None => vec![],
        };

        draw_selectable_list(
            f,
            albums_playlist_block[0],
            "Albums",
            &albums,
            get_search_results_highlight_state(app, SearchResultBlock::AlbumSearch),
            app.search_results.selected_album_index,
        );

        let playlists = match &app.search_results.playlists {
            Some(r) => r
                .playlists
                .items
                .iter()
                .map(|item| item.name.to_owned())
                .collect(),
            None => vec![],
        };
        draw_selectable_list(
            f,
            albums_playlist_block[1],
            "Playlists",
            &playlists,
            get_search_results_highlight_state(app, SearchResultBlock::PlaylistSearch),
            app.search_results.selected_playlists_index,
        );
    }
}

struct AlbumUI {
    selected_index: usize,
    items: Vec<TableItem>,
    title: String,
}

pub fn draw_album_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let header = [
        TableHeader {
            text: "#",
            width: 3,
        },
        TableHeader {
            text: "Title",
            width: get_percentage_width(layout_chunk.width, 0.80),
        },
        TableHeader {
            text: "Length",
            width: get_percentage_width(layout_chunk.width, 0.15),
        },
    ];

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::AlbumTracks,
        current_route.hovered_block == ActiveBlock::AlbumTracks,
    );

    let album_ui = match app.album_table_context.clone() {
        AlbumTableContext::Simplified => match &app.selected_album {
            Some(selected_album) => Some(AlbumUI {
                items: selected_album
                    .tracks
                    .items
                    .iter()
                    .map(|item| TableItem {
                        id: item.id.clone().unwrap_or_else(|| "".to_string()),
                        format: vec![
                            item.track_number.to_string(),
                            item.name.to_owned(),
                            millis_to_minutes(u128::from(item.duration_ms)),
                        ],
                    })
                    .collect::<Vec<TableItem>>(),
                title: format!(
                    "{} by {}",
                    selected_album.album.name,
                    create_artist_string(&selected_album.album.artists)
                ),
                selected_index: selected_album.selected_index.unwrap_or(0),
            }),
            None => None,
        },
        AlbumTableContext::Full => match &app.library.saved_albums.get_results(None) {
            Some(albums) => match albums.items.get(app.album_list_index) {
                Some(selected_album) => Some(AlbumUI {
                    items: selected_album
                        .album
                        .tracks
                        .items
                        .iter()
                        .map(|item| TableItem {
                            id: item.id.clone().unwrap_or_else(|| "".to_string()),
                            format: vec![
                                item.track_number.to_string(),
                                item.name.to_owned(),
                                millis_to_minutes(u128::from(item.duration_ms)),
                            ],
                        })
                        .collect::<Vec<TableItem>>(),
                    title: format!(
                        "{} by {}",
                        selected_album.album.name,
                        create_artist_string(&selected_album.album.artists)
                    ),
                    selected_index: app.saved_album_tracks_index,
                }),
                None => None,
            },
            None => None,
        },
    };

    if let Some(album_ui) = album_ui {
        draw_table(
            f,
            app,
            layout_chunk,
            (&album_ui.title, &header),
            &album_ui.items,
            album_ui.selected_index,
            highlight_state,
        );
    };
}

pub fn draw_song_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let header = [
        TableHeader {
            text: "Title",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
        TableHeader {
            text: "Artist",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
        TableHeader {
            text: "AlbumTracks",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
        TableHeader {
            text: "Length",
            width: get_percentage_width(layout_chunk.width, 0.1),
        },
    ];

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::TrackTable,
        current_route.hovered_block == ActiveBlock::TrackTable,
    );

    let items = app
        .track_table
        .tracks
        .iter()
        .map(|item| TableItem {
            id: item.id.clone().unwrap_or_else(|| "".to_string()),
            format: vec![
                item.name.to_owned(),
                create_artist_string(&item.artists),
                item.album.name.to_owned(),
                millis_to_minutes(u128::from(item.duration_ms)),
            ],
        })
        .collect::<Vec<TableItem>>();

    draw_table(
        f,
        app,
        layout_chunk,
        ("Songs", &header),
        &items,
        app.track_table.selected_index,
        highlight_state,
    )
}

pub fn draw_playing_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .margin(1)
        .split(layout_chunk);

    // If no track is playing, render paragraph showing which device is selected, if no selected
    // give hint to choose a device
    if let Some(current_playback_context) = &app.current_playback_context {
        if let Some(track_item) = &current_playback_context.item {
            let play_title = if current_playback_context.is_playing {
                "Playing"
            } else {
                "Paused"
            };

            let shuffle_text = if current_playback_context.shuffle_state {
                "On"
            } else {
                "Off"
            };

            let repeat_text = match current_playback_context.repeat_state {
                RepeatState::Off => "Off",
                RepeatState::Track => "Track",
                RepeatState::Context => "All",
            };

            let title = format!(
                "{} ({} | Shuffle: {} | Repeat: {})",
                play_title, current_playback_context.device.name, shuffle_text, repeat_text
            );

            Block::default()
                .borders(Borders::ALL)
                .title(&title)
                .title_style(Style::default().fg(Color::Gray))
                .border_style(Style::default().fg(Color::Gray))
                .render(f, layout_chunk);

            Paragraph::new(
                [Text::styled(
                    create_artist_string(&track_item.artists),
                    Style::default().fg(Color::White),
                )]
                .iter(),
            )
            .style(Style::default().fg(Color::White))
            .block(
                Block::default().title(&track_item.name).title_style(
                    Style::default()
                        .fg(Color::LightCyan)
                        .modifier(Modifier::BOLD),
                ),
            )
            .render(f, chunks[0]);

            let perc = (app.song_progress_ms as f64 / f64::from(track_item.duration_ms)) * 100_f64;

            Gauge::default()
                .block(Block::default().title(""))
                .style(
                    Style::default()
                        .fg(Color::LightCyan)
                        .bg(Color::Black)
                        .modifier(Modifier::ITALIC | Modifier::BOLD),
                )
                .percent(perc as u16)
                .label(&display_track_progress(
                    app.song_progress_ms,
                    track_item.duration_ms,
                ))
                .render(f, chunks[1]);
        }
    }
}

pub fn draw_error_screen<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(f.size());

    let mut playing_text = vec![
        Text::raw("Api response: "),
        Text::styled(&app.api_error, Style::default().fg(Color::LightRed)),
        Text::styled(
            "

If you are trying to play a track, please check that
    1. You have a Spotify Premium Account
    2. Your playback device is active and selected - press `d` to go to device selection menu
    3. If you're using spotifyd as a playback device, your device name must not contain spaces
            ",
            Style::default().fg(Color::White),
        ),
        Text::styled("
Hint: a playback device must be either an official spotify client or a light weight alternative such as spotifyd
        ", 
        Style::default().fg(Color::Yellow)),
        Text::styled(
            "\nPress <Esc> to return",
            Style::default().fg(Color::Gray),
        ),
    ];

    if app.client_config.device_id.is_none() {
        playing_text.push(Text::styled(
            "\nNo playback device is selected - follow point 2 above",
            Style::default().fg(Color::LightMagenta),
        ))
    }

    Paragraph::new(playing_text.iter())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Error")
                .title_style(Style::default().fg(Color::Red))
                .border_style(Style::default().fg(Color::Red)),
        )
        .render(f, chunks[0]);
}

fn draw_home<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Home,
        current_route.hovered_block == ActiveBlock::Home,
    );
    let display_block = Block::default()
        .title("Welcome!")
        .borders(Borders::ALL)
        .title_style(get_color(highlight_state))
        .border_style(get_color(highlight_state));

    let text = vec![
        Text::styled(BANNER, Style::default().fg(Color::LightCyan)),
        Text::raw("\nPlease report any bugs or missing features to https://github.com/Rigellute/spotify-tui"),
    ];

    Paragraph::new(text.iter())
        .style(Style::default().fg(Color::White))
        .block(display_block)
        .render(f, layout_chunk);
}

fn draw_not_implemented_yet<B>(
    f: &mut Frame<B>,
    app: &App,
    layout_chunk: Rect,
    block: ActiveBlock,
    title: &str,
) where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == block,
        current_route.hovered_block == block,
    );
    let display_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .title_style(get_color(highlight_state))
        .border_style(get_color(highlight_state));

    let text = vec![Text::raw("Not implemented yet!")];

    Paragraph::new(text.iter())
        .style(Style::default().fg(Color::White))
        .block(display_block)
        .render(f, layout_chunk);
}

fn draw_artist_albums<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Artist,
        current_route.hovered_block == ActiveBlock::Artist,
    );

    if let Some(artist_albums) = &app.artist_albums {
        let items = &artist_albums
            .albums
            .items
            .iter()
            .map(|item| item.name.to_owned())
            .collect::<Vec<String>>();

        draw_selectable_list(
            f,
            layout_chunk,
            &artist_albums.artist_name,
            &items,
            highlight_state,
            Some(artist_albums.selected_index),
        );
    };
}

pub fn draw_device_list<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .margin(10)
        .split(f.size());

    let device_instructions = vec![
        "To play tracks, please select a device.",
        "Use `j/k` or up/down arrow keys to move up and down and <Enter> to select",
        "The official spotify app needs to remain open on your selected device in order for playback to work.",
        "Your choice here will be cached so you can jump straight back in when you next open `spotify-tui`.",
        "You can change the playback device at any time by pressing `d`.",
    ];

    Paragraph::new([Text::raw(device_instructions.join("\n"))].iter())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::NONE)
                .title("Welcome to spotify-tui!")
                .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD)),
        )
        .render(f, chunks[0]);

    let no_device_message = vec!["No devices found: Make sure a device is active".to_string()];

    let items = match &app.devices {
        Some(items) => {
            if items.devices.is_empty() {
                no_device_message
            } else {
                items
                    .devices
                    .iter()
                    .map(|device| device.name.to_owned())
                    .collect()
            }
        }
        None => no_device_message,
    };

    SelectableList::default()
        .block(
            Block::default()
                .title("Devices")
                .borders(Borders::ALL)
                .title_style(Style::default().fg(Color::LightCyan))
                .border_style(Style::default().fg(Color::Gray)),
        )
        .items(&items)
        .style(Style::default().fg(Color::White))
        .select(app.selected_device_index)
        .highlight_style(
            Style::default()
                .fg(Color::LightCyan)
                .modifier(Modifier::BOLD),
        )
        .render(f, chunks[1]);
}

pub fn draw_album_list<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let header = [
        TableHeader {
            text: "Name",
            width: get_percentage_width(layout_chunk.width, 2.0 / 5.0),
        },
        TableHeader {
            text: "Artists",
            width: get_percentage_width(layout_chunk.width, 2.0 / 5.0),
        },
        TableHeader {
            text: "Release Date",
            width: get_percentage_width(layout_chunk.width, 1.0 / 3.0),
        },
    ];

    let current_route = app.get_current_route();

    let highlight_state = (
        current_route.active_block == ActiveBlock::AlbumList,
        current_route.hovered_block == ActiveBlock::AlbumList,
    );

    let selected_song_index = app.album_list_index;

    if let Some(saved_albums) = app.library.saved_albums.get_results(None) {
        let items = saved_albums
            .items
            .iter()
            .map(|album_page| TableItem {
                id: album_page.album.id.to_owned(),
                format: vec![
                    album_page.album.name.to_owned(),
                    create_artist_string(&album_page.album.artists),
                    album_page.album.release_date.to_owned(),
                ],
            })
            .collect::<Vec<TableItem>>();

        draw_table(
            f,
            app,
            layout_chunk,
            ("Saved Albums", &header),
            &items,
            selected_song_index,
            highlight_state,
        )
    };
}

pub fn draw_recently_played_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let header = [
        TableHeader {
            text: "Title",
            width: get_percentage_width(layout_chunk.width, 2.0 / 5.0),
        },
        TableHeader {
            text: "Artist",
            width: get_percentage_width(layout_chunk.width, 2.0 / 5.0),
        },
        TableHeader {
            text: "Length",
            width: get_percentage_width(layout_chunk.width, 1.0 / 5.0),
        },
    ];

    if let Some(recently_played) = &app.recently_played.result {
        let current_route = app.get_current_route();

        let highlight_state = (
            current_route.active_block == ActiveBlock::RecentlyPlayed,
            current_route.hovered_block == ActiveBlock::RecentlyPlayed,
        );

        let selected_song_index = app.recently_played.index;

        let items = recently_played
            .items
            .iter()
            .map(|item| TableItem {
                id: item.track.id.clone().unwrap_or_else(|| "".to_string()),
                format: vec![
                    item.track.name.to_owned(),
                    create_artist_string(&item.track.artists),
                    millis_to_minutes(u128::from(item.track.duration_ms)),
                ],
            })
            .collect::<Vec<TableItem>>();

        draw_table(
            f,
            app,
            layout_chunk,
            ("Recently Played Tracks", &header),
            &items,
            selected_song_index,
            highlight_state,
        )
    };
}

fn draw_selectable_list<B, S>(
    f: &mut Frame<B>,
    layout_chunk: Rect,
    title: &str,
    items: &[S],
    highlight_state: (bool, bool),
    selected_index: Option<usize>,
) where
    B: Backend,
    S: std::convert::AsRef<str>,
{
    SelectableList::default()
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .title_style(get_color(highlight_state))
                .border_style(get_color(highlight_state)),
        )
        .items(items)
        .style(Style::default().fg(Color::White))
        .select(selected_index)
        .highlight_style(get_color(highlight_state).modifier(Modifier::BOLD))
        .render(f, layout_chunk);
}

fn draw_table<B>(
    f: &mut Frame<B>,
    app: &App,
    layout_chunk: Rect,
    table_layout: (&str, &[TableHeader]), // (title, header colums)
    items: &[TableItem], // The nested vector must have the same length as the `header_columns`
    selected_index: usize,
    highlight_state: (bool, bool),
) where
    B: Backend,
{
    let selected_style = get_color(highlight_state).modifier(Modifier::BOLD);

    let track_playing_index = match &app.current_playback_context {
        Some(ctx) => items.iter().position(|t| match &ctx.item {
            Some(item) => Some(t.id.to_owned()) == item.id,
            None => false,
        }),
        None => None,
    };

    let rows = items.iter().enumerate().map(|(i, item)| {
        // Show this ♥ if the song is liked
        let mut formatted_row = item.format.clone();
        // First check if the item is under selection
        if i == selected_index {
            return Row::StyledData(formatted_row.into_iter(), selected_style);
        }

        // Next check if the song should be highlighted because it is currently playing
        if let Some(_track_playing_index) = track_playing_index {
            if i == _track_playing_index {
                formatted_row[0] = format!("|> {}", &formatted_row[0]);
                return Row::StyledData(
                    formatted_row.into_iter(),
                    Style::default().fg(Color::Cyan).modifier(Modifier::BOLD),
                );
            }
        }

        // Otherwise return default styling
        Row::StyledData(formatted_row.into_iter(), Style::default().fg(Color::White))
    });

    let (title, header_columns) = table_layout;

    let widths = header_columns.iter().map(|h| h.width).collect::<Vec<u16>>();

    Table::new(header_columns.iter().map(|h| h.text), rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title(title)
                .title_style(get_color(highlight_state))
                .border_style(get_color(highlight_state)),
        )
        .style(Style::default().fg(Color::White))
        .widths(&widths)
        .render(f, layout_chunk);
}
