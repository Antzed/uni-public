use clap::{Arg, ArgMatches, Command};
use std::process::Command as os_command;

pub fn subcommand() -> Command {
    Command::new("ls")
        .about("ls sub-subcommand")

}

pub fn execute(matches: &ArgMatches) {
     println!("listing all the command:\n" );
     let mut list_dir = os_command::new("ls");
     list_dir.arg("/usr/local/bin/");

     match list_dir.output() {
         Ok(output) => {
             // Convert the output to a string and print it
             let output_str = String::from_utf8_lossy(&output.stdout);
             println!("{}", output_str);
         },
         Err(e) => {
             // Handle the error here
             println!("Error executing command: {}", e);
         }
     }
}
