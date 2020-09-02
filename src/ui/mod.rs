pub mod audio_analysis;
pub mod help;
pub mod util;
use super::{
  app::{
    ActiveBlock, AlbumTableContext, App, ArtistBlock, RecommendationsContext, RouteId,
    SearchResultBlock, LIBRARY_OPTIONS,
  },
  banner::BANNER,
};
use help::get_help_docs;
use rspotify::model::PlayingItem;
use rspotify::senum::RepeatState;
use tui::{
  backend::Backend,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Modifier, Style},
  widgets::{Block, Borders, Clear, Gauge, List, ListState, Paragraph, Row, Table, Text},
  Frame,
};
use util::{
  create_artist_string, display_track_progress, get_artist_highlight_state, get_color,
  get_percentage_width, get_search_results_highlight_state, get_track_progress_percentage,
  millis_to_minutes,
};

pub enum TableId {
  Album,
  AlbumList,
  Artist,
  Song,
  RecentlyPlayed,
  MadeForYou,
}

#[derive(PartialEq)]
pub enum ColumnId {
  None,
  SongTitle,
  Liked,
}

impl Default for ColumnId {
  fn default() -> Self {
    ColumnId::None
  }
}

pub struct TableHeader<'a> {
  id: TableId,
  items: Vec<TableHeaderItem<'a>>,
}

impl TableHeader<'_> {
  pub fn get_index(&self, id: ColumnId) -> Option<usize> {
    self.items.iter().position(|item| item.id == id)
  }
}

#[derive(Default)]
pub struct TableHeaderItem<'a> {
  id: ColumnId,
  text: &'a str,
  width: u16,
}

pub struct TableItem {
  id: String,
  format: Vec<String>,
}

pub fn draw_help_menu<B>(f: &mut Frame<B>, app: &App)
where
  B: Backend,
{
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(100)].as_ref())
    .margin(2)
    .split(f.size());

  let white = Style::default().fg(app.user_config.theme.text);
  let gray = Style::default().fg(app.user_config.theme.text);
  let header = ["Description", "Event", "Context"];

  let help_docs = get_help_docs();
  let help_docs = &help_docs[app.help_menu_offset as usize..];

  let rows = help_docs
    .iter()
    .map(|item| Row::StyledData(item.iter(), gray));

  let help_menu = Table::new(header.iter(), rows)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .style(white)
        .title("Help (press <Esc> to go back)")
        .title_style(gray)
        .border_style(gray),
    )
    .style(Style::default().fg(app.user_config.theme.text))
    .widths(&[
      Constraint::Length(50),
      Constraint::Length(40),
      Constraint::Length(20),
    ]);
  f.render_widget(help_menu, chunks[0]);
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

  let input_string: String = app.input.iter().collect();
  let lines = [Text::raw(&input_string)];
  let input = Paragraph::new(lines.iter()).block(
    Block::default()
      .borders(Borders::ALL)
      .title("Search")
      .title_style(get_color(highlight_state, app.user_config.theme))
      .border_style(get_color(highlight_state, app.user_config.theme)),
  );
  f.render_widget(input, chunks[0]);

  let show_loading = app.is_loading && app.user_config.behavior.show_loading_indicator;
  let help_block_text = if show_loading {
    (app.user_config.theme.hint, "Loading...")
  } else {
    (app.user_config.theme.inactive, "Type ?")
  };

  let block = Block::default()
    .title("Help")
    .borders(Borders::ALL)
    .border_style(Style::default().fg(help_block_text.0))
    .title_style(Style::default().fg(help_block_text.0));

  let lines = [Text::raw(help_block_text.1)];
  let help = Paragraph::new(lines.iter())
    .block(block)
    .style(Style::default().fg(help_block_text.0));
  f.render_widget(help, chunks[1]);
}

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
  B: Backend,
{
  let margin = util::get_main_layout_margin(app);
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
    .margin(margin)
    .split(f.size());

  // Search input and help
  draw_input_and_help_box(f, app, parent_layout[0]);

  // Nested main block with potential routes
  draw_routes(f, app, parent_layout[1]);

  // Currently playing
  draw_playbar(f, app, parent_layout[2]);

  // Possibly draw confirm dialog
  draw_dialog(f, app);
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
      draw_made_for_you(f, app, chunks[1]);
    }
    RouteId::Artists => {
      draw_artist_table(f, app, chunks[1]);
    }
    RouteId::Podcasts => {
      draw_not_implemented_yet(f, app, chunks[1], ActiveBlock::Podcasts, "Podcasts");
    }
    RouteId::Recommendations => {
      draw_recommendations_table(f, app, chunks[1]);
    }
    RouteId::Error => {} // This is handled as a "full screen" route in main.rs
    RouteId::SelectedDevice => {} // This is handled as a "full screen" route in main.rs
    RouteId::Analysis => {} // This is handled as a "full screen" route in main.rs
    RouteId::BasicView => {} // This is handled as a "full screen" route in main.rs
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
    app,
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
    app,
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
    .constraints(
      [
        Constraint::Percentage(35),
        Constraint::Percentage(35),
        Constraint::Percentage(25),
      ]
      .as_ref(),
    )
    .split(layout_chunk);

  {
    let song_artist_block = Layout::default()
      .direction(Direction::Horizontal)
      .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
      .split(chunks[0]);

    let currently_playing_id = app
      .current_playback_context
      .clone()
      .and_then(|context| {
        context.item.and_then(|item| match item {
          PlayingItem::Track(track) => track.id,
          PlayingItem::Episode(episode) => Some(episode.id),
        })
      })
      .unwrap_or_else(|| "".to_string());

    let songs = match &app.search_results.tracks {
      Some(tracks) => tracks
        .items
        .iter()
        .map(|item| {
          let mut song_name = "".to_string();
          let id = item.clone().id.unwrap_or_else(|| "".to_string());
          if currently_playing_id == id {
            song_name += "▶ "
          }
          if app.liked_song_ids_set.contains(&id) {
            song_name += "♥ ";
          }

          song_name += &item.name;
          song_name += &format!(" - {}", &create_artist_string(&item.artists));
          song_name
        })
        .collect(),
      None => vec![],
    };

    draw_selectable_list(
      f,
      app,
      song_artist_block[0],
      "Songs",
      &songs,
      get_search_results_highlight_state(app, SearchResultBlock::SongSearch),
      app.search_results.selected_tracks_index,
    );

    let artists = match &app.search_results.artists {
      Some(artists) => artists
        .items
        .iter()
        .map(|item| {
          let mut artist = String::new();
          if app.followed_artist_ids_set.contains(&item.id.to_owned()) {
            artist.push_str("♥ ");
          }
          artist.push_str(&item.name.to_owned());
          artist
        })
        .collect(),
      None => vec![],
    };

    draw_selectable_list(
      f,
      app,
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
      Some(albums) => albums
        .items
        .iter()
        .map(|item| {
          let mut album_artist = String::new();
          if let Some(album_id) = &item.id {
            if app.saved_album_ids_set.contains(&album_id.to_owned()) {
              album_artist.push_str("♥ ");
            }
          }
          album_artist.push_str(&format!(
            "{} - {}",
            item.name.to_owned(),
            create_artist_string(&item.artists)
          ));
          album_artist
        })
        .collect(),
      None => vec![],
    };

    draw_selectable_list(
      f,
      app,
      albums_playlist_block[0],
      "Albums",
      &albums,
      get_search_results_highlight_state(app, SearchResultBlock::AlbumSearch),
      app.search_results.selected_album_index,
    );

    let playlists = match &app.search_results.playlists {
      Some(playlists) => playlists
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
      get_search_results_highlight_state(app, SearchResultBlock::PlaylistSearch),
      app.search_results.selected_playlists_index,
    );
  }

  {
    let podcasts_block = Layout::default()
      .direction(Direction::Horizontal)
      .constraints([Constraint::Percentage(100)].as_ref())
      .split(chunks[2]);

    let podcasts = match &app.search_results.shows {
      Some(podcasts) => podcasts
        .items
        .iter()
        .map(|item| format!("{:} - {}", item.name, item.publisher).to_owned())
        .collect(),
      None => vec![],
    };
    draw_selectable_list(
      f,
      app,
      podcasts_block[0],
      "Podcasts",
      &podcasts,
      get_search_results_highlight_state(app, SearchResultBlock::ShowSearch),
      app.search_results.selected_shows_index,
    );
  }
}

struct AlbumUI {
  selected_index: usize,
  items: Vec<TableItem>,
  title: String,
}

pub fn draw_artist_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
  B: Backend,
{
  let header = TableHeader {
    id: TableId::Artist,
    items: vec![TableHeaderItem {
      text: "Artist",
      width: get_percentage_width(layout_chunk.width, 1.0),
      ..Default::default()
    }],
  };

  let current_route = app.get_current_route();
  let highlight_state = (
    current_route.active_block == ActiveBlock::Artists,
    current_route.hovered_block == ActiveBlock::Artists,
  );
  let items = app
    .artists
    .iter()
    .map(|item| TableItem {
      id: item.id.clone(),
      format: vec![item.name.to_owned()],
    })
    .collect::<Vec<TableItem>>();

  draw_table(
    f,
    app,
    layout_chunk,
    ("Artists", &header),
    &items,
    app.artists_list_index,
    highlight_state,
  )
}

pub fn draw_album_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
  B: Backend,
{
  let header = TableHeader {
    id: TableId::Album,
    items: vec![
      TableHeaderItem {
        id: ColumnId::Liked,
        text: "",
        width: 2,
      },
      TableHeaderItem {
        text: "#",
        width: 3,
        ..Default::default()
      },
      TableHeaderItem {
        id: ColumnId::SongTitle,
        text: "Title",
        width: get_percentage_width(layout_chunk.width, 2.0 / 5.0) - 5,
      },
      TableHeaderItem {
        text: "Artist",
        width: get_percentage_width(layout_chunk.width, 2.0 / 5.0),
        ..Default::default()
      },
      TableHeaderItem {
        text: "Length",
        width: get_percentage_width(layout_chunk.width, 1.0 / 5.0),
        ..Default::default()
      },
    ],
  };

  let current_route = app.get_current_route();
  let highlight_state = (
    current_route.active_block == ActiveBlock::AlbumTracks,
    current_route.hovered_block == ActiveBlock::AlbumTracks,
  );

  let album_ui = match &app.album_table_context {
    AlbumTableContext::Simplified => match &app.selected_album_simplified {
      Some(selected_album_simplified) => Some(AlbumUI {
        items: selected_album_simplified
          .tracks
          .items
          .iter()
          .map(|item| TableItem {
            id: item.id.clone().unwrap_or_else(|| "".to_string()),
            format: vec![
              "".to_string(),
              item.track_number.to_string(),
              item.name.to_owned(),
              create_artist_string(&item.artists),
              millis_to_minutes(u128::from(item.duration_ms)),
            ],
          })
          .collect::<Vec<TableItem>>(),
        title: format!(
          "{} by {}",
          selected_album_simplified.album.name,
          create_artist_string(&selected_album_simplified.album.artists)
        ),
        selected_index: selected_album_simplified.selected_index,
      }),
      None => None,
    },
    AlbumTableContext::Full => match app.selected_album_full.clone() {
      Some(selected_album) => Some(AlbumUI {
        items: selected_album
          .album
          .tracks
          .items
          .iter()
          .map(|item| TableItem {
            id: item.id.clone().unwrap_or_else(|| "".to_string()),
            format: vec![
              "".to_string(),
              item.track_number.to_string(),
              item.name.to_owned(),
              create_artist_string(&item.artists),
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

pub fn draw_recommendations_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
  B: Backend,
{
  let header = TableHeader {
    id: TableId::Song,
    items: vec![
      TableHeaderItem {
        id: ColumnId::Liked,
        text: "",
        width: 2,
      },
      TableHeaderItem {
        id: ColumnId::SongTitle,
        text: "Title",
        width: get_percentage_width(layout_chunk.width, 0.3),
      },
      TableHeaderItem {
        text: "Artist",
        width: get_percentage_width(layout_chunk.width, 0.3),
        ..Default::default()
      },
      TableHeaderItem {
        text: "Album",
        width: get_percentage_width(layout_chunk.width, 0.3),
        ..Default::default()
      },
      TableHeaderItem {
        text: "Length",
        width: get_percentage_width(layout_chunk.width, 0.1),
        ..Default::default()
      },
    ],
  };

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
        "".to_string(),
        item.name.to_owned(),
        create_artist_string(&item.artists),
        item.album.name.to_owned(),
        millis_to_minutes(u128::from(item.duration_ms)),
      ],
    })
    .collect::<Vec<TableItem>>();
  // match RecommendedContext
  let recommendations_ui = match &app.recommendations_context {
    Some(RecommendationsContext::Song) => format!(
      "Recommendations based on Song \'{}\'",
      &app.recommendations_seed
    ),
    Some(RecommendationsContext::Artist) => format!(
      "Recommendations based on Artist \'{}\'",
      &app.recommendations_seed
    ),
    None => "Recommendations".to_string(),
  };
  draw_table(
    f,
    app,
    layout_chunk,
    (&recommendations_ui[..], &header),
    &items,
    app.track_table.selected_index,
    highlight_state,
  )
}

pub fn draw_song_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
  B: Backend,
{
  let header = TableHeader {
    id: TableId::Song,
    items: vec![
      TableHeaderItem {
        id: ColumnId::Liked,
        text: "",
        width: 2,
      },
      TableHeaderItem {
        id: ColumnId::SongTitle,
        text: "Title",
        width: get_percentage_width(layout_chunk.width, 0.3),
      },
      TableHeaderItem {
        text: "Artist",
        width: get_percentage_width(layout_chunk.width, 0.3),
        ..Default::default()
      },
      TableHeaderItem {
        text: "Album",
        width: get_percentage_width(layout_chunk.width, 0.3),
        ..Default::default()
      },
      TableHeaderItem {
        text: "Length",
        width: get_percentage_width(layout_chunk.width, 0.1),
        ..Default::default()
      },
    ],
  };

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
        "".to_string(),
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

pub fn draw_basic_view<B>(f: &mut Frame<B>, app: &App)
where
  B: Backend,
{
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
      [
        Constraint::Percentage(44),
        Constraint::Min(6),
        Constraint::Percentage(44),
      ]
      .as_ref(),
    )
    .margin(4)
    .split(f.size());

  draw_playbar(f, app, chunks[1]);
}

pub fn draw_playbar<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
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
        "{:-7} ({} | Shuffle: {:-3} | Repeat: {:-5} | Volume: {:-2}%)",
        play_title,
        current_playback_context.device.name,
        shuffle_text,
        repeat_text,
        current_playback_context.device.volume_percent
      );

      let current_route = app.get_current_route();
      let highlight_state = (
        current_route.active_block == ActiveBlock::PlayBar,
        current_route.hovered_block == ActiveBlock::PlayBar,
      );

      let title_block = Block::default()
        .borders(Borders::ALL)
        .title(&title)
        .title_style(get_color(highlight_state, app.user_config.theme))
        .border_style(get_color(highlight_state, app.user_config.theme));

      f.render_widget(title_block, layout_chunk);

      let (item_id, name, duration_ms) = match track_item {
        PlayingItem::Track(track) => (
          track.id.to_owned().unwrap_or_else(|| "".to_string()),
          track.name.to_owned(),
          track.duration_ms,
        ),
        PlayingItem::Episode(episode) => (
          episode.id.to_owned(),
          episode.name.to_owned(),
          episode.duration_ms,
        ),
      };

      let track_name = if app.liked_song_ids_set.contains(&item_id) {
        format!("♥ {}", name)
      } else {
        name
      };

      let play_bar_text = match track_item {
        PlayingItem::Track(track) => create_artist_string(&track.artists),
        PlayingItem::Episode(episode) => format!("{} - {}", episode.name, episode.show.name),
      };

      let lines = [Text::styled(
        play_bar_text,
        Style::default().fg(app.user_config.theme.playbar_text),
      )];

      let artist = Paragraph::new(lines.iter())
        .style(Style::default().fg(app.user_config.theme.playbar_text))
        .block(
          Block::default().title(&track_name).title_style(
            Style::default()
              .fg(app.user_config.theme.selected)
              .modifier(Modifier::BOLD),
          ),
        );
      f.render_widget(artist, chunks[0]);
      let perc = get_track_progress_percentage(app.song_progress_ms, duration_ms);

      let song_progress_label = display_track_progress(app.song_progress_ms, duration_ms);
      let song_progress = Gauge::default()
        .block(Block::default().title(""))
        .style(
          Style::default()
            .fg(app.user_config.theme.playbar_progress)
            .bg(app.user_config.theme.playbar_background)
            .modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .percent(perc)
        .label(&song_progress_label);
      f.render_widget(song_progress, chunks[1]);
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

  let playing_text = vec![
        Text::raw("Api response: "),
        Text::styled(&app.api_error, Style::default().fg(app.user_config.theme.error_text)),
        Text::styled(
            "

If you are trying to play a track, please check that
    1. You have a Spotify Premium Account
    2. Your playback device is active and selected - press `d` to go to device selection menu
    3. If you're using spotifyd as a playback device, your device name must not contain spaces
            ",
            Style::default().fg(app.user_config.theme.text),
        ),
        Text::styled("
Hint: a playback device must be either an official spotify client or a light weight alternative such as spotifyd
        ",
        Style::default().fg(app.user_config.theme.hint)),
        Text::styled(
            "\nPress <Esc> to return",
            Style::default().fg(app.user_config.theme.inactive),
        ),
    ];

  let playing_paragraph = Paragraph::new(playing_text.iter())
    .wrap(true)
    .style(Style::default().fg(app.user_config.theme.text))
    .block(
      Block::default()
        .borders(Borders::ALL)
        .title("Error")
        .title_style(Style::default().fg(app.user_config.theme.error_border))
        .border_style(Style::default().fg(app.user_config.theme.error_border)),
    );
  f.render_widget(playing_paragraph, chunks[0]);
}

fn draw_home<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
  B: Backend,
{
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(7), Constraint::Length(93)].as_ref())
    .margin(2)
    .split(layout_chunk);

  let current_route = app.get_current_route();
  let highlight_state = (
    current_route.active_block == ActiveBlock::Home,
    current_route.hovered_block == ActiveBlock::Home,
  );

  let welcome = Block::default()
    .title("Welcome!")
    .borders(Borders::ALL)
    .title_style(get_color(highlight_state, app.user_config.theme))
    .border_style(get_color(highlight_state, app.user_config.theme));
  f.render_widget(welcome, layout_chunk);

  let changelog = include_str!("../../CHANGELOG.md").to_string();

  // If debug mode show the "Unreleased" header. Otherwise it is a release so there should be no
  // unreleased features
  let clean_changelog = if cfg!(debug_assertions) {
    changelog
  } else {
    changelog.replace("\n## [Unreleased]\n", "")
  };

  let top_text = vec![Text::styled(
    BANNER,
    Style::default().fg(app.user_config.theme.banner),
  )];

  let bottom_text = vec![
        Text::raw("\nPlease report any bugs or missing features to https://github.com/Rigellute/spotify-tui\n\n"),
        Text::raw(clean_changelog)
    ];

  // Contains the banner
  let top_text = Paragraph::new(top_text.iter())
    .style(Style::default().fg(app.user_config.theme.text))
    .block(Block::default());
  f.render_widget(top_text, chunks[0]);

  // CHANGELOG
  let bottom_text = Paragraph::new(bottom_text.iter())
    .style(Style::default().fg(app.user_config.theme.text))
    .block(Block::default())
    .wrap(true)
    .scroll(app.home_scroll);
  f.render_widget(bottom_text, chunks[1]);
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
    .title_style(get_color(highlight_state, app.user_config.theme))
    .border_style(get_color(highlight_state, app.user_config.theme));

  let text = vec![Text::raw("Not implemented yet!")];

  let not_implemented = Paragraph::new(text.iter())
    .style(Style::default().fg(app.user_config.theme.text))
    .block(display_block)
    .wrap(true);
  f.render_widget(not_implemented, layout_chunk);
}

fn draw_artist_albums<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
  B: Backend,
{
  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
      [
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(33),
      ]
      .as_ref(),
    )
    .split(layout_chunk);

  if let Some(artist) = &app.artist {
    let top_tracks = artist
      .top_tracks
      .iter()
      .map(|top_track| {
        let mut name = String::new();
        if let Some(context) = &app.current_playback_context {
          let track_id = match &context.item {
            Some(PlayingItem::Track(track)) => track.id.to_owned(),
            Some(PlayingItem::Episode(episode)) => Some(episode.id.to_owned()),
            _ => None,
          };

          if track_id == top_track.id {
            name.push_str("▶ ");
          }
        };
        name.push_str(&top_track.name);
        name
      })
      .collect::<Vec<String>>();

    draw_selectable_list(
      f,
      app,
      chunks[0],
      &format!("{} - Top Tracks", &artist.artist_name),
      &top_tracks,
      get_artist_highlight_state(app, ArtistBlock::TopTracks),
      Some(artist.selected_top_track_index),
    );

    let albums = &artist
      .albums
      .items
      .iter()
      .map(|item| {
        let mut album_artist = String::new();
        if let Some(album_id) = &item.id {
          if app.saved_album_ids_set.contains(&album_id.to_owned()) {
            album_artist.push_str("♥ ");
          }
        }
        album_artist.push_str(&format!(
          "{} - {}",
          item.name.to_owned(),
          create_artist_string(&item.artists)
        ));
        album_artist
      })
      .collect::<Vec<String>>();

    draw_selectable_list(
      f,
      app,
      chunks[1],
      "Albums",
      albums,
      get_artist_highlight_state(app, ArtistBlock::Albums),
      Some(artist.selected_album_index),
    );

    let related_artists = artist
      .related_artists
      .iter()
      .map(|item| {
        let mut artist = String::new();
        if app.followed_artist_ids_set.contains(&item.id.to_owned()) {
          artist.push_str("♥ ");
        }
        artist.push_str(&item.name.to_owned());
        artist
      })
      .collect::<Vec<String>>();

    draw_selectable_list(
      f,
      app,
      chunks[2],
      "Related artists",
      &related_artists,
      get_artist_highlight_state(app, ArtistBlock::RelatedArtists),
      Some(artist.selected_related_artist_index),
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
    .margin(5)
    .split(f.size());

  let device_instructions = [
        Text::raw("To play tracks, please select a device. "),
        Text::raw("Use `j/k` or up/down arrow keys to move up and down and <Enter> to select. "),
        Text::raw("Your choice here will be cached so you can jump straight back in when you next open `spotify-tui`. "),
        Text::raw("You can change the playback device at any time by pressing `d`."),
    ];

  let instructions = Paragraph::new(device_instructions.iter())
    .style(Style::default().fg(app.user_config.theme.text))
    .wrap(true)
    .block(
      Block::default()
        .borders(Borders::NONE)
        .title("Welcome to spotify-tui!")
        .title_style(
          Style::default()
            .fg(app.user_config.theme.active)
            .modifier(Modifier::BOLD),
        ),
    );
  f.render_widget(instructions, chunks[0]);

  let no_device_message = vec![Text::raw("No devices found: Make sure a device is active")];

  let items: Box<dyn Iterator<Item = Text>> = match &app.devices {
    Some(items) => {
      if items.devices.is_empty() {
        Box::new(no_device_message.into_iter())
      } else {
        Box::new(items.devices.iter().map(|device| Text::raw(&device.name)))
      }
    }
    None => Box::new(no_device_message.into_iter()),
  };

  let mut state = ListState::default();
  state.select(app.selected_device_index);
  let list = List::new(items)
    .block(
      Block::default()
        .title("Devices")
        .borders(Borders::ALL)
        .title_style(Style::default().fg(app.user_config.theme.active))
        .border_style(Style::default().fg(app.user_config.theme.inactive)),
    )
    .style(Style::default().fg(app.user_config.theme.text))
    .highlight_style(
      Style::default()
        .fg(app.user_config.theme.active)
        .modifier(Modifier::BOLD),
    );
  f.render_stateful_widget(list, chunks[1], &mut state);
}

pub fn draw_album_list<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
  B: Backend,
{
  let header = TableHeader {
    id: TableId::AlbumList,
    items: vec![
      TableHeaderItem {
        text: "Name",
        width: get_percentage_width(layout_chunk.width, 2.0 / 5.0),
        ..Default::default()
      },
      TableHeaderItem {
        text: "Artists",
        width: get_percentage_width(layout_chunk.width, 2.0 / 5.0),
        ..Default::default()
      },
      TableHeaderItem {
        text: "Release Date",
        width: get_percentage_width(layout_chunk.width, 1.0 / 5.0),
        ..Default::default()
      },
    ],
  };

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

pub fn draw_made_for_you<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
  B: Backend,
{
  let header = TableHeader {
    id: TableId::MadeForYou,
    items: vec![TableHeaderItem {
      text: "Name",
      width: get_percentage_width(layout_chunk.width, 2.0 / 5.0),
      ..Default::default()
    }],
  };

  if let Some(playlists) = &app.library.made_for_you_playlists.get_results(None) {
    let items = playlists
      .items
      .iter()
      .map(|playlist| TableItem {
        id: playlist.id.to_owned(),
        format: vec![playlist.name.to_owned()],
      })
      .collect::<Vec<TableItem>>();

    let current_route = app.get_current_route();
    let highlight_state = (
      current_route.active_block == ActiveBlock::MadeForYou,
      current_route.hovered_block == ActiveBlock::MadeForYou,
    );

    draw_table(
      f,
      app,
      layout_chunk,
      ("Made For You", &header),
      &items,
      app.made_for_you_index,
      highlight_state,
    );
  }
}

pub fn draw_recently_played_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
  B: Backend,
{
  let header = TableHeader {
    id: TableId::RecentlyPlayed,
    items: vec![
      TableHeaderItem {
        id: ColumnId::Liked,
        text: "",
        width: 2,
      },
      TableHeaderItem {
        id: ColumnId::SongTitle,
        text: "Title",
        // We need to subtract the fixed value of the previous column
        width: get_percentage_width(layout_chunk.width, 2.0 / 5.0) - 2,
      },
      TableHeaderItem {
        text: "Artist",
        width: get_percentage_width(layout_chunk.width, 2.0 / 5.0),
        ..Default::default()
      },
      TableHeaderItem {
        text: "Length",
        width: get_percentage_width(layout_chunk.width, 1.0 / 5.0),
        ..Default::default()
      },
    ],
  };

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
          "".to_string(),
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
  app: &App,
  layout_chunk: Rect,
  title: &str,
  items: &[S],
  highlight_state: (bool, bool),
  selected_index: Option<usize>,
) where
  B: Backend,
  S: std::convert::AsRef<str>,
{
  let mut state = ListState::default();
  state.select(selected_index);

  let list = List::new(items.iter().map(|i| Text::raw(i.as_ref())))
    .block(
      Block::default()
        .title(title)
        .borders(Borders::ALL)
        .title_style(get_color(highlight_state, app.user_config.theme))
        .border_style(get_color(highlight_state, app.user_config.theme)),
    )
    .style(Style::default().fg(app.user_config.theme.text))
    .highlight_style(get_color(highlight_state, app.user_config.theme).modifier(Modifier::BOLD));
  f.render_stateful_widget(list, layout_chunk, &mut state);
}

fn draw_dialog<B>(f: &mut Frame<B>, app: &App)
where
  B: Backend,
{
  if let ActiveBlock::Dialog(_) = app.get_current_route().active_block {
    if let Some(playlist) = app.dialog.as_ref() {
      let bounds = f.size();
      // maybe do this better
      let width = std::cmp::min(bounds.width - 2, 45);
      let height = 8;
      let left = (bounds.width - width) / 2;
      let top = bounds.height / 4;

      let rect = Rect::new(left, top, width, height);

      f.render_widget(Clear, rect);

      let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.user_config.theme.inactive));

      f.render_widget(block, rect);

      let vchunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
        .split(rect);

      // suggestion: possibly put this as part of
      // app.dialog, but would have to introduce lifetime
      let text = [
        Text::raw("Are you sure you want to delete\nthe playlist: "),
        Text::styled(playlist.as_str(), Style::default().modifier(Modifier::BOLD)),
        Text::raw("?"),
      ];

      let text = Paragraph::new(text.iter()).alignment(Alignment::Center);

      f.render_widget(text, vchunks[0]);

      let hchunks = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
        .split(vchunks[1]);

      let ok_text = [Text::raw("Ok")];
      let ok = Paragraph::new(ok_text.iter())
        .style(Style::default().fg(if app.confirm {
          app.user_config.theme.hovered
        } else {
          app.user_config.theme.inactive
        }))
        .alignment(Alignment::Center);

      f.render_widget(ok, hchunks[0]);

      let cancel_text = [Text::raw("Cancel")];
      let cancel = Paragraph::new(cancel_text.iter())
        .style(Style::default().fg(if app.confirm {
          app.user_config.theme.inactive
        } else {
          app.user_config.theme.hovered
        }))
        .alignment(Alignment::Center);

      f.render_widget(cancel, hchunks[1]);
    }
  }
}

fn draw_table<B>(
  f: &mut Frame<B>,
  app: &App,
  layout_chunk: Rect,
  table_layout: (&str, &TableHeader), // (title, header colums)
  items: &[TableItem], // The nested vector must have the same length as the `header_columns`
  selected_index: usize,
  highlight_state: (bool, bool),
) where
  B: Backend,
{
  let selected_style = get_color(highlight_state, app.user_config.theme).modifier(Modifier::BOLD);

  let track_playing_index = app.current_playback_context.to_owned().and_then(|ctx| {
    ctx.item.and_then(|item| match item {
      PlayingItem::Track(track) => items
        .iter()
        .position(|item| track.id.to_owned().map(|id| id == item.id).unwrap_or(false)),
      PlayingItem::Episode(_episode) => None,
    })
  });

  let (title, header) = table_layout;

  // Make sure that the selected item is visible on the page. Need to add some rows of padding
  // to chunk height for header and header space to get a true table height
  let padding = 5;
  let offset = layout_chunk
    .height
    .checked_sub(padding)
    .and_then(|height| selected_index.checked_sub(height as usize))
    .unwrap_or(0);

  let rows = items.iter().skip(offset).enumerate().map(|(i, item)| {
    let mut formatted_row = item.format.clone();
    let mut style = Style::default().fg(app.user_config.theme.text); // default styling

    // if table displays songs
    match header.id {
      TableId::Song | TableId::RecentlyPlayed | TableId::Album => {
        // First check if the song should be highlighted because it is currently playing
        if let Some(title_idx) = header.get_index(ColumnId::SongTitle) {
          if let Some(track_playing_offset_index) =
            track_playing_index.and_then(|idx| idx.checked_sub(offset))
          {
            if i == track_playing_offset_index {
              formatted_row[title_idx] = format!("▶ {}", &formatted_row[title_idx]);
              style = Style::default()
                .fg(app.user_config.theme.active)
                .modifier(Modifier::BOLD);
            }
          }
        }

        // Show this ♥ if the song is liked
        if let Some(liked_idx) = header.get_index(ColumnId::Liked) {
          if app.liked_song_ids_set.contains(item.id.as_str()) {
            formatted_row[liked_idx] = " ♥".to_string();
          }
        }
      }
      _ => {}
    }

    // Next check if the item is under selection.
    if Some(i) == selected_index.checked_sub(offset) {
      style = selected_style;
    }

    // Return row styled data
    Row::StyledData(formatted_row.into_iter(), style)
  });

  let widths = header
    .items
    .iter()
    .map(|h| Constraint::Length(h.width))
    .collect::<Vec<tui::layout::Constraint>>();

  let table = Table::new(header.items.iter().map(|h| h.text), rows)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(app.user_config.theme.text))
        .title(title)
        .title_style(get_color(highlight_state, app.user_config.theme))
        .border_style(get_color(highlight_state, app.user_config.theme)),
    )
    .style(Style::default().fg(app.user_config.theme.text))
    .widths(&widths);
  f.render_widget(table, layout_chunk);
}
