mod util;

use std::io::{self, Write};

use termion::cursor::Goto;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Row, SelectableList, Table, Text, Widget};
use tui::Terminal;

use util::{Event, Events};

#[derive(PartialEq)]
enum ActiveBlock {
    Input,
    Playlist,
    SongTable,
}

struct App<'a> {
    items: Vec<Vec<&'a str>>,
    input: String,
    selected_song: usize,
    selected_playlist: Option<usize>,
    active_block: ActiveBlock,
    playlists: Vec<&'a str>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            input: String::new(),
            playlists: vec!["Liked songs", "Made for you"],
            items: vec![
                vec!["Row11", "Row12", "Row13"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row31", "Row32", "Row33"],
                vec!["Row41", "Row42", "Row43"],
                vec!["Row51", "Row52", "Row53"],
                vec!["Row61", "Row62", "Row63"],
            ],
            selected_song: 0,
            selected_playlist: None,
            active_block: ActiveBlock::Playlist,
        }
    }
}

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    // App
    let mut app = App::new();

    loop {
        terminal.draw(|mut f| {
            let selected_style = Style::default().fg(Color::Cyan).modifier(Modifier::BOLD);
            let normal_style = Style::default().fg(Color::White);
            let header = ["Title", "Artist", "Album"];
            let rows = app.items.iter().enumerate().map(|(i, item)| {
                if i == app.selected_song {
                    Row::StyledData(item.iter(), selected_style)
                } else {
                    Row::StyledData(item.iter(), normal_style)
                }
            });

            let size = f.size();

            let parent_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                .margin(2)
                .split(size);

            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(parent_layout[0]);

                Paragraph::new([Text::raw(&app.input)].iter())
                    .style(Style::default().fg(Color::Yellow))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Input")
                            .border_style(
                                Style::default()
                                    .fg(get_border_color(&app.active_block, ActiveBlock::Input)),
                            ),
                    )
                    .render(&mut f, chunks[0]);
            }

            // Playlist and song block
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                    .split(parent_layout[1]);

                SelectableList::default()
                    .block(
                        Block::default()
                            .title("Playlists")
                            .borders(Borders::ALL)
                            .border_style(
                                Style::default()
                                    .fg(get_border_color(&app.active_block, ActiveBlock::Playlist)),
                            ),
                    )
                    .items(&app.playlists)
                    .select(app.selected_playlist)
                    .highlight_style(
                        Style::default()
                            .fg(Color::LightGreen)
                            .modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">")
                    .render(&mut f, chunks[0]);

                Table::new(header.iter(), rows)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Songs")
                            .border_style(
                                Style::default().fg(get_border_color(
                                    &app.active_block,
                                    ActiveBlock::SongTable,
                                )),
                            ),
                    )
                    .widths(&[10, 10, 10])
                    .render(&mut f, chunks[1]);
            }
        })?;

        // Put the cursor back inside the input box
        write!(
            terminal.backend_mut(),
            "{}",
            Goto(4 + app.input.len() as u16, 4)
        )?;
        // stdout is buffered, flush it to see the effect immediately when hitting backspace
        io::stdout().flush().ok();

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') | Key::Ctrl('c') => {
                    break;
                }
                Key::Char('/') => {
                    app.active_block = ActiveBlock::Input;
                }
                Key::Left | Key::Char('h') => match app.active_block {
                    ActiveBlock::Playlist => {} // Could wrap around and go to SongTable, like tmux?
                    ActiveBlock::SongTable => {
                        app.active_block = ActiveBlock::Playlist;
                    }
                    ActiveBlock::Input => app.input.push('h'),
                },
                Key::Right | Key::Char('l') => match app.active_block {
                    ActiveBlock::Playlist => {
                        app.active_block = ActiveBlock::SongTable;
                    }
                    ActiveBlock::SongTable => {}
                    ActiveBlock::Input => app.input.push('l'),
                },
                Key::Down | Key::Char('j') => match app.active_block {
                    ActiveBlock::Playlist => {
                        app.selected_playlist =
                            if let Some(selected_playlist) = app.selected_playlist {
                                if selected_playlist >= app.items.len() - 1 {
                                    Some(0)
                                } else {
                                    Some(selected_playlist + 1)
                                }
                            } else {
                                Some(0)
                            }
                    }
                    ActiveBlock::SongTable => {
                        app.selected_song += 1;
                        if app.selected_song > app.items.len() - 1 {
                            app.selected_song = 0;
                        }
                    }
                    ActiveBlock::Input => {
                        // NOTE: this will also type `j` if the user presses the down arrow!
                        app.input.push('j');
                    }
                },
                Key::Up | Key::Char('k') => match app.active_block {
                    ActiveBlock::Playlist => {
                        app.selected_playlist =
                            if let Some(selected_playlist) = app.selected_playlist {
                                if selected_playlist > 0 {
                                    Some(selected_playlist - 1)
                                } else {
                                    Some(app.items.len() - 1)
                                }
                            } else {
                                Some(0)
                            }
                    }
                    ActiveBlock::SongTable => {
                        if app.selected_song > 0 {
                            app.selected_song -= 1;
                        } else {
                            app.selected_song = app.items.len() - 1;
                        }
                    }
                    ActiveBlock::Input => {
                        // NOTE: this will also type `k` if the user presses the down arrow!
                        app.input.push('k')
                    }
                },
                Key::Char('\n') => {
                    if app.active_block == ActiveBlock::Input {
                        // TODO: search for tracks
                        app.active_block = ActiveBlock::SongTable;
                    }
                }
                Key::Char(c) => {
                    if app.active_block == ActiveBlock::Input {
                        app.input.push(c);
                    }
                }
                Key::Backspace => {
                    if app.active_block == ActiveBlock::Input {
                        app.input.pop();
                    }
                }
                _ => {}
            },
            _ => {}
        };
    }

    Ok(())
}

fn get_border_color(active_block: &ActiveBlock, block_to_match: ActiveBlock) -> Color {
    if *active_block == block_to_match {
        Color::Green
    } else {
        Color::White
    }
}
