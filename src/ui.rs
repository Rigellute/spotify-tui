use super::app::{ActiveBlock, App};
use rspotify::spotify::model::artist::SimplifiedArtist;
use rspotify::spotify::model::track::FullTrack;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Row, SelectableList, Table, Text, Widget};
use tui::Frame;

fn format_song(song: &Option<FullTrack>) -> [Text<'static>; 3] {
    match song {
        Some(s) => [
            Text::styled(
                s.name.to_owned(),
                Style::default().fg(Color::Magenta).modifier(Modifier::BOLD),
            ),
            Text::raw(" - "),
            Text::styled(
                create_artist_string(&s.artists),
                Style::default().fg(Color::White),
            ),
        ],
        None => [Text::raw(""), Text::raw(""), Text::raw("")],
    }
}

fn get_color(active_block: &ActiveBlock, block_to_match: ActiveBlock) -> Style {
    if *active_block == block_to_match {
        Style::default().fg(Color::LightCyan)
    } else {
        Style::default().fg(Color::Gray)
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

    // Would be nice to share the same source of truth as the event match below
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

    Paragraph::new([Text::raw(&app.input)].iter())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Input")
                .title_style(get_color(&app.active_block, ActiveBlock::Input))
                .border_style(get_color(&app.active_block, ActiveBlock::Input)),
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

    // This should be a home page?
    draw_search_results(f, app, chunks[1]);
}

pub fn draw_playlist_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let playlist_items = match &app.playlists {
        Some(p) => p.items.iter().map(|item| item.name.to_owned()).collect(),
        None => vec![],
    };

    draw_selectable_list(
        f,
        app,
        layout_chunk,
        "Playlists",
        &playlist_items,
        ActiveBlock::MyPlaylists,
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

        let songs = match &app.searched_tracks {
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
            app,
            song_artist_block[0],
            "Songs",
            &songs,
            ActiveBlock::SongSearch,
        );

        let artists = match &app.searched_artists {
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
            app,
            song_artist_block[1],
            "Artists",
            &artists,
            ActiveBlock::ArtistSearch,
        );
    }

    {
        let albums_playlist_block = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[1]);

        let albums = match &app.searched_albums {
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
            app,
            albums_playlist_block[0],
            "Albums",
            &albums,
            ActiveBlock::AlbumSearch,
        );

        let playlists = match &app.searched_albums {
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
            app,
            albums_playlist_block[1],
            "Playlists",
            &playlists,
            ActiveBlock::PlaylistSearch,
        );
    }
}

pub fn draw_song_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let normal_style = Style::default().fg(Color::White);
    let header = ["Title", "Artist", "Album"];

    let formatted_songs = display_songs(&app.songs_for_table);

    let selected_style =
        get_color(&app.active_block, ActiveBlock::SongTable).modifier(Modifier::BOLD);

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
                .title_style(get_color(&app.active_block, ActiveBlock::SongTable))
                .border_style(get_color(&app.active_block, ActiveBlock::SongTable)),
        )
        .style(Style::default().fg(Color::White))
        .widths(&[40, 40, 40])
        .render(f, layout_chunk);
}

pub fn draw_playing_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(layout_chunk);

    let playing_text = format_song(&app.current_playing_song);

    Paragraph::new(playing_text.iter())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Playing")
                .title_style(Style::default().fg(Color::Gray))
                .border_style(Style::default().fg(Color::Gray)),
        )
        .render(f, chunks[0]);
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

    let playing_text = [
        Text::raw("Api response: "),
        Text::styled(&app.api_error, Style::default().fg(Color::LightRed)),
        Text::styled("\nPress <Esc> to return", Style::default().fg(Color::White)),
    ];

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

pub fn draw_device_list<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(10)
        .split(f.size());

    let no_device_message = vec![
        "No devices found: Make sure a device has is active".to_string(),
        "\nHint: Press `d` to go to device selection menu".to_string(),
    ];

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
    app: &App,
    layout_chunk: Rect,
    title: &str,
    items: &[String],
    active_block: ActiveBlock,
) where
    B: Backend,
{
    SelectableList::default()
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .title_style(get_color(&app.active_block, active_block))
                .border_style(get_color(&app.active_block, active_block)),
        )
        .items(items)
        .style(Style::default().fg(Color::White))
        .select(app.selected_playlist_index)
        .highlight_style(get_color(&app.active_block, active_block).modifier(Modifier::BOLD))
        .render(f, layout_chunk);
}
