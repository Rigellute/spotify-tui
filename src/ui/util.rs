use super::super::app::{ActiveBlock, App, ArtistBlock, SearchResultBlock};
use crate::user_config::Theme;
use rspotify::model::artist::SimplifiedArtist;
use tui::style::Style;

pub const BASIC_VIEW_HEIGHT: u16 = 6;
pub const SMALL_TERMINAL_WIDTH: u16 = 150;
pub const SMALL_TERMINAL_HEIGHT: u16 = 45;

pub fn get_search_results_highlight_state(
  app: &App,
  block_to_match: SearchResultBlock,
) -> (bool, bool) {
  let current_route = app.get_current_route();
  (
    app.search_results.selected_block == block_to_match,
    current_route.hovered_block == ActiveBlock::SearchResultBlock
      && app.search_results.hovered_block == block_to_match,
  )
}

pub fn get_artist_highlight_state(app: &App, block_to_match: ArtistBlock) -> (bool, bool) {
  let current_route = app.get_current_route();
  if let Some(artist) = &app.artist {
    let is_hovered = artist.artist_selected_block == block_to_match;
    let is_selected = current_route.hovered_block == ActiveBlock::ArtistBlock
      && artist.artist_hovered_block == block_to_match;
    (is_hovered, is_selected)
  } else {
    (false, false)
  }
}

pub fn get_color((is_active, is_hovered): (bool, bool), theme: Theme) -> Style {
  match (is_active, is_hovered) {
    (true, _) => Style::default().fg(theme.selected),
    (false, true) => Style::default().fg(theme.hovered),
    _ => Style::default().fg(theme.inactive),
  }
}

pub fn create_artist_string(artists: &[SimplifiedArtist]) -> String {
  artists
    .iter()
    .map(|artist| artist.name.to_string())
    .collect::<Vec<String>>()
    .join(", ")
}

pub fn millis_to_minutes(millis: u128) -> String {
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

pub fn display_track_progress(progress: u128, track_duration: u32) -> String {
  let duration = millis_to_minutes(u128::from(track_duration));
  let progress_display = millis_to_minutes(progress);
  let remaining = millis_to_minutes(u128::from(track_duration).saturating_sub(progress));

  format!("{}/{} (-{})", progress_display, duration, remaining,)
}

// `percentage` param needs to be between 0 and 1
pub fn get_percentage_width(width: u16, percentage: f32) -> u16 {
  let padding = 3;
  let width = width - padding;
  (f32::from(width) * percentage) as u16
}

// Ensure track progress percentage is between 0 and 100 inclusive
pub fn get_track_progress_percentage(song_progress_ms: u128, track_duration_ms: u32) -> u16 {
  let min_perc = 0_f64;
  let track_progress = std::cmp::min(song_progress_ms, track_duration_ms.into());
  let track_perc = (track_progress as f64 / f64::from(track_duration_ms)) * 100_f64;
  min_perc.max(track_perc) as u16
}

// Make better use of space on small terminals
pub fn get_main_layout_margin(app: &App) -> u16 {
  if app.size.height > SMALL_TERMINAL_HEIGHT {
    1
  } else {
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn millis_to_minutes_test() {
    assert_eq!(millis_to_minutes(0), "0:00");
    assert_eq!(millis_to_minutes(1000), "0:01");
    assert_eq!(millis_to_minutes(1500), "0:01");
    assert_eq!(millis_to_minutes(1900), "0:01");
    assert_eq!(millis_to_minutes(60 * 1000), "1:00");
    assert_eq!(millis_to_minutes(60 * 1500), "1:30");
  }

  #[test]
  fn display_track_progress_test() {
    assert_eq!(
      display_track_progress(0, 2 * 60 * 1000),
      "0:00/2:00 (-2:00)"
    );

    assert_eq!(
      display_track_progress(60 * 1000, 2 * 60 * 1000),
      "1:00/2:00 (-1:00)"
    );
  }

  #[test]
  fn get_track_progress_percentage_test() {
    let track_length = 60 * 1000;
    assert_eq!(get_track_progress_percentage(0, track_length), 0);
    assert_eq!(
      get_track_progress_percentage((60 * 1000) / 2, track_length),
      50
    );

    // If progress is somehow higher than total duration, 100 should be max
    assert_eq!(
      get_track_progress_percentage(60 * 1000 * 2, track_length),
      100
    );
  }
}
