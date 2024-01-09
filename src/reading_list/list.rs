// use clap::{Command, Arg, ArgMatches};
// use rusqlite::Connection;
// use super::db;

// pub fn subcommand() -> Command {
//     Command::new("list")
//         .about("List all reading list items")
//         .alias("ls")
// }

// pub fn execute(_: &ArgMatches, conn: &Connection) {
//     let items = db::list_reading_list(conn).expect("Failed to query items");

//     println!("Reading List\n============\n");

//     println!("Read Articles\n-------------");
//     for (id, url, status, added_at, read_at) in &items {
//         if status == "read" {
//             println!("{}, URL: {}", id, url);
//             println!("Added at: {}, Read at: {:?}", added_at, read_at.as_ref().unwrap());
//             println!("-------------");
//         }
//     }

//     println!("\nUnread Articles\n---------------");
//     for (id, url, status, added_at, read_at) in &items {
//         if status == "unread" {
//             println!("{}, URL: {}", id, url);
//             println!("Added at: {}", added_at);
//             println!("----------------");
//         }
//     }    
// }

use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers, Event},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
    style::{Style, Modifier},
    text::{Span, Text},
};
use chrono::Local;
use clap::{Command, ArgMatches};
use rusqlite::Connection;
use super::db;
use std::io;

pub fn subcommand() -> Command {
    Command::new("list")
        .about("List all reading list items")
        .alias("ls")
}

pub fn execute(_: &ArgMatches, conn: &Connection) { 
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(run_tui(conn)).expect("Failed to run TUI");
}

async fn run_tui(conn: &Connection) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let items = db::list_reading_list(conn).expect("Failed to query items");
    let mut unread_items: Vec<_> = items.iter().filter(|(_, _, status, _, _)| *status == "unread").cloned().collect();
    let mut read_items: Vec<_> = items.iter().filter(|(_, _, status, _, _)| *status == "read").cloned().collect();

    let mut unread_state = ListState::default();
    unread_state.select(Some(0));
    let mut read_state = ListState::default();
    read_state.select(Some(0));

    let mut active_frame = 0;

    let mut last_key = ' '; 

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let max_width = chunks[0].width as usize - 10;


            let unread_list_items: Vec<ListItem> = unread_items
                .iter()
                .map(|(id, url, _, added_at, _)| {
                    let text = format!("ID: {}, {}, Added at: {}", id, url, added_at);
                    let wrapped_text = wrap_text(&text, max_width); // Replace 100 with your desired max width
                    ListItem::new(Text::from(wrapped_text.join("\n")))
                })
                .collect();
            let read_list_items: Vec<ListItem> = read_items
                .iter()
                .map(|(id, url, _, added_at, read_at)| {
                    let text = format!("ID: {}, {}, Added at: {}, Read at: {:?}", id, url, added_at, read_at.as_ref().unwrap());
                    let wrapped_text = wrap_text(&text, max_width); // Replace 100 with your desired max width
                    ListItem::new(Text::from(wrapped_text.join("\n")))
                })
                .collect();

            let unread_list = List::new(unread_list_items)
                .block(Block::default().borders(Borders::ALL).title("Unread Articles"))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");

            let read_list = List::new(read_list_items)
                .block(Block::default().borders(Borders::ALL).title("Read Articles"))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");

            f.render_stateful_widget(unread_list, chunks[0], &mut unread_state);
            f.render_stateful_widget(read_list, chunks[1], &mut read_state);
        })?;

        // Update the key handling code
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
            match code {
                KeyCode::Down => {
                    if active_frame == 0 {
                        if let Some(selected) = unread_state.selected() {
                            let amount = unread_items.len();
                            if selected >= amount - 1 {
                                unread_state.select(Some(0));
                            } else {
                                unread_state.select(Some(selected + 1));
                            }
                        }
                    } else if active_frame == 1 {
                        if let Some(selected) = read_state.selected() {
                            let amount = read_items.len();
                            if selected >= amount - 1 {
                                read_state.select(Some(0));
                            } else {
                                read_state.select(Some(selected + 1));
                            }
                        }
                    }
                }
                KeyCode::Up => {
                    if active_frame == 0 {
                        if let Some(selected) = unread_state.selected() {
                            let amount = unread_items.len();
                            if selected > 0 {
                                unread_state.select(Some(selected - 1));
                            } else {
                                unread_state.select(Some(amount - 1));
                            }
                        }
                    } else if active_frame == 1 {
                        if let Some(selected) = read_state.selected() {
                            let amount = read_items.len();
                            if selected > 0 {
                                read_state.select(Some(selected - 1));
                            } else {
                                read_state.select(Some(amount - 1));
                            }
                        }
                    }
                }
                KeyCode::Left => {
                    active_frame = 0;
                }
                KeyCode::Right => {
                    active_frame = 1;
                }
                KeyCode::Char('m') => {
                    last_key = 'm'; // Update the last key when 'm' is pressed
                }
                KeyCode::Char('r') => {
                    if last_key == 'm' {
                        match active_frame {
                            0 => { // Moving from unread to read
                                if let Some(selected) = unread_state.selected() {
                                    let mut item = unread_items.remove(selected);
                                    item.2 = "read".to_string(); // Update status to 'read'
                                    item.4 = Some(Local::now().naive_local().to_string()); // Update read_at timestamp
                                    read_items.push(item);

                                    // Update database
                                    let (id, _, _, _, _) = &read_items.last().unwrap();
                                    db::mark_as_read(conn, id.clone()).expect("Failed to mark item as read");

                                    // Update list state
                                    update_list_state(&mut unread_state, &unread_items, selected);
                                    read_state.select(Some(read_items.len() - 1));
                                }
                            },
                            1 => { // Moving from read to unread
                                if let Some(selected) = read_state.selected() {
                                    let mut item = read_items.remove(selected);
                                    item.2 = "unread".to_string(); // Update status to 'unread'
                                    item.4 = None; // Clear read_at timestamp
                                    unread_items.push(item);

                                    // Update database
                                    let (id, _, _, _, _) = &unread_items.last().unwrap();
                                    db::mark_as_unread(conn, id.clone()).expect("Failed to mark item as unread");

                                    // Update list state
                                    update_list_state(&mut read_state, &read_items, selected);
                                    unread_state.select(Some(unread_items.len() - 1));
                                }
                            },
                            _ => {
                                last_key = ' '; 
                            }
                        }
                    }
                }
                KeyCode::Char('d') => {
                    last_key = 'd'; // Update the last key when 'd' is pressed
                }
                KeyCode::Char('e') => {
                    if last_key == 'd' {
                        match active_frame {
                            0 => { // Deleting from unread
                                if let Some(selected) = unread_state.selected() {
                                    let (id, _, _, _, _) = unread_items.remove(selected);
                                    db::delete_reading_list_item(conn, id.clone()).expect("Failed to delete item");
    
                                    // Update list state
                                    update_list_state(&mut unread_state, &unread_items, selected);
                                }
                            },
                            1 => { // Deleting from read
                                if let Some(selected) = read_state.selected() {
                                    let (id, _, _, _, _) = read_items.remove(selected);
                                    db::delete_reading_list_item(conn, id.clone()).expect("Failed to delete item");
                                    // Update list state
                                    update_list_state(&mut read_state, &read_items, selected);
                                }
                            },
                            _ => {
                                last_key = ' '; 
                            }
                        }
                    }
                }
                KeyCode::Esc => {
                    break;
                }
                _ => {
                    last_key = ' '; // Reset last_key for any other key press
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}


type ItemType = (i32, String, String, String, Option<String>);

fn update_list_state(state: &mut ListState, items: &[ItemType], selected: usize) {
    if items.is_empty() {
        state.select(None);
    } else if selected >= items.len() {
        state.select(Some(items.len() - 1));
    } else {
        state.select(Some(selected));
    }
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let comma_lines = text.split(",").collect::<Vec<&str>>();
    let mut lines = Vec::new();

    for line in comma_lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut line = String::new();

        for part in parts {
            if part.len() > max_width {
                let chunks: Vec<String> = part.chars().collect::<Vec<char>>().chunks(max_width).map(|chunk| chunk.iter().collect::<String>()).collect();
                for chunk in chunks {
                    if !line.is_empty() {
                        lines.push(line.clone());
                        line.clear();
                    }
                    lines.push(chunk);
                }
            } else if line.len() + part.len() > max_width {
                if !line.is_empty() {
                    lines.push(line.clone());
                }
                line = String::from(part);
            } else {
                if !line.is_empty() {
                    line.push(' ');
                }
                line.push_str(part);
            }
        }

        if !line.is_empty() {
            lines.push(line);
        }
    }

    lines
}