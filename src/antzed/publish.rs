use super::imported_git as git;


use clap::{Arg, ArgMatches, Command, ArgAction};
use std::env;

pub fn subcommand() -> Command {
    Command::new("publish")
        .alias("p")
        .about("Publish antzed's website to netlify")
        .arg(Arg::new("m")
            .long("message")
            .short('m')
            .required(true)
            .action(ArgAction::Set)
            .help("input the commit message"))
        .arg(Arg::new("branch")
            .long("branch")
            .short('b')
            .action(ArgAction::Set)
            .help("input the branch this standard operation is pushing to"))
        .arg(Arg::new("no-add")
            .long("no-add")
            .alias("na")
            .action(ArgAction::SetTrue)
            .help("skip the git add operation"))
        .arg(Arg::new("no-push")
            .long("no-push")
            .alias("np")
            .action(ArgAction::SetTrue)
            .help("skip the git push operation"))
}


pub fn execute(matches: &ArgMatches, antzed_path: &String ) {
    let publish_path = format!("{}{}", antzed_path, "Anthony-s-writing-publish/");

    match env::set_current_dir(&publish_path) {
        Ok(_) => {
            println!("Changed to antzed publish directory: {}", publish_path);
            git::standard(matches);  
        }
        Err(e) => {
            eprintln!("Failed to change to antzed publish directory: {} (Error: {})", publish_path, e);
            return;
        }
    }
    
}