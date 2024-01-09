use clap::{Arg, ArgMatches, Command, ArgAction};
use std::process::Command as os_command;

pub fn subcommand() -> Command {
    Command::new("install")
        .about("install installs a script into /usr/local/bin")
        .arg(Arg::new("new")
            .long("new") 
            .action(ArgAction::SetTrue)
            .help("creating a new file from scratch"))
        .arg(Arg::new("script")
            .long("script") // Long flag (--script)
            .short('s')
            .action(ArgAction::Set)
            .help("Path to the script"))
        .arg(Arg::new("name")
            .long("name") // Long flag (--name)
            .short('n')
            .action(ArgAction::Set)
            .help("Name to save script as in /usr/local/bin"))

}

pub fn execute(matches: &ArgMatches) {

    if matches.contains_id("new") {
        println!("--new is called, yet to be implemented")        
    } else{
        // Assuming 'script' and 'name' are command line arguments passed to your CLI
        let script: &String = matches.get_one("script").expect("Required argument 'script' missing");
        let name: &String = matches.get_one("name").expect("Required argument 'name' missing");

        // Copying the script to /usr/local/bin/
        match os_command::new("sudo")
            .arg("cp")
            .arg(script)
            .arg(format!("/usr/local/bin/{}", name))
            .status() {
            Ok(status) if status.success() => println!("copied file to user bin"),
            _ => {
                eprintln!("Failed to copy file");
                return;
            }
        }

        // Giving execution access to the file in bin
        match os_command::new("sudo")
            .arg("chmod")
            .arg("+x")
            .arg(format!("/usr/local/bin/{}", name))
            .status() {
            Ok(status) if status.success() => println!("gave execution access to file in bin"),
            _ => eprintln!("Failed to change file permissions"),
        }
    }

    
}