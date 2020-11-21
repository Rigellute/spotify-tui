use crate::error::VisualizationError;
use crate::event::Key;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
  fs,
  path::{Path, PathBuf},
};
use tui::style::Color;

const FILE_NAME: &str = "config.yml";
const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "spotify-tui";
const PLUGIN_DIR: &str = "visulizations";
const PLUGIN_EXTENSION: &str = "rhai";

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UserTheme {
  pub active: Option<String>,
  pub banner: Option<String>,
  pub error_border: Option<String>,
  pub error_text: Option<String>,
  pub hint: Option<String>,
  pub hovered: Option<String>,
  pub inactive: Option<String>,
  pub playbar_background: Option<String>,
  pub playbar_progress: Option<String>,
  pub playbar_progress_text: Option<String>,
  pub playbar_text: Option<String>,
  pub selected: Option<String>,
  pub text: Option<String>,
  pub header: Option<String>,
}

#[derive(Copy, Clone, Debug)]
pub struct Theme {
  pub analysis_bar: Color,
  pub analysis_bar_text: Color,
  pub active: Color,
  pub banner: Color,
  pub error_border: Color,
  pub error_text: Color,
  pub hint: Color,
  pub hovered: Color,
  pub inactive: Color,
  pub playbar_background: Color,
  pub playbar_progress: Color,
  pub playbar_progress_text: Color,
  pub playbar_text: Color,
  pub selected: Color,
  pub text: Color,
  pub header: Color,
}

impl Default for Theme {
  fn default() -> Self {
    Theme {
      analysis_bar: Color::LightCyan,
      analysis_bar_text: Color::Black,
      active: Color::Cyan,
      banner: Color::LightCyan,
      error_border: Color::Red,
      error_text: Color::LightRed,
      hint: Color::Yellow,
      hovered: Color::Magenta,
      inactive: Color::Gray,
      playbar_background: Color::Black,
      playbar_progress: Color::LightCyan,
      playbar_progress_text: Color::LightCyan,
      playbar_text: Color::White,
      selected: Color::LightCyan,
      text: Color::White,
      header: Color::White,
    }
  }
}

fn parse_key(key: String) -> Result<Key> {
  fn get_single_char(string: &str) -> char {
    match string.chars().next() {
      Some(c) => c,
      None => panic!(),
    }
  }

  match key.len() {
    1 => Ok(Key::Char(get_single_char(key.as_str()))),
    _ => {
      let sections: Vec<&str> = key.split('-').collect();

      if sections.len() > 2 {
        return Err(anyhow!(
          "Shortcut can only have 2 keys, \"{}\" has {}",
          key,
          sections.len()
        ));
      }

      match sections[0].to_lowercase().as_str() {
        "ctrl" => Ok(Key::Ctrl(get_single_char(sections[1]))),
        "alt" => Ok(Key::Alt(get_single_char(sections[1]))),
        "left" => Ok(Key::Left),
        "right" => Ok(Key::Right),
        "up" => Ok(Key::Up),
        "down" => Ok(Key::Down),
        "backspace" | "delete" => Ok(Key::Backspace),
        "del" => Ok(Key::Delete),
        "esc" | "escape" => Ok(Key::Esc),
        "pageup" => Ok(Key::PageUp),
        "pagedown" => Ok(Key::PageDown),
        "space" => Ok(Key::Char(' ')),
        _ => Err(anyhow!("The key \"{}\" is unknown.", sections[0])),
      }
    }
  }
}

fn check_reserved_keys(key: Key) -> Result<()> {
  let reserved = [
    Key::Char('h'),
    Key::Char('j'),
    Key::Char('k'),
    Key::Char('l'),
    Key::Char('H'),
    Key::Char('M'),
    Key::Char('L'),
    Key::Up,
    Key::Down,
    Key::Left,
    Key::Right,
    Key::Backspace,
    Key::Enter,
  ];
  for item in reserved.iter() {
    if key == *item {
      // TODO: Add pretty print for key
      return Err(anyhow!(
        "The key {:?} is reserved and cannot be remapped",
        key
      ));
    }
  }
  Ok(())
}

#[derive(Clone)]
pub struct UserConfigPaths {
  pub config_file_path: PathBuf,
}
#[derive(Clone)]
pub struct PluginPaths {
  pub plugin_path: PathBuf,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyBindingsString {
  back: Option<String>,
  next_page: Option<String>,
  previous_page: Option<String>,
  jump_to_start: Option<String>,
  jump_to_end: Option<String>,
  jump_to_album: Option<String>,
  jump_to_artist_album: Option<String>,
  jump_to_context: Option<String>,
  manage_devices: Option<String>,
  decrease_volume: Option<String>,
  increase_volume: Option<String>,
  toggle_playback: Option<String>,
  seek_backwards: Option<String>,
  seek_forwards: Option<String>,
  next_track: Option<String>,
  previous_track: Option<String>,
  help: Option<String>,
  shuffle: Option<String>,
  repeat: Option<String>,
  search: Option<String>,
  submit: Option<String>,
  copy_song_url: Option<String>,
  copy_album_url: Option<String>,
  audio_analysis: Option<String>,
  basic_view: Option<String>,
  add_item_to_queue: Option<String>,
}

#[derive(Clone)]
pub struct KeyBindings {
  pub back: Key,
  pub next_page: Key,
  pub previous_page: Key,
  pub jump_to_start: Key,
  pub jump_to_end: Key,
  pub jump_to_album: Key,
  pub jump_to_artist_album: Key,
  pub jump_to_context: Key,
  pub manage_devices: Key,
  pub decrease_volume: Key,
  pub increase_volume: Key,
  pub toggle_playback: Key,
  pub seek_backwards: Key,
  pub seek_forwards: Key,
  pub next_track: Key,
  pub previous_track: Key,
  pub help: Key,
  pub shuffle: Key,
  pub repeat: Key,
  pub search: Key,
  pub submit: Key,
  pub copy_song_url: Key,
  pub copy_album_url: Key,
  pub audio_analysis: Key,
  pub basic_view: Key,
  pub add_item_to_queue: Key,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BehaviorConfigString {
  pub seek_milliseconds: Option<u32>,
  pub volume_increment: Option<u8>,
  pub tick_rate_milliseconds: Option<u64>,
  pub enable_text_emphasis: Option<bool>,
  pub show_loading_indicator: Option<bool>,
}

#[derive(Clone)]
pub struct BehaviorConfig {
  pub seek_milliseconds: u32,
  pub volume_increment: u8,
  pub tick_rate_milliseconds: u64,
  pub enable_text_emphasis: bool,
  pub show_loading_indicator: bool,
}

#[derive(Clone)]
pub enum VisualStyle {
  Bar,
  Chart,
  Invalid,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VisualsString {
  pub default: Option<String>,
  pub bar: Vec<String>,
  pub chart: Vec<String>,
}

#[derive(Clone)]
pub struct VisualApp {
  pub warning: Option<VisualizationError>,
  pub path: PathBuf,
  pub style: VisualStyle,
  pub name: String,
}

#[derive(Clone)]
pub struct VisualPlugins {
  pub current: Option<usize>,
  pub plugins: Vec<VisualApp>,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserConfigString {
  keybindings: Option<KeyBindingsString>,
  behavior: Option<BehaviorConfigString>,
  theme: Option<UserTheme>,
  visuals: Option<VisualsString>,
}

#[derive(Clone)]
pub struct UserConfig {
  pub keys: KeyBindings,
  pub theme: Theme,
  pub behavior: BehaviorConfig,
  pub visuals: VisualPlugins,
  pub path_to_config: Option<UserConfigPaths>,
  pub path_to_plugin: Option<PluginPaths>,
}

impl UserConfig {
  pub fn new() -> UserConfig {
    UserConfig {
      theme: Default::default(),
      keys: KeyBindings {
        back: Key::Char('q'),
        next_page: Key::Ctrl('d'),
        previous_page: Key::Ctrl('u'),
        jump_to_start: Key::Ctrl('a'),
        jump_to_end: Key::Ctrl('e'),
        jump_to_album: Key::Char('a'),
        jump_to_artist_album: Key::Char('A'),
        jump_to_context: Key::Char('o'),
        manage_devices: Key::Char('d'),
        decrease_volume: Key::Char('-'),
        increase_volume: Key::Char('+'),
        toggle_playback: Key::Char(' '),
        seek_backwards: Key::Char('<'),
        seek_forwards: Key::Char('>'),
        next_track: Key::Char('n'),
        previous_track: Key::Char('p'),
        help: Key::Char('?'),
        shuffle: Key::Ctrl('s'),
        repeat: Key::Ctrl('r'),
        search: Key::Char('/'),
        submit: Key::Enter,
        copy_song_url: Key::Char('c'),
        copy_album_url: Key::Char('C'),
        audio_analysis: Key::Char('v'),
        basic_view: Key::Char('B'),
        add_item_to_queue: Key::Char('z'),
      },
      behavior: BehaviorConfig {
        seek_milliseconds: 5 * 1000,
        volume_increment: 10,
        tick_rate_milliseconds: 250,
        enable_text_emphasis: true,
        show_loading_indicator: true,
      },
      visuals: VisualPlugins {
        current: None,
        plugins: vec![],
      },
      path_to_config: None,
      path_to_plugin: None,
    }
  }

  pub fn get_or_build_paths(&mut self) -> Result<()> {
    match dirs::home_dir() {
      Some(home) => {
        if self.path_to_plugin.is_none() || self.path_to_config.is_none() {
          let path = Path::new(&home);
          let home_config_dir = path.join(CONFIG_DIR);
          let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);
          let plugin_dir = app_config_dir.join(PLUGIN_DIR);

          if !home_config_dir.exists() {
            fs::create_dir(&home_config_dir)?;
          }

          if !app_config_dir.exists() {
            fs::create_dir(&app_config_dir)?;
          }

          let config_file_path = &app_config_dir.join(FILE_NAME);

          if self.path_to_config.is_none() {
            let paths = UserConfigPaths {
              config_file_path: config_file_path.to_path_buf(),
            };
            self.path_to_config = Some(paths);
          }

          if self.path_to_plugin.is_none() {
            let paths = PluginPaths {
              plugin_path: plugin_dir,
            };
            self.path_to_plugin = Some(paths);
          }
        }

        Ok(())
      }
      None => Err(anyhow!("No $HOME directory found for client config")),
    }
  }

  pub fn load_keybindings(&mut self, keybindings: KeyBindingsString) -> Result<()> {
    macro_rules! to_keys {
      ($name: ident) => {
        if let Some(key_string) = keybindings.$name {
          self.keys.$name = parse_key(key_string)?;
          check_reserved_keys(self.keys.$name)?;
        }
      };
    };

    to_keys!(back);
    to_keys!(next_page);
    to_keys!(previous_page);
    to_keys!(jump_to_start);
    to_keys!(jump_to_end);
    to_keys!(jump_to_album);
    to_keys!(jump_to_artist_album);
    to_keys!(jump_to_context);
    to_keys!(manage_devices);
    to_keys!(decrease_volume);
    to_keys!(increase_volume);
    to_keys!(toggle_playback);
    to_keys!(seek_backwards);
    to_keys!(seek_forwards);
    to_keys!(next_track);
    to_keys!(previous_track);
    to_keys!(help);
    to_keys!(shuffle);
    to_keys!(repeat);
    to_keys!(search);
    to_keys!(submit);
    to_keys!(copy_song_url);
    to_keys!(copy_album_url);
    to_keys!(audio_analysis);
    to_keys!(basic_view);
    to_keys!(add_item_to_queue);

    Ok(())
  }

  pub fn load_theme(&mut self, theme: UserTheme) -> Result<()> {
    macro_rules! to_theme_item {
      ($name: ident) => {
        if let Some(theme_item) = theme.$name {
          self.theme.$name = parse_theme_item(&theme_item)?;
        }
      };
    };

    to_theme_item!(active);
    to_theme_item!(banner);
    to_theme_item!(error_border);
    to_theme_item!(error_text);
    to_theme_item!(hint);
    to_theme_item!(hovered);
    to_theme_item!(inactive);
    to_theme_item!(playbar_background);
    to_theme_item!(playbar_progress);
    to_theme_item!(playbar_progress_text);
    to_theme_item!(playbar_text);
    to_theme_item!(selected);
    to_theme_item!(text);
    to_theme_item!(header);
    Ok(())
  }

  pub fn load_behaviorconfig(&mut self, behavior_config: BehaviorConfigString) -> Result<()> {
    if let Some(behavior_string) = behavior_config.seek_milliseconds {
      self.behavior.seek_milliseconds = behavior_string;
    }

    if let Some(behavior_string) = behavior_config.volume_increment {
      if behavior_string > 100 {
        return Err(anyhow!(
          "Volume increment must be between 0 and 100, is {}",
          behavior_string,
        ));
      }
      self.behavior.volume_increment = behavior_string;
    }

    if let Some(tick_rate) = behavior_config.tick_rate_milliseconds {
      if tick_rate >= 1000 {
        return Err(anyhow!("Tick rate must be below 1000"));
      } else {
        self.behavior.tick_rate_milliseconds = tick_rate;
      }
    }

    if let Some(text_emphasis) = behavior_config.enable_text_emphasis {
      self.behavior.enable_text_emphasis = text_emphasis;
    }

    if let Some(loading_indicator) = behavior_config.show_loading_indicator {
      self.behavior.show_loading_indicator = loading_indicator;
    }

    Ok(())
  }

  pub fn load_visuals(&mut self, visuals: VisualsString) -> Result<()> {
    let mut index: usize = 0;
    let mut default_index: Option<usize> = None;
    let default_plugin = visuals.default.unwrap_or_else(|| "<None>".to_string());
    let plugin_path = match &self.path_to_plugin {
      Some(path) => path,
      None => {
        self.get_or_build_paths()?;
        self.path_to_plugin.as_ref().unwrap()
      }
    }
    .plugin_path
    .clone();

    macro_rules! extract_plugin {
      ($kind: ident, $enum: ident) => {
        for name in visuals.$kind.iter() {
          if name == &default_plugin {
            match default_index {
              Some(_) => return Err(anyhow!("Ambigious default plugin: \"{}\"", default_plugin)),
              None => {
                default_index = Some(index);
              }
            }
          }
          let mut plugin_file = plugin_path.clone();
          plugin_file.push(name);
          plugin_file.set_extension(PLUGIN_EXTENSION);
          let plugin_file = plugin_file;
          let warning = if plugin_file.exists() {
            None
          } else {
            Some(VisualizationError::from("Plugin cannot be found."))
          };

          let app = VisualApp {
            style: VisualStyle::$enum,
            path: plugin_file,
            name: name.to_string(),
            warning,
          };
          self.visuals.plugins.push(app);
          index += 1;
        }
      };
    };

    extract_plugin!(bar, Bar);
    extract_plugin!(chart, Chart);

    self.visuals.current = default_index;

    Ok(())
  }

  pub fn load_config(&mut self) -> Result<()> {
    let paths = match &self.path_to_config {
      Some(path) => path,
      None => {
        self.get_or_build_paths()?;
        self.path_to_config.as_ref().unwrap()
      }
    };
    if paths.config_file_path.exists() {
      let config_string = fs::read_to_string(&paths.config_file_path)?;
      // serde fails if file is empty
      if config_string.trim().is_empty() {
        return Ok(());
      }

      let config_yml: UserConfigString = serde_yaml::from_str(&config_string)?;

      if let Some(keybindings) = config_yml.keybindings.clone() {
        self.load_keybindings(keybindings)?;
      }
      if let Some(behavior) = config_yml.behavior {
        self.load_behaviorconfig(behavior)?;
      }
      if let Some(theme) = config_yml.theme {
        self.load_theme(theme)?;
      }
      if let Some(visuals) = config_yml.visuals {
        self.load_visuals(visuals)?;
      }
    }
    Ok(())
  }

  pub fn get_visualizer(&self) -> Result<VisualApp, VisualizationError> {
    match self.visuals.current {
      Some(index) => Ok(self.visuals.plugins[index].clone()),
      None => Err(VisualizationError::from("No visualizer plugin set.")),
    }
  }
  pub fn get_visualizer_or_default(&self) -> VisualApp {
    match self.get_visualizer() {
      Ok(visualizer) => visualizer,
      Err(err) => VisualApp {
        style: VisualStyle::Invalid,
        warning: Some(err),
        path: PathBuf::new(),
        name: "<None>".to_string(),
      },
    }
  }
}

fn parse_theme_item(theme_item: &str) -> Result<Color> {
  let color = match theme_item {
    "Reset" => Color::Reset,
    "Black" => Color::Black,
    "Red" => Color::Red,
    "Green" => Color::Green,
    "Yellow" => Color::Yellow,
    "Blue" => Color::Blue,
    "Magenta" => Color::Magenta,
    "Cyan" => Color::Cyan,
    "Gray" => Color::Gray,
    "DarkGray" => Color::DarkGray,
    "LightRed" => Color::LightRed,
    "LightGreen" => Color::LightGreen,
    "LightYellow" => Color::LightYellow,
    "LightBlue" => Color::LightBlue,
    "LightMagenta" => Color::LightMagenta,
    "LightCyan" => Color::LightCyan,
    "White" => Color::White,
    _ => {
      let colors = theme_item.split(',').collect::<Vec<&str>>();
      if let (Some(r), Some(g), Some(b)) = (colors.get(0), colors.get(1), colors.get(2)) {
        Color::Rgb(
          r.trim().parse::<u8>()?,
          g.trim().parse::<u8>()?,
          b.trim().parse::<u8>()?,
        )
      } else {
        println!("Unexpected color {}", theme_item);
        Color::Black
      }
    }
  };

  Ok(color)
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_parse_key() {
    use super::parse_key;
    use crate::event::Key;
    assert_eq!(parse_key(String::from("j")).unwrap(), Key::Char('j'));
    assert_eq!(parse_key(String::from("J")).unwrap(), Key::Char('J'));
    assert_eq!(parse_key(String::from("ctrl-j")).unwrap(), Key::Ctrl('j'));
    assert_eq!(parse_key(String::from("ctrl-J")).unwrap(), Key::Ctrl('J'));
    assert_eq!(parse_key(String::from("-")).unwrap(), Key::Char('-'));
    assert_eq!(parse_key(String::from("esc")).unwrap(), Key::Esc);
    assert_eq!(parse_key(String::from("del")).unwrap(), Key::Delete);
  }

  #[test]
  fn parse_theme_item_test() {
    use super::parse_theme_item;
    use tui::style::Color;
    assert_eq!(parse_theme_item("Reset").unwrap(), Color::Reset);
    assert_eq!(parse_theme_item("Black").unwrap(), Color::Black);
    assert_eq!(parse_theme_item("Red").unwrap(), Color::Red);
    assert_eq!(parse_theme_item("Green").unwrap(), Color::Green);
    assert_eq!(parse_theme_item("Yellow").unwrap(), Color::Yellow);
    assert_eq!(parse_theme_item("Blue").unwrap(), Color::Blue);
    assert_eq!(parse_theme_item("Magenta").unwrap(), Color::Magenta);
    assert_eq!(parse_theme_item("Cyan").unwrap(), Color::Cyan);
    assert_eq!(parse_theme_item("Gray").unwrap(), Color::Gray);
    assert_eq!(parse_theme_item("DarkGray").unwrap(), Color::DarkGray);
    assert_eq!(parse_theme_item("LightRed").unwrap(), Color::LightRed);
    assert_eq!(parse_theme_item("LightGreen").unwrap(), Color::LightGreen);
    assert_eq!(parse_theme_item("LightYellow").unwrap(), Color::LightYellow);
    assert_eq!(parse_theme_item("LightBlue").unwrap(), Color::LightBlue);
    assert_eq!(
      parse_theme_item("LightMagenta").unwrap(),
      Color::LightMagenta
    );
    assert_eq!(parse_theme_item("LightCyan").unwrap(), Color::LightCyan);
    assert_eq!(parse_theme_item("White").unwrap(), Color::White);
    assert_eq!(
      parse_theme_item("23, 43, 45").unwrap(),
      Color::Rgb(23, 43, 45)
    );
  }

  #[test]
  fn test_reserved_key() {
    use super::check_reserved_keys;
    use crate::event::Key;

    assert!(
      check_reserved_keys(Key::Enter).is_err(),
      "Enter key should be reserved"
    );
  }
}
