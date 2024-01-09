use clap::{Command, Arg, ArgMatches, ArgAction};
use rusqlite::Connection;
use super::db;
use urlshortener::{providers::Provider, client::UrlShortener};

pub fn subcommand() -> Command {
    Command::new("add")
        .about("Add a new reading list item")
        .alias("a")
        .arg(Arg::new("url")
            .help("URL of the article")
            .required(true)
            .index(1))
        .arg(Arg::new("shorten")
            .short('s')
            .long("shorten")
            .action(ArgAction::SetTrue)
            .help("Shorten the URL"))
}

pub fn execute(matches: &ArgMatches, conn: &Connection) {
    if let Some(url) = matches.get_one::<String>("url") {
        let url = if matches.get_flag("shorten") || url.len() > 100 {
            url_shortener(&url)
        } else {
            url.to_string()
        };

        match db::add_reading_list_item(conn, &url) {
            Ok(()) => println!("Item added: {}", url),
            Err(e) => {
                if let rusqlite::Error::SqliteFailure(ref info, Some(ref message)) = e {
                    if info.code == rusqlite::ErrorCode::ConstraintViolation {
                        println!("Error: {}", message);
                    } else {
                        println!("Failed to add item: {}", e);
                    }
                } else {
                    println!("Failed to add item: {}", e);
                }
            },
        }
    } else {
        println!("URL is required");
    }
}

fn url_shortener(url: &str) -> String {
    let us = UrlShortener::new().unwrap();
    match us.generate(url, &Provider::IsGd) {
        Ok(short_url) => short_url,
        Err(_) => String::from("Error generating short URL"),
    }
}
