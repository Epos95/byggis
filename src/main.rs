use byggis;
use std::env;
use byggis::ByggisErrors;
use termion::*;

mod creator;
mod runner;
mod helper;
mod submitter;

#[tokio::main]
async fn main() {
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
                    println!("   {}Error{}: Could not find byggis file in folder",
                        color::Fg(color::Red),
                        color::Fg(color::Reset));
                    println!("    Did you run \"byggis new 'name'\"?");
                },
                Err(ByggisErrors::TestsNotFound) => {
                    println!("   {}Error{}: Could not find tests in byggis file",
                        color::Fg(color::Red),
                        color::Fg(color::Reset));
                },
                Err(ByggisErrors::MainNotFound) => {
                    println!("   {}Error{}: Could not find a main file to test with",
                        color::Fg(color::Red),
                        color::Fg(color::Reset));
                },
                Err(ByggisErrors::UnknownLanguage) => {
                    println!("   {}Error{}: Language not implemented",
                        color::Fg(color::Red),
                        color::Fg(color::Reset));
                },
                Err(ByggisErrors::CompileTimeError(e)) => {
                    println!("     Compilation error:");

                    for line in e.trim().split("\n") {
                        println!("      {}{}{}", style::Bold, line, style::Reset);
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
                        println!("   {}Created{} new byggis folder {}\"{}\"{}",
                            color::Fg(color::Green),
                            color::Fg(color::Reset),
                            style::Bold,
                            n,
                            style::Reset);
                    },
                    Err(ByggisErrors::NetworkError) => {
                        println!("   {}Error{}: Could not connect to open.kattis.com",
                            color::Fg(color::Red),
                            color::Fg(color::Reset));
                    },
                    Err(ByggisErrors::ProblemNotFound) => {
                        println!("   {}Error{}: Could not find that problem on kattis",
                            color::Fg(color::Red),
                            color::Fg(color::Reset));
                    },
                    Err(ByggisErrors::DirectoryNotCreated) => {
                        println!("   {}Error{}: Director could not be created",
                            color::Fg(color::Red),
                            color::Fg(color::Reset));
                    },
                    Err(ByggisErrors::ByggisFileNotCreated) => {
                        println!("   {}Error{}: byggis file could not be created",
                            color::Fg(color::Red),
                            color::Fg(color::Reset));
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
                    println!("   {}Error{}: Could not connect to open.kattis.com",
                        color::Fg(color::Red),
                        color::Fg(color::Reset));
                },
                Err(ByggisErrors::MainNotFound) => {
                    println!("   {}Error{}: Could not find a main file to test with",
                        color::Fg(color::Red),
                        color::Fg(color::Reset));
                },
                Err(ByggisErrors::ConfigFileNotFound) => {
                    println!("   {}Error{}: Could not find config file containing token\n    You can generate one with {}\"byggis generate\"{}",
                        color::Fg(color::Red),
                        color::Fg(color::Reset),
                        style::Bold,
                        style::Reset);
                },
                Err(ByggisErrors::InvalidToken) => {
                    println!("   {}Error{}: Invalid token",
                        color::Fg(color::Red),
                        color::Fg(color::Reset));
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
