mod list_var;

use clap::{ArgMatches, Command};

pub fn subcommand() -> Command {
    Command::new("environment")
        .about("anything related to environment setup")
        .alias("env")
        .subcommand(list_var::subcommand())
}

pub fn execute(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("list-var", list_var_matches)) => list_var::execute(list_var_matches),
        _ => unreachable!(),
    }
}
