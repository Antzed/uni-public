use clap::{ArgMatches, Command};
use std::process::Command as os_command;


pub fn subcommand() -> Command {
    Command::new("ga")
        .about("git add")
}

pub fn execute(matches: &ArgMatches) {
    match os_command::new("git")
            .arg("add")
            .arg(".")
            .status() {
            Ok(status) if status.success() => println!("git added all"),
            _ => eprintln!("something went wrong, git did not add all"),
    }

}