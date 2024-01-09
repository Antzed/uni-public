use clap::{Arg, ArgMatches, Command, ArgAction};
use std::env;

use std::process::Command as os_command;

pub fn subcommand() -> Command {
    Command::new("build")
        .alias("b")
        .about("build antzed's mdbook")
        .arg(Arg::new("open")
            .long("open")
            .short('o')
            .action(ArgAction::SetTrue)
            .help("open the build in browser"))
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
    let build_path = format!("{}{}", antzed_path, "Anthony-s-writing-publish/book/");

    match env::set_current_dir(&antzed_path) {
        Ok(_) => {
            println!("Changed to antzed build directory: {}", antzed_path);

            match os_command::new("./mdbook-summary-maker")
            .status() {
                Ok(status) if status.success() => println!("runned mdbook-summary-maker"),
                _ => {
                    eprintln!("Failed to run mdbook-summary-maker");
                    return;
                }
            }

            if matches.get_flag("open"){
                match os_command::new("mdbook")
                .arg("build")
                .arg("--open")
                .arg("--dest-dir")
                .arg(build_path)
                .status() {
                    Ok(status) if status.success() => println!("build mdbook and opened in browser"),
                    _ => {
                        eprintln!("Failed to build mdbook and opened in browser");
                        return;
                    }
                }
            } else {
                match os_command::new("mdbook")
                .arg("build")
                .arg("--dest-dir")
                .arg(build_path)
                .status() {
                    Ok(status) if status.success() => println!("build mdbook"),
                    _ => {
                        eprintln!("Failed to build mdbook");
                        return;
                    }
                }

            }
        }
        Err(e) => {
            eprintln!("Failed to change to antzed build directory: {} (Error: {})", antzed_path, e);
            return;
        }
    }
    
}

