use super::util;
use crate::app::App;
use tui::{
  backend::Backend,
  layout::{Constraint, Direction, Layout},
  style::Style,
  widgets::{BarChart, Block, Borders, Paragraph, Text},
  Frame,
};
const PITCHES: [&str; 12] = [
  "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

pub fn draw<B>(f: &mut Frame<B>, app: &App)
where
  B: Backend,
{
  let margin = util::get_main_layout_margin(app);

  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Min(5), Constraint::Length(95)].as_ref())
    .margin(margin)
    .split(f.size());

  let analysis_block = Block::default()
    .title("Analysis")
    .borders(Borders::ALL)
    .border_style(Style::default().fg(app.user_config.theme.inactive))
    .title_style(Style::default().fg(app.user_config.theme.inactive));

  let white = Style::default().fg(app.user_config.theme.text);
  let gray = Style::default().fg(app.user_config.theme.inactive);
  let width = (chunks[1].width) as f32 / (1 + PITCHES.len()) as f32;
  let tick_rate = app.user_config.behavior.tick_rate_milliseconds;
  let bar_chart_title = &format!("Pitches | Tick Rate {} {}FPS", tick_rate, 1000 / tick_rate);

  let bar_chart_block = Block::default()
    .borders(Borders::ALL)
    .style(white)
    .title(bar_chart_title)
    .title_style(gray)
    .border_style(gray);

  let analysis_text = [Text::raw("No analysis available")];
  let empty_analysis_block = || {
    Paragraph::new(analysis_text.iter())
      .block(analysis_block)
      .style(Style::default().fg(app.user_config.theme.text))
  };
  let pitch_text = [Text::raw("No pitch information available")];
  let empty_pitches_block = || {
    Paragraph::new(pitch_text.iter())
      .block(bar_chart_block)
      .style(Style::default().fg(app.user_config.theme.text))
  };

  if let Some(analysis) = &app.audio_analysis {
    let progress_seconds = (app.song_progress_ms as f32) / 1000.0;

    let beat = analysis
      .beats
      .iter()
      .find(|beat| beat.start >= progress_seconds);

    let beat_offset = beat
      .map(|beat| beat.start - progress_seconds)
      .unwrap_or(0.0);
    let segment = analysis
      .segments
      .iter()
      .find(|segment| segment.start.unwrap_or(0.0) >= progress_seconds);
    let section = analysis
      .sections
      .iter()
      .find(|section| section.start.unwrap_or(0.0) >= progress_seconds);

    if let (Some(segment), Some(section)) = (segment, section) {
      let texts = [
        Text::raw(format!(
          "Tempo: {} (confidence {:.0}%)\n",
          section.tempo.unwrap_or(0.0),
          section.tempo_confidence.unwrap_or(0.0) * 100.0
        )),
        Text::raw(format!(
          "Key: {} (confidence {:.0}%)\n",
          PITCHES
            .get(section.key.unwrap_or(0) as usize)
            .unwrap_or(&PITCHES[0]),
          section.key_confidence.unwrap_or(0.0) * 100.0
        )),
        Text::raw(format!(
          "Time Signature: {}/4 (confidence {:.0}%)\n",
          section.time_signature.unwrap_or(0),
          section.time_signature_confidence.unwrap_or(0.0) * 100.0
        )),
      ];
      let p = Paragraph::new(texts.iter())
        .block(analysis_block)
        .style(Style::default().fg(app.user_config.theme.text));
      f.render_widget(p, chunks[0]);

      let data: Vec<(&str, u64)> = segment
        .clone()
        .pitches
        .unwrap_or_default()
        .iter()
        .enumerate()
        .map(|(index, pitch)| {
          let display_pitch = *PITCHES.get(index).unwrap_or(&PITCHES[0]);
          let bar_value = ((pitch * 1000.0) as u64)
            // Add a beat offset to make the bar animate between beats
            .checked_add((beat_offset * 3000.0) as u64)
            .unwrap_or(0);

          (display_pitch, bar_value)
        })
        .collect();

      let analysis_bar = BarChart::default()
        .block(bar_chart_block)
        .data(&data)
        .bar_width(width as u16)
        .style(Style::default().fg(app.user_config.theme.analysis_bar))
        .value_style(
          Style::default()
            .fg(app.user_config.theme.analysis_bar_text)
            .bg(app.user_config.theme.analysis_bar),
        );
      f.render_widget(analysis_bar, chunks[1]);
    } else {
      f.render_widget(empty_analysis_block(), chunks[0]);
      f.render_widget(empty_pitches_block(), chunks[1]);
    };
  } else {
    f.render_widget(empty_analysis_block(), chunks[0]);
    f.render_widget(empty_pitches_block(), chunks[1]);
  }
}
