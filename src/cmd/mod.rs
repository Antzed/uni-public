mod ls;
mod install;

use clap::{ArgMatches, Command};

pub fn subcommand() -> Command {
    Command::new("cmd")
        .about("cmd are in charge of everything command wise, such as all the executables in /usr/local/bin/")
        .alias("command")
        .subcommand(ls::subcommand())
        .subcommand(install::subcommand())
}

pub fn execute(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("ls", ls_matches)) => ls::execute(ls_matches),
        Some(("install", install_matches)) => install::execute(install_matches),
        _ => unreachable!(),
    }
}
