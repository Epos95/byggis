use crossterm::style::*;
use clap::{
    App, 
    Arg,
};


mod creator;
mod runner;
mod submitter;
mod describer;


const VERSION: &str = "0.3.7";
const AUTHOR:  &str = "Epos95";


#[tokio::main]
async fn main() {
    // TODO: Implement submissions
    // TODO: Finish describer.rs
    // TODO: Write tests for a basic workflow
    // TODO: Comment more things
    // TODO: Touch up comments in lib.rs
    // TODO: Add options "run" that lets you specifiy alloted time
    // TODO: solve creator bug when theres an uneven amount of pre tags in the problem body
    //       do this by looking for sample tags instead and iterating over it smarterly


    let matches = App::new("Byggis")
        .version(VERSION)
        .author(AUTHOR)
        .about("A build and testing system for kattis problems")
        .subcommand(App::new("run")
            .about("Runs tests for the selected problem")
            .version(VERSION)
            .author(AUTHOR)
            .arg(Arg::new("ignore time")
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
        .subcommand(App::new("commit")
            .about("Submits the selected file to kattis (TODO)")
            .version(VERSION)
            .author(AUTHOR)
            .arg(Arg::new("filename to submit")
                .takes_value(true)
                .required(false)
                .value_name("FILE")))
        .subcommand(App::new("describe")
            .about("Prints the description for a kattis problem")
            .version(VERSION)
            .author(AUTHOR))
        .get_matches();

    if matches.subcommand_matches("run").is_some() {

        let test_time = matches.subcommand_matches("run").unwrap().is_present("ignore time");

        let r = runner::run_tests(test_time);
        match r {
            Ok(_) => {
                println!("   Tests completed.");
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    } else if matches.subcommand_matches("new").is_some() {
        let filename = matches
            .subcommand_matches("new")
            .unwrap()
            .value_of("filename")
            .unwrap()
            .to_string();

        let r = creator::create_new(filename).await;

        match r {
            Ok(n) => {
                println!("   {} new byggis folder \"{}\"",
                    "Created".green(),
                    n.bold());
            },
            Err(e) => {
                println!("{}", e);
            },
        }
    }  else if matches.subcommand_matches("commit").is_some() {
        panic!("Not implemented yet");
        #[allow(unreachable_code)] {
            let r = submitter::commit().await;

            match r {
                Ok(_) => {
                    println!("Success!")
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    } else if matches.subcommand_matches("describe").is_some() {
        let r = describer::describe();
        if let Err(x) = r {
            println!("{}", x);
        }
    }
}
