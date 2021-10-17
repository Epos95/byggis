use crossterm::style::*;
pub use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::io;

#[derive(Serialize, Deserialize)]
pub struct DotByggis {
    pub tests: HashMap<String, String>,
    pub description: Vec<String>,
}

/// Error enum containing all the errors that can be returned from the modules
pub enum ByggisErrors {
    /// Reflects errors connecting to kattis
    NetworkError,
    /// Reflects error scenario where directory could not be created
    DirectoryNotCreated,
    /// Reflects error scenario where byggis file cannot be created
    ByggisFileNotCreated,
    /// Reflects 404 from kattis when searching for the problem
    ProblemNotFound,
    /// Scenario where byggis file is not found
    ByggisFileNotFound,
    /// Scenario where the tests in the file is not found
    TestsNotFound,
    /// Reflects scenario where main file cant be found in the directory
    MainNotFound,
    /// Reflects scenario where the users code cant compile
    CompileTimeError(String),
    /// Scenario where byggis cannot find a config file to read token from
    ConfigFileNotFound,
    /// Scenario where kattis refuses the given token
    InvalidToken,
    /// Scenario where byggis cannot create the main file
    MainFailure,
}

// Maybe implement as a method for byggiserrors instead
// byggiserror::error.display() does make alot of sense
impl fmt::Display for ByggisErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let thing = match &self {
            ByggisErrors::NetworkError => {
                format!("   {}: Could not connect to open.kattis.com", "Error".red())
            }
            ByggisErrors::DirectoryNotCreated => {
                format!("   {}: Directory could not be created", "Error".red())
            }
            ByggisErrors::ByggisFileNotCreated => {
                format!("   {}: byggis file could not be created", "Error".red())
            }
            ByggisErrors::ProblemNotFound => {
                format!(
                    "   {}: Could not find that problem on kattis",
                    "Error".red()
                )
            }
            ByggisErrors::ByggisFileNotFound => {
                format!(
                    "   {}: Could not find byggis file in folder\n{}",
                    "Error".red(),
                    "    Did you run \"byggis new 'name'\"?"
                )
            }
            ByggisErrors::TestsNotFound => {
                format!("   {}: Could not find tests in byggis file", "Error".red())
            }
            ByggisErrors::MainNotFound => {
                format!(
                    "   {}: Could not find a main file to test with",
                    "Error".red()
                )
            }
            ByggisErrors::CompileTimeError(s) => {
                let mut x = format!("     Compilation error:");

                for line in s.trim().split("\n") {
                    x = format!("{}    {}\n", x, line.bold());
                }

                format!("{}\n", x)
            }
            ByggisErrors::ConfigFileNotFound => {
                format!("   {}: Could not find config file containing token\n    You can generate one with \"{}\"",
                    "Error".red(),
                    "Byggis generate".bold())
            }
            ByggisErrors::InvalidToken => {
                format!("   {}: Invalid token", "Error".red())
            }
            ByggisErrors::MainFailure => {
                format!("   {}: Could not create main file", "Error".red())
            }
        };

        write!(f, "{}", thing)
    }
}

pub fn get_mainfile(f_vec: Vec<String>) -> Result<String, ByggisErrors> {
    // error handling for if the folder is empty, e.g no files found
    if f_vec.is_empty() {
        return Err(ByggisErrors::MainNotFound);
    }

    let mut file_index: i32 = 1;

    // if more than one main file detected...
    if f_vec.len() > 1 {
        println!("  {}: Detected more than one main file...", "Note".blue());
        println!("   Select main file to use:");

        // prints out the files in a nice manner
        for (i, f) in f_vec.iter().enumerate() {
            println!("     {}: {}", i + 1, f.to_string().bold());
        }

        // read from stdin to n to be used as a option in selection process
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Could not read from stdin");
        n.pop();

        // parse the input from n/stdin into a clean integer
        file_index = n
            .replace("\n", "")
            .replace("\r", "")
            .parse()
            .unwrap_or_else(|_| {
                println!(
                    "    {}: Could not convert to int, defaulting to first option.",
                    "Error".red()
                );
                1
            });

        // error checking operation, defaults to first option
        if file_index > f_vec.len() as i32 {
            println!(
                "    {}:  Not an option, defaulting to first option",
                "Error".red()
            );
            file_index = 1;
        }
    }

    // gets the file name from the vector of names based on the inputed index
    Ok(f_vec[(file_index - 1) as usize].to_owned())
}
