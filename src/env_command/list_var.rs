use super::super::common;
use clap::{Arg, ArgMatches, Command, ArgAction};
use std::process::Command as os_command;
use std::fs;
use std::error::Error;
use regex::Regex;



pub fn subcommand() -> Command {
    Command::new("list-var")
        .about("list all the environment variable in .bashrc")
        .alias("lv")
        .arg(Arg::new("with_alias")
            .long("with-alias") 
            .short('a')
            .action(ArgAction::SetTrue)
            .help("creating a new file from scratch"))
}

pub fn execute(matches: &ArgMatches) {
    let bashrc_path: String = "~/.bashrc".to_string();
    let expanded_bashrc_path = common::expand_home_path(&bashrc_path);
    // Read the contents of ~/.bashrc
    let contents = fs::read_to_string(expanded_bashrc_path).expect("failed to read .bashrc");

    // Create a regex to match lines with `export varname="var content"`
    let pattern = "^export\\s+([a-zA-Z_][a-zA-Z0-9_]*)\\s*=\\s*(\"([^\"]*)\"|'([^']*)'|(\\d+))$";
    let re = Regex::new(pattern).unwrap();

    let mut re_alias = None;
    if matches.get_flag("with_alias"){
        let alias_pattern = "^alias\\s+([a-zA-Z_][a-zA-Z0-9_]*)\\s*=\\s*(\"([^\"]*)\"|'([^']*)'|(\\d+))$";
        re_alias = Some(Regex::new(alias_pattern).unwrap());
    }

    // Iterate over each line in the file
    for line in contents.lines() {
        if let Some(caps) = re.captures(line) {
            println!("Variable: {}, Content: {}", &caps[1], &caps[2]);
        } else if let Some(caps) = re_alias.as_ref().and_then(|re| re.captures(line)) {
            println!("Alias: {}, Content: {}", &caps[1], &caps[2]);
        }
    }
    
}