use byggis;
use byggis::ByggisErrors;
use crossterm::style::*;
use clap::{
    App, 
    Arg,
};

mod creator;
mod runner;
mod submitter;

const VERSION: &str = "0.3.3";
const AUTHOR:  &str = "Epos95";

#[tokio::main]
async fn main() {
    // TODO: Implement submissions
    // TODO: Let byggis show the problen description in the terminal
    // TODO: Write tests for a basic workflow
    // TODO: Add comments where they are needed
    // TODO: Show the output byggis expected on error
    // TODO: Report error if simulation runs too long
    // TODO: program output testing doesnt work on windows!

    let matches = App::new("Byggis")
        .version(VERSION)
        .author(AUTHOR)
        .about("A build and testing system for kattis problems")
        .subcommand(App::new("run")
            .about("Runs tests for the selected problem")
            .version(VERSION)
            .author(AUTHOR))
        .subcommand(App::new("new")
            .about("Downloads and creates a new directory for a given kattis problem")
            .version(VERSION)
            .author(AUTHOR)
            .arg(Arg::new("filename")
                .takes_value(true)
                .required(true)
                .value_name("FILE")))
        .subcommand(App::new("commit")
            .about("Submits the selected file to kattis (not done)")
            .version(VERSION)
            .author(AUTHOR)
            .arg(Arg::new("filename to submit")
                .takes_value(true)
                .required(false)
                .value_name("FILE")))
        .get_matches();

    if matches.is_present("run") {

        let r = runner::run_tests();
        match r {
            Ok(_) => {
                println!("   Tests completed.");
            },
            Err(ByggisErrors::ByggisFileNotFound) => {
                println!("   {}: Could not find byggis file in folder",
                    "Error".red());
                println!("    Did you run \"byggis new 'name'\"?");
            },
            Err(ByggisErrors::TestsNotFound) => {
                println!("   {}: Could not find tests in byggis file",
                    "Error".red());
            },
            Err(ByggisErrors::MainNotFound) => {
                println!("   {}: Could not find a main file to test with",
                    "Error".red());
            },
            Err(ByggisErrors::UnknownLanguage) => {
                println!("   {}: Language not implemented",
                    "Error".red());
            },
            Err(ByggisErrors::CompileTimeError(e)) => {
                println!("     Compilation error:");

                for line in e.trim().split("\n") {
                    println!("      {}", line.bold());
                }
                println!("");
            },
            _ => {}
        }
    } else if matches.is_present("new") {
        let filename: String = if let Some(ref m) = matches.subcommand_matches("new") {
            m.value_of("filename").unwrap().to_string()
        } else {
            println!("   {}: Somehow failed...", "Error".red());
            panic!();
        };

        let r = creator::create_new(filename).await;

        match r {
            Ok(n) => {
                println!("   {} new byggis folder \"{}\"",
                    "Created".green(),
                    n.bold());
            },
            Err(ByggisErrors::NetworkError) => {
                println!("   {}: Could not connect to open.kattis.com",
                    "Error".red());
            },
            Err(ByggisErrors::ProblemNotFound) => {
                println!("   {}: Could not find that problem on kattis",
                    "Error".red());
            },
            Err(ByggisErrors::DirectoryNotCreated) => {
                println!("   {}: Director could not be created",
                    "Error".red());
            },
            Err(ByggisErrors::ByggisFileNotCreated) => {
                println!("   {}: byggis file could not be created",
                    "Error".red());
            },
            _ => {}
        }
    }  else if matches.is_present("commit") {
        panic!("Not implemented yet");
        #[allow(unreachable_code)] {
            let r = submitter::commit().await;

            match r {
                // we dont need to return anything in the ok
                Ok(_) => {
                    println!("Success!")
                },
                Err(ByggisErrors::NetworkError) => {
                    println!("   {}: Could not connect to open.kattis.com",
                        "Error".red());
                },
                Err(ByggisErrors::MainNotFound) => {
                    println!("   {}: Could not find a main file to test with",
                        "Error".red());
                },
                Err(ByggisErrors::ConfigFileNotFound) => {
                    println!("   {}: Could not find config file containing token\n    You can generate one with \"{}\"",
                        "Error".red(),
                        "Byggis generate".bold());
                },
                Err(ByggisErrors::InvalidToken) => {
                    println!("   {}: Invalid token",
                        "Error".red());
                },
                _ => {
                    panic!("Unimplemented error");
                }
            }
        }
    }
}
