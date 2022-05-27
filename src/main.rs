use clap::{Command, command, arg};
use crossterm::style::*;

mod creator;
mod describer;
mod runner;
pub mod supported_languages;

#[tokio::main]
async fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("run")
                .about("Runs tests for the selected problem")
                .arg(
                    arg!(-t --timeout "Whether or not to ignore timeout when running testes")
                )
        )
        .subcommand(
            Command::new("new")
                .about("Downloads and creates a new directory for a given kattis problem")
                .arg(
                    arg!(<FILE>)
                )
        )
        .subcommand(
            Command::new("describe")
                .about("Prints the description for a kattis problem")
        )
        .get_matches();

    if matches.subcommand_matches("run").is_some() {
        let test_time = matches
            .subcommand_matches("run")
            .unwrap()
            .is_present("timeout");

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
            .value_of("FILE")
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
    } 
}
