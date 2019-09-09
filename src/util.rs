use super::app::ClientConfig;
use dirs;
use failure::err_msg;
use std::fs;
use std::io::{stdin, Write};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;

pub enum Event<I> {
    Input(I),
    Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: Key::Ctrl('c'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin_result = stdin();
                for evt in stdin_result.keys() {
                    if let Ok(key) = evt {
                        if tx.send(Event::Input(key)).is_err() {
                            return;
                        }
                        if key == config.exit_key {
                            return;
                        }
                    }
                }
            })
        };
        let tick_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let tx = tx.clone();
                loop {
                    tx.send(Event::Tick).unwrap();
                    thread::sleep(config.tick_rate);
                }
            })
        };
        Events {
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}

pub fn get_config() -> Result<ClientConfig, failure::Error> {
    match dirs::home_dir() {
        Some(home) => {
            let path = Path::new(&home);
            let file_name = "client.yml";
            let config_path = path.join(".config/spotify-tui");

            if config_path.exists() {
                let config_string = fs::read_to_string(config_path.join(file_name))?;
                let config_yml: ClientConfig = serde_yaml::from_str(&config_string)?;

                Ok(config_yml)
            } else {
                println!("Config does not exist, creating it");
                fs::create_dir(&config_path)?;
                let mut new_config = fs::File::create(&config_path.join(file_name))?;
                let content = ClientConfig {
                    client_id: "abddfslkjsj1234".to_string(),
                    client_secret: "abddfslkjsj1234".to_string(),
                };

                let content_yml = serde_yaml::to_string(&content)?;
                write!(new_config, "{}", content_yml)?;
                Err(err_msg(format!(
                    "Add your spotify client_id and client_secret to {}",
                    config_path.display()
                )))
            }
        }
        None => Err(err_msg("No $HOME directory found for client config")),
    }
}
