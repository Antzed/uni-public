mod cmd;
mod ga;
mod git;
mod antzed;
mod common;
mod env_command;
mod note;
mod reading_list;
mod backup;

use clap::{Arg, ArgMatches, Command, ArgAction};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command as os_command;
use std::string::String;


fn main() {

    // Capture the full command line input
    let args: Vec<String> = env::args().collect();
    let cmdline = args.join(" ");


    let app = Command::new("uni")
        .version("0.1")
        .author("Antzed")
        .about("uni stands for universal, this cli aim to does everything that i need")
        .arg(Arg::new("summarize")
                .long("summarize")
                .alias("smrz")
                .action(ArgAction::SetTrue)
                .help("adding an alias of uni command you want to summarize")
                .global(true))
        .arg(Arg::new("summarize-name")
                .long("summarize-name")
                .alias("sum-name")
                .action(ArgAction::Set)
                .requires("summarize")
                .help("provide the alias name to you summarized command")
                .global(true))
        .subcommand(cmd::subcommand())
        .subcommand(ga::subcommand())
        .subcommand(git::subcommand())
        .subcommand(antzed::subcommand())
        .subcommand(env_command::subcommand())
        .subcommand(note::subcommand())
        .subcommand(reading_list::subcommand())
        .subcommand(backup::subcommand())
        .get_matches();

    match app.subcommand() {
        Some(("cmd", sub_m)) => cmd::execute(sub_m),
        Some(("ga", sub_m)) => ga::execute(sub_m),
        Some(("git", sub_m)) => git::execute(sub_m),
        Some(("antzed", sub_m)) => antzed::execute(sub_m),
        Some(("environment", sub_m)) => env_command::execute(sub_m),
        Some(("note", sub_m)) => note::execute(sub_m),
        Some(("reading-list", sub_m)) => reading_list::execute(sub_m),
        Some(("backup", sub_m)) => backup::execute(sub_m),
        _ => unreachable!(),
    }
    

    if *app.get_one::<bool>("summarize").unwrap_or(&false) {
        let alias_name: &String = app.get_one("summarize-name").expect("Required argument 'summarize-name' missing");
        let trimmed_cmdline = trim_after_summarize(&cmdline);
        let alias_command = format!("alias {}='{}'",alias_name, trimmed_cmdline);
        let mut file = OpenOptions::new()
            .append(true)
            .open("/home/anthoz/.bashrc") // Make sure to replace with the correct path
            .expect("Unable to open file");

        writeln!(file, "{}", alias_command).expect("Unable to write to file");
        
        match os_command::new("source")
        .arg("~/.bashrc")
        .status() {
            _ => {
                eprintln!("added alias");
                return;
            }
        }
    }
}


fn trim_after_summarize(cmdline: &str) -> String {
    if let Some(index) = cmdline.find("--summarize") {
        // If --summarize is found, return everything before it
        String::from(&cmdline[..index])
    } else if let Some(index) = cmdline.find("--smrz") {
        // If --smrz is found, return everything before it
        String::from(&cmdline[..index])
    } else {
        // If neither is found, return the original string
        String::from(cmdline)
    }
}