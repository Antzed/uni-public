use clap::{ArgMatches, Command};
use std::process::Command as os_command;
use super::common;

pub fn subcommand() -> Command {
    Command::new("backup")
    .alias("bu")
    .about("back up all the .db files from ~/app/uni/ to uni repo")
}

pub fn execute(matches: &ArgMatches) {
    let db_path = common::get_db_path();
    let backup_path = common::get_uni_repo_path();

    // Copy .db files to backup_path
    match os_command::new("cp")
        .arg("-r")
        .arg(db_path)
        .arg(&backup_path)
        .status() {
        Ok(status) if status.success() => println!("copied all .db files to uni repo"),
        _ => eprintln!("something went wrong, did not copy all .db files to uni repo"),
    }

    // Create a tar file of the .db files
    match os_command::new("tar")
        .arg("-cvf")
        .arg("backup.tar")
        .arg("*.db")
        .current_dir(&backup_path)
        .status() {
        Ok(status) if status.success() => println!("created tar file of .db files"),
        _ => eprintln!("something went wrong, did not create tar file of .db files"),
    }

    // Remove the .db files from backup_path
    match os_command::new("rm")
        .arg("-r")
        .arg("*.db")
        .current_dir(&backup_path)
        .status() {
        Ok(status) if status.success() => println!("removed .db files from uni repo"),
        _ => eprintln!("something went wrong, did not remove .db files from uni repo"),
    }

    // Commit the changes with a message
    match os_command::new("uni")
        .arg("g")
        .arg("s")
        .arg("-m")
        .arg("backup db")
        .current_dir(&backup_path)
        .status() {
        Ok(status) if status.success() => println!("committed the changes"),
        _ => eprintln!("something went wrong, did not commit the changes"),
    }
}