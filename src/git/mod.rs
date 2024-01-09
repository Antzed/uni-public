mod standard;
mod link;

use clap::{ArgMatches, Command, Arg};

pub fn subcommand() -> Command {
    Command::new("git")
        .about("automate complex git operations")
        .alias("g")
        .subcommand(standard::subcommand())
        .subcommand(link::subcommand())
}

pub fn execute(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("standard", standard_matches)) => standard::execute(standard_matches),
        Some(("link", link_matches)) => link::execute(link_matches),
        _ => unreachable!(),
    }
}

pub fn interface(matches: &ArgMatches, name: &str){
    match name {
        "standard"=>standard::execute(matches),
        _ => {
            // Handle other cases or provide an error message
            println!("Unsupported command: {}", name);
        }
    }
}
