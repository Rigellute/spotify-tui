use crate::event::Key;
use crossterm::event;
use std::{sync::mpsc, thread, time::Duration};

#[derive(Debug, Clone, Copy)]
/// Configuration for event handling.
pub struct EventConfig {
    /// The key that is used to exit the application.
    pub exit_key: Key,
    /// The tick rate at which the application will sent an tick event.
    pub tick_rate: Duration,
}

impl Default for EventConfig {
    fn default() -> EventConfig {
        EventConfig {
            exit_key: Key::Ctrl('c'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

/// An occurred event.
pub enum Event<I> {
    /// An input event occurred.
    Input(I),
    /// An tick event occurred.
    Tick,
}

/// A small event handler that wrap crossterm input and tick event. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
}

impl Events {
    /// Constructs an new instance of `Events` with the default config.
    pub fn new() -> Events {
        Events::with_config(EventConfig::default())
    }

    /// Constructs an new instance of `Events` from given config.
    pub fn with_config(config: EventConfig) -> Events {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                if event::poll(config.tick_rate).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        let key = Key::from(key);

                        if key == config.exit_key {
                            return;
                        }
                        tx.send(Event::Input(key)).unwrap();
                    }
                }

                tx.send(Event::Tick).unwrap();
            }
        });

        Events { rx }
    }

    /// Attempts to read an event.
    /// This function will block the current thread.
    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
