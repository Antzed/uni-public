use clap::{Command, Arg, ArgMatches};
use rusqlite::Connection;
use super::db;

pub fn subcommand() -> Command {
    Command::new("mark-read")
        .about("Mark a reading list item as read")
        .alias("mr")
        .arg(Arg::new("id")
            .help("ID of the article to mark as read")
            .required(true)
            .index(1))
}

pub fn execute(matches: &ArgMatches, conn: &Connection) {
    if let Some(id_str) = matches.get_one::<String>("id") {
        if let Ok(id) = id_str.parse::<i32>() {
            db::mark_as_read(conn, id).expect("Failed to mark item as read");
            println!("Item marked as read: {}", id);
        } else {
            println!("Invalid ID");
        }
    } else {
        println!("ID is required");
    }
}
