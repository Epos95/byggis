use byggis;
use std::env;
use byggis::ByggisErrors;
use crossterm::style::*;

mod creator;
mod runner;
mod helper;
mod submitter;

#[tokio::main]
async fn main() {
    // TODO: port this to clap
    // TODO: next level errror handling, we talking inheritance, structs etc
    // TODO: port all this to crossterm
    // TODO: absorb the structure of ica (all modules sharing errors and handling them all is actually a bit bad.)


    let mut args: Vec<String> = env::args().rev().collect();

    // get rid of file name
    args.pop();

    // pop the rightmost element in args, if there is none (e.g args.len == 0)
    // it will default to "help" which will show the help command
    // lowercases input for normalization purposes
    let command: String = args.pop()
        .unwrap_or("help".to_string())
        .to_lowercase();


    match command.as_str() {
        "run" => {
            // maybe accept name of file to run as input
            // default to main.something

            // TODO: maybe solve multiple main files
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
        },
        "new" => {
            if args.len() > 0 {
                let r = creator::create_new(args.pop().unwrap()).await;

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
            } else {
                helper::show_help(helper::HelpTypes::New);
            }

        },
        "commit" => {
            // run commit code here
            // should commit have any args?
            // dont think so

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
        _ => {
            helper::show_help(helper::HelpTypes::Program);
        }
    }
}
