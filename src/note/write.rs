use clap::{Command, Arg, ArgMatches};
use rusqlite::Connection;
use super::db;

use std::process::Command as os_command;
use std::fs::File;
use std::io::Read;
use std::env;
use std::path::PathBuf;

pub fn subcommand() -> Command {
    Command::new("write")
        .about("write a new note")
        .alias("w")
        .arg(Arg::new("content")
            .help("Content of the note")
            .index(1))
}


pub fn execute(matches: &ArgMatches, conn: &Connection) {
    // Create a temporary file
    let mut temp_file = env::temp_dir();
    temp_file.push("temp.txt");
    File::create(&temp_file).expect("Failed to create temporary file");

    // Open the text editor
    let editor = env::var("EDITOR").unwrap_or_else(|_| String::from("vim"));
    let status = os_command::new(editor)
        .arg(&temp_file)
        .status()
        .expect("Failed to open text editor");

    if status.success() {
        // Read the content of the temporary file
        let mut file = File::open(&temp_file).expect("Failed to open temporary file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read temporary file");

        // Insert the note into the database
        db::insert_note(conn, content.trim()).expect("Failed to insert note");
        println!("Note added: {}", content.trim());
    } else {
        println!("Failed to write note");
    }

    // Delete the temporary file
    std::fs::remove_file(temp_file).expect("Failed to remove temporary file");
}
