use super::app::{ActiveBlock, App, Routes, SearchResultBlock};
use rspotify::spotify::model::artist::SimplifiedArtist;
use rspotify::spotify::model::track::FullTrack;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Gauge, Paragraph, Row, SelectableList, Table, Text, Widget};
use tui::Frame;

fn format_song(song: &FullTrack) -> [Text<'static>; 3] {
    [
        Text::styled(
            song.name.to_owned(),
            Style::default().fg(Color::Magenta).modifier(Modifier::BOLD),
        ),
        Text::raw(" - "),
        Text::styled(
            create_artist_string(&song.artists),
            Style::default().fg(Color::White),
        ),
    ]
}

fn get_color((is_active, is_hovered): (bool, bool)) -> Style {
    match (is_active, is_hovered) {
        (true, _) => Style::default().fg(Color::LightCyan),
        (false, true) => Style::default().fg(Color::Magenta),
        _ => Style::default().fg(Color::Gray),
    }
}

fn create_artist_string(artists: &[SimplifiedArtist]) -> String {
    artists
        .iter()
        .map(|artist| artist.name.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

fn display_songs(track_search_results: &[FullTrack]) -> Vec<Vec<String>> {
    track_search_results
        .iter()
        .map(|item| {
            vec![
                item.name.to_owned(),
                create_artist_string(&item.artists),
                item.album.name.to_owned(),
                millis_to_minutes(item.duration_ms as u128),
            ]
        })
        .collect()
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
    let header = ["Active block", "Event", "Description"];

    // Would be nice to share the same source of truth as the event matches in `src/handlers`
    let help_rows = vec![
        vec!["Playlist/Song block", "j", "Move selection down"],
        vec!["Playlist/Song blocks", "k", "Move selection up"],
        vec!["General", "/", "Enter input for search"],
        vec!["General", "q", "Quit"],
        vec!["General", "<Ctrl+c>", "Quit"],
        vec!["General", "<Esc>", "Go back"],
        vec!["General", "d", "Select device to play music on"],
        vec!["Input", "<Ctrl+u>", "Delete input"],
        vec!["Input", "<Enter>", "Search with input text"],
        vec![
            "Input",
            "<Esc>",
            "Escape from the input back to playlist view",
        ],
    ];

    let rows = help_rows
        .into_iter()
        .map(|item| Row::StyledData(item.into_iter(), gray));

    Table::new(header.iter(), rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(white)
                .title("Help")
                .title_style(gray)
                .border_style(gray),
        )
        .style(Style::default().fg(Color::White))
        .widths(&[30, 20, 30])
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

    let highlight_state = (app.active_block == ActiveBlock::Input, false);

    Paragraph::new([Text::raw(&app.input)].iter())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Input")
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

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    draw_playlist_block(f, app, chunks[0]);

    let active_route = app.get_current_route();

    match active_route {
        Some(route) => {
            match route {
                Routes::Search => {
                    draw_search_results(f, app, chunks[1]);
                }
                Routes::SongTable => {
                    draw_song_table(f, app, chunks[1]);
                }
                Routes::Album(_album_id) => {}
                Routes::Artist(_artist_id) => {}
                Routes::TrackInfo(_track_id) => {}
            };
        }
        None => {
            draw_home(f, app, chunks[1]);
        }
    }
}

pub fn draw_playlist_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let playlist_items = match &app.playlists {
        Some(p) => p.items.iter().map(|item| item.name.to_owned()).collect(),
        None => vec![],
    };

    let highlight_state = (app.active_block == ActiveBlock::MyPlaylists, false);

    draw_selectable_list(
        f,
        layout_chunk,
        "Playlists",
        &playlist_items,
        highlight_state,
        app.selected_playlist_index,
    );
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
            (
                app.search_results.selected_block == SearchResultBlock::SongSearch,
                app.search_results.hovered_block == SearchResultBlock::SongSearch,
            ),
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
            (
                app.search_results.selected_block == SearchResultBlock::ArtistSearch,
                app.search_results.hovered_block == SearchResultBlock::ArtistSearch,
            ),
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
                .map(|item| item.name.to_owned())
                .collect(),
            None => vec![],
        };

        draw_selectable_list(
            f,
            albums_playlist_block[0],
            "Albums",
            &albums,
            (
                app.search_results.selected_block == SearchResultBlock::AlbumSearch,
                app.search_results.hovered_block == SearchResultBlock::AlbumSearch,
            ),
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
            (
                app.search_results.selected_block == SearchResultBlock::PlaylistSearch,
                app.search_results.hovered_block == SearchResultBlock::PlaylistSearch,
            ),
            app.search_results.selected_playlists_index,
        );
    }
}

pub fn draw_song_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let normal_style = Style::default().fg(Color::White);
    let header = ["Title", "Artist", "Album", "Length"];

    let formatted_songs = display_songs(&app.songs_for_table);

    let highlight_state = (app.active_block == ActiveBlock::SongTable, false);

    let selected_style = get_color(highlight_state).modifier(Modifier::BOLD);

    let selected_song_index = app.select_song_index;
    let rows = formatted_songs.into_iter().enumerate().map(|(i, item)| {
        if i == selected_song_index {
            Row::StyledData(item.into_iter(), selected_style)
        } else {
            Row::StyledData(item.into_iter(), normal_style)
        }
    });

    Table::new(header.iter(), rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Songs")
                .title_style(get_color(highlight_state))
                .border_style(get_color(highlight_state)),
        )
        .style(Style::default().fg(Color::White))
        .widths(&[35, 35, 35, 10])
        .render(f, layout_chunk);
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
    if let Some(current_playing_context) = &app.current_playing_context {
        if let Some(track_item) = &current_playing_context.item {
            let playing_text = format_song(track_item);
            Block::default()
                .borders(Borders::ALL)
                .title("Playing")
                .title_style(Style::default().fg(Color::Gray))
                .border_style(Style::default().fg(Color::Gray))
                .render(f, layout_chunk);

            Paragraph::new(playing_text.iter())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().title("Track"))
                .render(f, chunks[0]);

            let perc = (app.song_progress_ms as f64 / track_item.duration_ms as f64) * 100 as f64;
            Gauge::default()
                .block(Block::default().title(""))
                .style(
                    Style::default()
                        .fg(Color::Magenta)
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

pub fn draw_api_error<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(10)
        .split(f.size());

    let mut playing_text = vec![
        Text::raw("Api response: "),
        Text::styled(&app.api_error, Style::default().fg(Color::LightRed)),
        Text::styled("\nPress <Esc> to return", Style::default().fg(Color::White)),
    ];

    if app.device_id.is_none() {
        playing_text.push(Text::styled(
            "\nHint: Press `d` to go to device selection menu",
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

// TODO: fill out home page
fn draw_home<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let highlight_state = (app.active_block == ActiveBlock::Home, false);
    Block::default()
        .title("Home")
        .borders(Borders::ALL)
        .title_style(get_color(highlight_state))
        .border_style(get_color(highlight_state))
        .render(f, layout_chunk);
}

pub fn draw_device_list<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(10)
        .split(f.size());

    let no_device_message = vec!["No devices found: Make sure a device has is active".to_string()];

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
        .render(f, chunks[0]);
}

pub fn draw_selectable_list<B>(
    f: &mut Frame<B>,
    layout_chunk: Rect,
    title: &str,
    items: &[String],
    highlight_state: (bool, bool),
    selected_index: Option<usize>,
) where
    B: Backend,
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

fn millis_to_minutes(millis: u128) -> String {
    let minutes = millis / 60000;
    let seconds = (millis % 60000) / 1000;
    let seconds_display = if seconds < 10 {
        format!("0{}", seconds)
    } else {
        format!("{}", seconds)
    };

    if seconds == 60 {
        format!("{}:00", minutes + 1)
    } else {
        format!("{}:{}", minutes, seconds_display)
    }
}

fn display_track_progress(progress: u128, track_duration: u32) -> String {
    let duration = millis_to_minutes(track_duration as u128);
    let progress_display = millis_to_minutes(progress);
    let remaining = millis_to_minutes(track_duration as u128 - progress);

    format!("{}/{} (-{})", progress_display, duration, remaining,)
}
