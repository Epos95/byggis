use clap::{App, Arg};
use crossterm::style::*;

mod creator;
mod describer;
mod runner;
pub mod supported_languages;

const VERSION: &str = "0.3.9";
const AUTHOR: &str = "Epos95";

#[tokio::main]
async fn main() {
    let mut app = App::new("Byggis")
        .version(VERSION)
        .author(AUTHOR)
        .about("A build and testing system for kattis problems")
        .subcommand(App::new("run")
            .about("Runs tests for the selected problem")
            .version(VERSION)
            .author(AUTHOR)
            .arg(Arg::new("ignore time")
                .about("Stops byggis from warning the user about timeouts on successfull tests")
                .takes_value(false)
                .required(false)
                .long("ignore-time")
                .short('t')))
        .subcommand(App::new("new")
            .about("Downloads and creates a new directory for a given kattis problem")
            .version(VERSION)
            .author(AUTHOR)
            .arg(Arg::new("filename")
                .takes_value(true)
                .required(true)
                .value_name("FILE")))
        .subcommand(App::new("describe")
            .about("Prints the description for a kattis problem (BETA)")
            .version(VERSION)
            .author(AUTHOR));
    let matches = app.clone().get_matches();

    if matches.subcommand_matches("run").is_some() {
        let test_time = matches
            .subcommand_matches("run")
            .unwrap()
            .is_present("ignore time");

        let r = runner::run_tests(test_time);
        match r {
            Ok(_) => {
                println!("   Tests completed.");
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    } else if matches.subcommand_matches("new").is_some() {
        let filename: String = matches
            .subcommand_matches("new")
            .unwrap()
            .value_of("filename")
            .unwrap()
            .to_string();

        let r = creator::create_new(filename).await;

        match r {
            Ok(n) => {
                println!(
                    "   {} new byggis folder \"{}\"",
                    "Created".green(),
                    n.bold()
                );
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    } else if matches.subcommand_matches("describe").is_some() {
        let r = describer::describe();
        if let Err(x) = r {
            println!("{}", x);
        }
    } else {
        app.print_help().unwrap();
    }
}
