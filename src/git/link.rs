use clap::{Arg, ArgMatches, Command, ArgAction};
use std::process::{Command as OsCommand};
use webbrowser;

pub fn subcommand() -> Command {
    Command::new("link")
        .about("Get links to your repository")
        .alias("l")
        .arg(Arg::new("open")
                .long("open")
                .short('o')
                .action(ArgAction::SetTrue)
                .help("Open the git remote -v link in browser"))
}

pub fn execute(matches: &ArgMatches) {
    if matches.get_flag("open") {
        let output = match OsCommand::new("git")
            .arg("remote")
            .arg("-v")
            .output() {
                Ok(output) => output,
                Err(_) => {
                    eprintln!("Failed to get git remote links");
                    return;
                }
            };

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let first_line = stdout.lines().next().unwrap_or_default();
            let url = first_line.split_whitespace().nth(1).unwrap_or_default();

            if webbrowser::open(url).is_ok() {
                println!("Opened URL: {}", url);
            } else {
                eprintln!("Failed to open URL: {}", url);
            }
        } else {
            eprintln!("Failed to get git remote links");
        }
    } else {
        match OsCommand::new("git")
            .arg("remote")
            .arg("-v")
            .status() {
                Ok(status) if status.success() => println!("Got git remote links"),
                _ => {
                    eprintln!("Failed to get git remote links");
                    return;
                }
            }
    }
}
