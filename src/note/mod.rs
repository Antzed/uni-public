// note/mod.rs

pub mod write;
pub mod list;
pub mod db;

use super::common;
use clap::{Command, ArgMatches};
use rusqlite::Connection;

pub fn subcommand() -> Command {
    Command::new("note")
        .about("Manage notes")
        .alias("n")
        .subcommand(write::subcommand())
        .subcommand(list::subcommand())
}

pub fn execute(matches: &ArgMatches) {

    let mut db_path = common::get_db_path();
    db_path.push("my_notes.db");
    let conn = Connection::open(&db_path).expect("Failed to open database");
    db::setup_database(&conn).expect("Failed to set up database");

    match matches.subcommand() {
        Some(("write", sub_matches)) => write::execute(sub_matches, &conn),
        Some(("list", sub_matches)) => list::execute(sub_matches, &conn),
        _ => unreachable!(),
    }
}
