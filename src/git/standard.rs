use clap::{Arg, ArgMatches, Command, ArgAction};
use std::process::Command as os_command;

use crate::main;

pub fn subcommand() -> Command {
    Command::new("standard")
        .about("standard git operation: git add . && git commit -m 'message' && git push")
        .alias("s")
        .arg(Arg::new("m")
            .long("message") 
            .short('m')
            .action(ArgAction::Set)
            .required(true)
            .help("input the message of the git commit"))
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

pub fn execute(matches: &ArgMatches) {

    let message: &String = matches.get_one("m").expect("Required argument 'm' missing");
    let branch = matches.get_one("branch").cloned().or_else(get_default_branch);
    let no_add = matches.get_flag("no-add");
    let no_push = matches.get_flag("no-push");

    if !no_add {
        match os_command::new("git")
            .arg("add")
            .arg(".")
            .status() {
                Ok(status) if status.success() => println!("git added all"),
                _ => {
                    eprintln!("Failed git add all");
                    return;
                }
            }
    }

    match os_command::new("git")
    .arg("commit")
    .arg("-m")
    .arg(message)
    .status() {
        Ok(status) if status.success() => println!("git committed with '{}'", message),
        _ => {
            eprintln!("Failed git commit with intended message: {}", message);
            return;
        }
    }

    if !no_push {
        if let Some(branch) = branch {
            match os_command::new("git")
            .arg("push")
            .arg("origin")
            .arg(&branch)
            .status() {
                Ok(status) if status.success() => println!("git pushed to {}", branch),
                _ => {
                    eprintln!("Failed git push to {}", branch);
                    return;
                }
            }
        }
    }

}

fn get_default_branch() -> Option<String> {
    let master_exists = os_command::new("git")
        .arg("rev-parse")
        .arg("--verify")
        .arg("master")
        .status()
        .map(|status| status.success())
        .unwrap_or(false);

    if master_exists {
        println!("Default branch is called master");
        Some("master".to_string())
    } else {
        println!("Default branch is called main");
        Some("main".to_string())
    }
}