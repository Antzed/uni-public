pub mod add;
pub mod list;
pub mod mark_read;
pub mod db;

use clap::{Command, ArgMatches};
use rusqlite::Connection;
use super::common;


pub fn subcommand() -> Command {
    Command::new("reading-list")
        .about("Manage notes")
        .alias("rl")
        .subcommand(add::subcommand())
        .subcommand(list::subcommand())
        .subcommand(mark_read::subcommand())
}

pub fn execute(matches: &ArgMatches) {
    let mut db_dir = common::get_db_path();
    db_dir.push("my_reading_list.db");

    let conn = Connection::open(&db_dir).expect("Failed to open database");
    db::setup_reading_list(&conn).expect("Failed to set up database");

    match matches.subcommand() {
        Some(("add", sub_matches)) => add::execute(sub_matches, &conn),
        Some(("list", sub_matches)) => list::execute(sub_matches, &conn),
        Some(("mark-read", sub_matches))=> mark_read::execute(sub_matches, &conn),
        _ => unreachable!(),
    }
}