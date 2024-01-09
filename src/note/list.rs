// note/list.rs
use std::{io, thread, time::Duration, io::Write};
use std::io::stdout;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, Event, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType}
};
use ratatui::symbols::scrollbar;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, ListState, Scrollbar, Paragraph},
    Terminal,
    style::{Style, Modifier},
    text::Text,
};
use clap::{Command, ArgMatches};
use rusqlite::Connection;
use super::db;
use tokio::runtime::Runtime;

pub fn subcommand() -> Command {
    Command::new("list")
        .alias("ls")
        .about("List all notes")
}

pub fn execute(_: &ArgMatches, conn: &Connection) {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(run_tui(conn)).expect("Failed to run TUI");
}

async fn run_tui(conn: &Connection) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = ListState::default();
    state.select(Some(0));

    let notes = db::query_notes(conn).expect("Failed to query notes");

    let mut selected_note_created_at: Option<String> = None;
    let mut items: Vec<_> = notes.iter().map(|(_, content, created_at)| {
        (ListItem::new(content.lines().next().unwrap_or("").chars().take(50).collect::<String>()), created_at)
    }).collect();

    let mut selected_note: Option<String> = None;
    let mut selected_note_scroll: usize = 0;
    loop {
        terminal.draw(|f| {
            let chunks = if selected_note.is_some() {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(f.size())
            } else {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(f.size())
            };

            let list_items: Vec<ListItem> = items.iter().map(|(item, _)| item.clone()).collect();

            let list = List::new(list_items)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>");
            f.render_stateful_widget(list, chunks[0], &mut state);

            if let Some(note) = &selected_note {
                let width = chunks[1].width as usize - 2;
                let lines: Vec<_> = note
                    .lines()
                    .skip(selected_note_scroll)
                    .flat_map(|line| {
                        line.chars()
                            .collect::<Vec<_>>()
                            .chunks(width)
                            .map(|chunk| chunk.iter().collect::<String>())
                            .collect::<Vec<_>>()
                    })
                    .map(ListItem::new)
                    .collect();
                let list = List::new(lines)
                    .block(Block::default().borders(Borders::ALL).title("Note"));
                f.render_widget(list, chunks[1]);

                 // Render the created_at timestamp at the bottom of the second frame
                let timestamp_text = format!("Created at: {}", selected_note_created_at.as_ref().unwrap());
                let timestamp = Paragraph::new(Text::from(timestamp_text))
                    .block(Block::default().borders(Borders::BOTTOM));
                f.render_widget(timestamp, chunks[1]);
            }
        })?;

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Down => {
                    if selected_note.is_some() {
                        // Scroll down in the selected note
                        selected_note_scroll = selected_note_scroll.saturating_add(1);
                    } else {
                        // Move selection down in the list
                        let i = match state.selected() {
                            Some(i) if i < items.len() - 1 => i + 1,
                            _ => 0,
                        };
                        state.select(Some(i));
                    }
                }
                KeyCode::Up => {
                    if selected_note.is_some() {
                        // Scroll up in the selected note
                        selected_note_scroll = selected_note_scroll.saturating_sub(1);
                    } else {
                        // Move selection up in the list
                        let i = match state.selected() {
                            Some(i) if i > 0 => i - 1,
                            _ => items.len() - 1,
                        };
                        state.select(Some(i));
                    }
                }
                KeyCode::Enter => {
                    if let Some(index) = state.selected() {
                        let (_, content, created_at) = &notes[index];
                        selected_note = Some(content.clone());
                        selected_note_created_at = Some(created_at.clone());
                    }
                }
                KeyCode::Char('q') => {
                    selected_note = None;
                }
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}