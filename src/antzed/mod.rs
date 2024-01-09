mod save;
mod build;
mod publish;
mod write;

use super::common;

use clap::{Arg, ArgAction, ArgMatches, Command};

pub mod imported_git{
    use crate::git;
    use clap::ArgMatches;

    pub fn standard(matches: &ArgMatches){
        git::interface(matches, "standard");
    }
}

pub fn subcommand() -> Command {
    Command::new("antzed")
        .about("commands for antzed.com")
        .alias("az")
        .arg(Arg::new("antzed-directory")
            .long("az-dir")
            .env("ANTZED_DIR")
            .action(ArgAction::Set)
            .help("enviromental variable of the antzed directory"))
        .subcommand(save::subcommand())
        .subcommand(build::subcommand())
        .subcommand(publish::subcommand())
        // TODO:　add write subcommand
}

pub fn execute(matches: &ArgMatches) {

    let antzed_path: &String = matches.get_one("antzed-directory")
    .expect("Require environmental variable ANTZED_DIR to have the directory to antzed's path, try put this line in .bashrc 'export ANTZED_DIR='[insert antzed directory here]'");

    let expand_antzed_path = common::expand_home_path(antzed_path);
    let antzed_path = &expand_antzed_path.to_string_lossy().to_string();


    match matches.subcommand() {
        Some(("save", save_matches)) => save::execute(save_matches, antzed_path),
        Some(("build", build_matches)) => build::execute(build_matches, antzed_path),
        Some(("publish", publish_matches)) => publish::execute(publish_matches, antzed_path),
        _ => unreachable!(),
    }
}


// mod save;
// mod build;
// mod publish;


// use std::env;
// use super::common;

// use clap::{Arg, ArgAction, ArgMatches, Command};
// use lazy_static::lazy_static;

// // Get the ANTZED_DIR environment variable
// lazy_static! {
//     static ref ANTZED_DIR: String = env::var("ANTZED_DIR").expect("Require environmental variable ANTZED_DIR to have the directory to antzed's path, try put this line in .bashrc 'export ANTZED_DIR='[insert antzed directory here]'");
//     static ref ANTZED_PATH: String = common::expand_home_path(&ANTZED_DIR).to_string_lossy().to_string();
// }

// pub mod imported_git{
//     use crate::git;
//     use clap::ArgMatches;

//     pub fn standard(matches: &ArgMatches){
//         git::interface(matches, "standard");
//     }
// }

// pub fn subcommand() -> Command {
//     Command::new("antzed")
//         .about("commands for antzed.com")
//         .alias("az")
//         .arg(Arg::new("antzed-directory")
//             .long("az-dir")
//             .env("ANTZED_DIR")
//             .action(ArgAction::Set)
//             .help("enviromental variable of the antzed directory"))
//         .subcommand(save::subcommand(&ANTZED_PATH))
//         .subcommand(build::subcommand())
//         .subcommand(publish::subcommand())
// }

// pub fn execute(matches: &ArgMatches) {
//     match matches.subcommand() {
//         Some(("save", save_matches)) => save::execute(save_matches, &ANTZED_PATH),
//         Some(("build", build_matches)) => build::execute(build_matches, &ANTZED_PATH),
//         Some(("publish", publish_matches)) => publish::execute(publish_matches, &ANTZED_PATH),
//         _ => unreachable!(),
//     }
// }