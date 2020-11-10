use std::{io::stdin, sync::mpsc, thread, time::Duration};
use termion::{event::Key, input::TermRead};

pub enum Event<I> {
  Input(I),
  Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
  rx: mpsc::Receiver<Event<Key>>,
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
    let _input_handle = {
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

    let _tick_handle = {
      let tx = tx;
      thread::spawn(move || {
        let tx = tx.clone();
        loop {
          tx.send(Event::Tick).unwrap();
          thread::sleep(config.tick_rate);
        }
      })
    };

    Events { rx }
  }

  pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
    self.rx.recv()
  }
}
