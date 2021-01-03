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

// Swaps out single character strings with Key structs.
fn parse_keymap(key: String) -> Result<Key> {
  fn get_single_char(string: &str) -> char {
    match string.chars().next() {
      Some(c) => c,
      None => panic!(),
    }
  }

  // .len() counts number of bytes, some unicode caracters concist of multiple bytes.
  // key.chars.count() still returns 1 for a single unicode caracter.
  match key.chars().count() {
    1 => Ok(Key::Char(get_single_char(key.as_str()))),
    _ => Err(anyhow!(
        "Keymap can only rebind single keys. \"{}\" has {} characters.",
        key,
        key.len()
      )),
    
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
pub struct KeymapString {
  pub q: Option<String>,
  pub w: Option<String>,
  pub e: Option<String>,
  pub r: Option<String>,
  pub t: Option<String>,
  pub y: Option<String>,
  pub u: Option<String>,
  pub i: Option<String>,
  pub o: Option<String>,
  pub p: Option<String>,
  pub a: Option<String>,
  pub s: Option<String>,
  pub d: Option<String>,
  pub f: Option<String>,
  pub g: Option<String>,
  pub h: Option<String>,
  pub j: Option<String>,
  pub k: Option<String>,
  pub l: Option<String>,
  pub z: Option<String>,
  pub x: Option<String>,
  pub c: Option<String>,
  pub v: Option<String>,
  pub b: Option<String>,
  pub n: Option<String>,
  pub m: Option<String>,
  // The compiler spits out some warnings here about snake case, but I think this is more readable than caps_q or other alternatives.
  pub shift_q: Option<String>,
  pub shift_w: Option<String>,
  pub shift_e: Option<String>,
  pub shift_r: Option<String>,
  pub shift_t: Option<String>,
  pub shift_y: Option<String>,
  pub shift_u: Option<String>,
  pub shift_i: Option<String>,
  pub shift_o: Option<String>,
  pub shift_p: Option<String>,
  pub shift_a: Option<String>,
  pub shift_s: Option<String>,
  pub shift_d: Option<String>,
  pub shift_f: Option<String>,
  pub shift_g: Option<String>,
  pub shift_h: Option<String>,
  pub shift_j: Option<String>,
  pub shift_k: Option<String>,
  pub shift_l: Option<String>,
  pub shift_z: Option<String>,
  pub shift_x: Option<String>,
  pub shift_c: Option<String>,
  pub shift_v: Option<String>,
  pub shift_b: Option<String>,
  pub shift_n: Option<String>,
  pub shift_m: Option<String>,
}

#[derive(Clone)]
pub struct Keymap {
  pub q: Key,
  pub w: Key,
  pub e: Key,
  pub r: Key,
  pub t: Key,
  pub y: Key,
  pub u: Key,
  pub i: Key,
  pub o: Key,
  pub p: Key,
  pub a: Key,
  pub s: Key,
  pub d: Key,
  pub f: Key,
  pub g: Key,
  pub h: Key,
  pub j: Key,
  pub k: Key,
  pub l: Key,
  pub z: Key,
  pub x: Key,
  pub c: Key,
  pub v: Key,
  pub b: Key,
  pub n: Key,
  pub m: Key,
  // The compiler spits out some warnings here about snake case, but I think this is more readable than caps_q or other alternatives.
  pub shift_q: Key,
  pub shift_w: Key,
  pub shift_e: Key,
  pub shift_r: Key,
  pub shift_t: Key,
  pub shift_y: Key,
  pub shift_u: Key,
  pub shift_i: Key,
  pub shift_o: Key,
  pub shift_p: Key,
  pub shift_a: Key,
  pub shift_s: Key,
  pub shift_d: Key,
  pub shift_f: Key,
  pub shift_g: Key,
  pub shift_h: Key,
  pub shift_j: Key,
  pub shift_k: Key,
  pub shift_l: Key,
  pub shift_z: Key,
  pub shift_x: Key,
  pub shift_c: Key,
  pub shift_v: Key,
  pub shift_b: Key,
  pub shift_n: Key,
  pub shift_m: Key,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BehaviorConfigString {
  pub seek_milliseconds: Option<u32>,
  pub volume_increment: Option<u8>,
  pub tick_rate_milliseconds: Option<u64>,
  pub enable_text_emphasis: Option<bool>,
  pub show_loading_indicator: Option<bool>,
  pub liked_icon: Option<String>,
  pub shuffle_icon: Option<String>,
  pub repeat_track_icon: Option<String>,
  pub repeat_context_icon: Option<String>,
  pub playing_icon: Option<String>,
  pub paused_icon: Option<String>,
}

#[derive(Clone)]
pub struct BehaviorConfig {
  pub seek_milliseconds: u32,
  pub volume_increment: u8,
  pub tick_rate_milliseconds: u64,
  pub enable_text_emphasis: bool,
  pub show_loading_indicator: bool,
  pub liked_icon: String,
  pub shuffle_icon: String,
  pub repeat_track_icon: String,
  pub repeat_context_icon: String,
  pub playing_icon: String,
  pub paused_icon: String,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserConfigString {
  keybindings: Option<KeyBindingsString>,
  keymap: Option<KeymapString>,
  behavior: Option<BehaviorConfigString>,
  theme: Option<UserTheme>,
}

#[derive(Clone)]
pub struct UserConfig {
  pub keys: KeyBindings,
  pub keymap: Keymap,
  pub theme: Theme,
  pub behavior: BehaviorConfig,
  pub path_to_config: Option<UserConfigPaths>,
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

      // default keymap
      keymap: Keymap {
        q: Key::Char('q'),
        w: Key::Char('w'),
        e: Key::Char('e'),
        r: Key::Char('r'),
        t: Key::Char('t'),
        y: Key::Char('y'),
        u: Key::Char('u'),
        i: Key::Char('i'),
        o: Key::Char('o'),
        p: Key::Char('p'),
        a: Key::Char('a'),
        s: Key::Char('s'),
        d: Key::Char('d'),
        f: Key::Char('f'),
        g: Key::Char('g'),
        h: Key::Char('h'),
        j: Key::Char('j'),
        k: Key::Char('k'),
        l: Key::Char('l'),
        z: Key::Char('z'),
        x: Key::Char('x'),
        c: Key::Char('c'),
        v: Key::Char('v'),
        b: Key::Char('b'),
        n: Key::Char('n'),
        m: Key::Char('m'),
        shift_q: Key::Char('Q'),
        shift_w: Key::Char('W'),
        shift_e: Key::Char('E'),
        shift_r: Key::Char('R'),
        shift_t: Key::Char('T'),
        shift_y: Key::Char('Y'),
        shift_u: Key::Char('U'),
        shift_i: Key::Char('I'),
        shift_o: Key::Char('O'),
        shift_p: Key::Char('P'),
        shift_a: Key::Char('A'),
        shift_s: Key::Char('S'),
        shift_d: Key::Char('D'),
        shift_f: Key::Char('F'),
        shift_g: Key::Char('G'),
        shift_h: Key::Char('H'),
        shift_j: Key::Char('J'),
        shift_k: Key::Char('K'),
        shift_l: Key::Char('L'),
        shift_z: Key::Char('Z'),
        shift_x: Key::Char('X'),
        shift_c: Key::Char('C'),
        shift_v: Key::Char('V'),
        shift_b: Key::Char('B'),
        shift_n: Key::Char('N'),
        shift_m: Key::Char('M'),
      },

      behavior: BehaviorConfig {
        seek_milliseconds: 5 * 1000,
        volume_increment: 10,
        tick_rate_milliseconds: 250,
        enable_text_emphasis: true,
        show_loading_indicator: true,
        liked_icon: " ".to_string(),
        shuffle_icon: "咽".to_string(),
        repeat_track_icon: "綾".to_string(),
        repeat_context_icon: "凌".to_string(),
        playing_icon: "契".to_string(),
        paused_icon: " ".to_string(),
      },
      path_to_config: None,
    }
  }

  pub fn get_or_build_paths(&mut self) -> Result<()> {
    match dirs::home_dir() {
      Some(home) => {
        let path = Path::new(&home);
        let home_config_dir = path.join(CONFIG_DIR);
        let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);

        if !home_config_dir.exists() {
          fs::create_dir(&home_config_dir)?;
        }

        if !app_config_dir.exists() {
          fs::create_dir(&app_config_dir)?;
        }

        let config_file_path = &app_config_dir.join(FILE_NAME);

        let paths = UserConfigPaths {
          config_file_path: config_file_path.to_path_buf(),
        };
        self.path_to_config = Some(paths);
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

  pub fn load_keymap(&mut self, keymap: KeymapString) -> Result<()> {
    macro_rules! to_keymap {
      ($name: ident) => {
        if let Some(key_string) = keymap.$name {
          self.keymap.$name = parse_keymap(key_string)?;
        }
      };
    };
    to_keymap!(q);
    to_keymap!(w);
    to_keymap!(e);
    to_keymap!(r);
    to_keymap!(t);
    to_keymap!(y);
    to_keymap!(u);
    to_keymap!(i);
    to_keymap!(o);
    to_keymap!(p);
    to_keymap!(a);
    to_keymap!(s);
    to_keymap!(d);
    to_keymap!(f);
    to_keymap!(g);
    to_keymap!(h);
    to_keymap!(j);
    to_keymap!(k);
    to_keymap!(l);
    to_keymap!(z);
    to_keymap!(x);
    to_keymap!(c);
    to_keymap!(v);
    to_keymap!(b);
    to_keymap!(n);
    to_keymap!(m);
    to_keymap!(shift_q);
    to_keymap!(shift_w);
    to_keymap!(shift_e);
    to_keymap!(shift_r);
    to_keymap!(shift_t);
    to_keymap!(shift_y);
    to_keymap!(shift_u);
    to_keymap!(shift_i);
    to_keymap!(shift_o);
    to_keymap!(shift_p);
    to_keymap!(shift_a);
    to_keymap!(shift_s);
    to_keymap!(shift_d);
    to_keymap!(shift_f);
    to_keymap!(shift_g);
    to_keymap!(shift_h);
    to_keymap!(shift_j);
    to_keymap!(shift_k);
    to_keymap!(shift_l);
    to_keymap!(shift_z);
    to_keymap!(shift_x);
    to_keymap!(shift_c);
    to_keymap!(shift_v);
    to_keymap!(shift_b);
    to_keymap!(shift_n);
    to_keymap!(shift_m);

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

    if let Some(liked_icon) = behavior_config.liked_icon {
      self.behavior.liked_icon = liked_icon;
    }

    if let Some(paused_icon) = behavior_config.paused_icon {
      self.behavior.paused_icon = paused_icon;
    }

    if let Some(playing_icon) = behavior_config.playing_icon {
      self.behavior.playing_icon = playing_icon;
    }

    if let Some(shuffle_icon) = behavior_config.shuffle_icon {
      self.behavior.shuffle_icon = shuffle_icon;
    }

    if let Some(repeat_track_icon) = behavior_config.repeat_track_icon {
      self.behavior.repeat_track_icon = repeat_track_icon;
    }

    if let Some(repeat_context_icon) = behavior_config.repeat_context_icon {
      self.behavior.repeat_context_icon = repeat_context_icon;
    }

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
      if let Some(keymap) = config_yml.keymap.clone() {
        self.load_keymap(keymap)?;
      }

      Ok(())
    } else {
      Ok(())
    }
  }

  pub fn padded_liked_icon(&self) -> String {
    format!("{} ", &self.behavior.liked_icon)
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
