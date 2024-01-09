use super::{imported_git as git};

use clap::{Arg, ArgMatches, Command, ArgAction};
use std::env;

use std::process::Command as os_command;
use chrono::Local;


pub fn subcommand() -> Command {
    Command::new("save")
    .alias("s")
    .about("Save the changes in antzed src code to repo")
    .arg(Arg::new("m")
        .long("message")
        .short('m')
        .action(ArgAction::Set)
        .help("input the save message"))
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

    let no_add = matches.get_flag("no-add");
    let no_push = matches.get_flag("no-push");

    match env::set_current_dir(antzed_path) {
        Ok(_) => {
            println!("Changed to antzed directory: {}", antzed_path);
            let file_changed_count = number_of_file_changed().expect("Failed to get the total files changes number").to_string();
            let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let message = format!("Save {} files at {}", file_changed_count, time);

            let message_static: &'static str = Box::leak(message.into_boxed_str());
            // git::standard(matches);  

            let mut command = os_command::new("uni");
            command.arg("g").arg("s").arg("-m").arg(message_static);

            // Manually check for each argument and modify the command accordingly
            if no_add {
                command.arg("--no-add");
            }
            if no_push {
                command.arg("--no-push");
            }

            let output = command.output().expect("Failed to execute command");

            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(e) => {
            eprintln!("Failed to change to antzed directory: {} (Error: {})", antzed_path, e);
            return;
        }
    }
    
}

fn number_of_file_changed() -> Result<usize, String> {
    // Execute the git command
    let output = os_command::new("git")
        .args(["status", "--porcelain"])
        .output();

    // print!("output: {:?}", output);

    match output {
        Ok(output) => {
            // Convert output to a string
            let output_str = match String::from_utf8(output.stdout) {
                Ok(s) => s,
                Err(e) => return Err(format!("Failed to parse output: {}", e)),
            };

            // Count the number of lines, each representing a changed file
            let count = output_str
                .lines()
                .filter(|line| !line.starts_with(" M Anthony-s-writing-publish"))
                .count();
            // println!("Number of files changed: {}", count);
            Ok(count)
        }
        Err(e) => Err(format!("Failed to execute command: {}", e)),
    }
}