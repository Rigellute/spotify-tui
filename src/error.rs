use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum VisualizationError {
  Warning(String),
}

impl From<String> for VisualizationError {
  fn from(err: String) -> VisualizationError {
    VisualizationError::Warning(err)
  }
}

impl<'a> From<&'a str> for VisualizationError {
  fn from(err: &'a str) -> VisualizationError {
    VisualizationError::Warning(String::from(err))
  }
}

impl fmt::Display for VisualizationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      VisualizationError::Warning(ref warning) => write!(f, "Visulization Error: {}", warning),
    }
  }
}

impl error::Error for VisualizationError {
  fn description(&self) -> &str {
    match *self {
      VisualizationError::Warning(ref warning) => warning,
    }
  }
}
