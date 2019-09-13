use super::super::app::{ActiveBlock, App, SearchResultBlock};
use rspotify::spotify::model::artist::SimplifiedArtist;
use tui::style::{Color, Style};

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

pub fn get_color((is_active, is_hovered): (bool, bool)) -> Style {
    match (is_active, is_hovered) {
        (true, _) => Style::default().fg(Color::LightCyan),
        (false, true) => Style::default().fg(Color::Magenta),
        _ => Style::default().fg(Color::Gray),
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
    let remaining = millis_to_minutes(u128::from(track_duration) - progress);

    format!("{}/{} (-{})", progress_display, duration, remaining,)
}

// `percentage` param needs to be between 0 and 1
pub fn get_percentage_width(width: u16, percentage: f32) -> u16 {
    let padding = 3;
    let width = width - padding;
    (f32::from(width) * percentage) as u16
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
}
