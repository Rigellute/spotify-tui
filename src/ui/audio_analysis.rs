use super::util;
use crate::app::App;
use crate::error::VisualizationError;
use crate::user_config::VisualStyle;
use rhai::{
  serde::{from_dynamic, to_dynamic},
  Array, Engine, Scope,
};
use tui::{
  backend::Backend,
  layout::{Constraint, Direction, Layout},
  style::Style,
  text::{Span, Spans},
  widgets::{BarChart, Block, Borders, Paragraph},
  Frame,
};

// Trait to produce widget
#[derive(Debug, Clone, serde::Deserialize)]
struct BarChartInfo {
  error: bool,
  labels: Vec<String>,
  counts: Vec<u64>,
}

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

  let visual_app = app.user_config.get_visualizer_or_default();
  let white = Style::default().fg(app.user_config.theme.text);
  let gray = Style::default().fg(app.user_config.theme.inactive);
  let tick_rate = app.user_config.behavior.tick_rate_milliseconds;
  let bar_chart_title = &format!(
    "{} | Tick Rate {} {}FPS",
    visual_app.name,
    tick_rate,
    1000 / tick_rate,
  );
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
      match &app.visualizer {
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
                      item.clone().try_cast::<String>().unwrap_or_else(|| {
                        "Bad analysis provided, could not cast to String.".to_string()
                      })
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

    let texts: Vec<Spans> = info.iter().map(|span| Spans::from(span.clone())).collect();
    let p = Paragraph::new(texts)
      .block(analysis_block)
      .style(Style::default().fg(app.user_config.theme.text));
    f.render_widget(p, chunks[0]);

    // TODO: Pass back a struct with render
    // Struct should have optional error
    // Match against config
    // Invalid viz
    match visual_app.style {
      VisualStyle::Bar => {
        let data: Result<Vec<(String, u64)>, VisualizationError> = {
          // TODO: Set up raw engine
          match &app.visualizer {
            Ok(ast) => {
              let engine = Engine::new();
              match to_dynamic(analysis.clone()) {
                Ok(dynamic_analysis) => {
                  match engine.call_fn(
                    &mut Scope::new(),
                    &ast,
                    "draw",
                    (dynamic_analysis, progress_seconds),
                  ) {
                    Ok(data) => match from_dynamic(&data) {
                      Ok(data) => {
                        let data: BarChartInfo = data;
                        Ok(
                          data
                            .labels
                            .iter()
                            .zip(data.counts.iter())
                            .map(|(label, count)| (label.clone(), *count))
                            .collect(),
                        )
                      }
                      Err(err) => Err(VisualizationError::from(format!(
                        "Unable to type cast: {}",
                        err
                      ))),
                    },
                    Err(err) => Err(VisualizationError::from(format!(
                      "Error in script: {}",
                      err
                    ))),
                  }
                }
                Err(err) => Err(VisualizationError::from(format!(
                  "Unable to serialize spotify information: {}",
                  err
                ))),
              }
            }
            Err(err) => Err(VisualizationError::from(format!(
              "Compilation error: {}",
              err
            ))),
          }
        };
        match data {
          Ok(data) => {
            let data: Vec<(&str, u64)> =
              data.iter().map(|item| (item.0.as_str(), item.1)).collect();
            let width = (chunks[1].width) as f32 / (1 + data.len()) as f32;

            let analysis_bar = BarChart::default()
              .block(bar_chart_block)
              .data(data.as_slice())
              .bar_width(width as u16)
              .bar_style(Style::default().fg(app.user_config.theme.analysis_bar))
              .value_style(
                Style::default()
                  .fg(app.user_config.theme.analysis_bar_text)
                  .bg(app.user_config.theme.analysis_bar),
              );
            f.render_widget(analysis_bar, chunks[1]);
          }
          Err(VisualizationError::Warning(message)) => {
            let ts: Vec<Spans> = vec![Spans::from(message)];
            let p = Paragraph::new(ts)
              .block(bar_chart_block)
              .style(Style::default().fg(app.user_config.theme.text));
            f.render_widget(p, chunks[1]);
          }
        }
      }
      _ => {
        let ts: Vec<Spans> = vec![Spans::from("Unsupported type.")];
        let p = Paragraph::new(ts)
          .block(bar_chart_block)
          .style(Style::default().fg(app.user_config.theme.text));
        f.render_widget(p, chunks[1]);
      }
    }
  } else {
    f.render_widget(empty_analysis_block(), chunks[0]);
    f.render_widget(empty_pitches_block(), chunks[1]);
  }
}
