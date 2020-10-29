use super::util;
use crate::app::App;
use rhai::serde::to_dynamic;
use rhai::{Array, Dynamic, Map};
use rhai::{Engine, Scope};
use tui::{
  backend::Backend,
  layout::{Constraint, Direction, Layout},
  style::Style,
  text::{Span, Spans},
  widgets::{BarChart, Block, Borders, Paragraph},
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
    .constraints([Constraint::Min(5), Constraint::Length(75)].as_ref())
    .margin(margin)
    .split(f.size());

  let analysis_block = Block::default()
    .title(Span::styled(
      "Analysis",
      Style::default().fg(app.user_config.theme.inactive),
    ))
    .borders(Borders::ALL)
    .border_style(Style::default().fg(app.user_config.theme.inactive));

  let white = Style::default().fg(app.user_config.theme.text);
  let gray = Style::default().fg(app.user_config.theme.inactive);
  let width = (chunks[1].width) as f32 / (1 + PITCHES.len()) as f32;
  let tick_rate = app.user_config.behavior.tick_rate_milliseconds;
  let bar_chart_title = &format!("Pitches | Tick Rate {} {}FPS", tick_rate, 1000 / tick_rate,);
  let bar_chart_block = Block::default()
    .borders(Borders::ALL)
    .style(white)
    .title(Span::styled(bar_chart_title, gray))
    .border_style(gray);

  let empty_analysis_block = || {
    Paragraph::new("No analysis available")
      .block(analysis_block.clone())
      .style(Style::default().fg(app.user_config.theme.text))
  };
  let empty_pitches_block = || {
    Paragraph::new("No pitch information available")
      .block(bar_chart_block.clone())
      .style(Style::default().fg(app.user_config.theme.text))
  };

  if let Some(analysis) = &app.audio_analysis {
    let progress_seconds = (app.song_progress_ms as f32) / 1000.0;

    // TODO: Move to own function
    let info: Vec<String> = {
      // TODO: Set up raw engine
      match &app.visulizer {
        Ok(ast) => {
          let engine = Engine::new();
          match to_dynamic(analysis.clone()) {
            Ok(dynamic_analysis) => {
              match engine.call_fn(
                &mut Scope::new(),
                &ast,
                "analysis",
                (dynamic_analysis, progress_seconds),
              ) {
                Ok(txt) => {
                  let txt: Array = txt;
                  txt
                    .to_vec()
                    .iter()
                    .map(|item| {
                      item
                        .clone()
                        .try_cast::<String>()
                        .unwrap_or("Bad analysis provided, could not cast to String.".to_string())
                    })
                    .collect()
                }
                Err(err) => vec!["Error in script".to_string(), format!("{}", err)],
              }
            }
            Err(err) => vec!["Spotify error".to_string(), format!("{}", err)],
          }
        }
        Err(err) => vec!["Script compilation error".to_string(), format!("{}", err)],
      }
    };

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
      .find(|segment| segment.start >= progress_seconds);

    if let Some(segment) = segment {
      let texts: Vec<Spans> = info.iter().map(|span| Spans::from(span.clone())).collect();
      let p = Paragraph::new(texts)
        .block(analysis_block)
        .style(Style::default().fg(app.user_config.theme.text));
      f.render_widget(p, chunks[0]);

      let data: Vec<(&str, u64)> = segment
        .clone()
        .pitches
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
        .bar_style(Style::default().fg(app.user_config.theme.analysis_bar))
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
