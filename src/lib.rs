pub use enum_iterator::IntoEnumIterator;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fmt;
use crossterm::style::*;

#[derive(Serialize, Deserialize)]
pub struct DotByggis {
    pub tests: HashMap<String, String>,
    pub description: Vec<String>
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
                format!("   {}: Could not connect to open.kattis.com",
                        "Error".red())
            },
            ByggisErrors::DirectoryNotCreated => {
                format!("   {}: Directory could not be created",
                    "Error".red())
            },
            ByggisErrors::ByggisFileNotCreated => {
                format!("   {}: byggis file could not be created",
                    "Error".red())
            },
            ByggisErrors::ProblemNotFound => {
                format!("   {}: Could not find that problem on kattis",
                    "Error".red())
            }, 
            ByggisErrors::ByggisFileNotFound => {
                format!("   {}: Could not find byggis file in folder\n{}",
                    "Error".red(),
                    "    Did you run \"byggis new 'name'\"?")
            }, 
            ByggisErrors::TestsNotFound => {
                format!("   {}: Could not find tests in byggis file",
                    "Error".red())
            },
            ByggisErrors::MainNotFound => {
                format!("   {}: Could not find a main file to test with",
                    "Error".red())
            }, 
            ByggisErrors::CompileTimeError(s) => {
                let mut x = format!("     Compilation error:");

                for line in s.trim().split("\n") {
                    x = format!("{}    {}\n", x, line.bold());
                }
                
                format!("{}\n", x)
            },
            ByggisErrors::ConfigFileNotFound => {
                format!("   {}: Could not find config file containing token\n    You can generate one with \"{}\"",
                    "Error".red(),
                    "Byggis generate".bold())
            },
            ByggisErrors::InvalidToken => {
                format!("   {}: Invalid token",
                    "Error".red())
            },
            ByggisErrors::MainFailure => {
                format!("   {}: Could not create main file", "Error".red())
            },
        };

        write!(f, "{}", thing)
    }
}

#[derive(Debug, IntoEnumIterator, PartialEq)]
/// Enum describing the different kinds of supported languages.
/// To implement a new language we just have to add things to `SupportedLanguages`
/// and the language will be useable.
pub enum SupportedLanguages {
    Rust,
    Java,
    Python,
    Haskell,
}
 
impl SupportedLanguages {
    /// Create a langauge from a string.
    pub fn from_string(s: String) -> Option<Self> {
        match s.as_str() {
            "rust" | "rs"    => Some(SupportedLanguages::Rust),
            "python" | "py"  => Some(SupportedLanguages::Python),
            "java"           => Some(SupportedLanguages::Java),
            "haskell" | "hs" => Some(SupportedLanguages::Haskell),
            _                => None
        }
    }

    /// Get the name of the language.
    pub fn name(&self) -> String {
        match &self {
            SupportedLanguages::Rust    => String::from("rust"),
            SupportedLanguages::Python  => String::from("python"),
            SupportedLanguages::Java    => String::from("java"),
            SupportedLanguages::Haskell => String::from("haskell")
        }
    }

    /// Get the extension of the language.
    pub fn extension(&self) -> String {
        match &self {
            SupportedLanguages::Rust    => String::from("rs"),
            SupportedLanguages::Python  => String::from("py"),
            SupportedLanguages::Java    => String::from("java"),
            SupportedLanguages::Haskell => String::from("hs")
        }
    }

    /// Get the file contents for a language.
    pub fn get_contents(&self, name: &String) -> String {
        match self {
            SupportedLanguages::Python => {
                format!("# main.py for problem: {}\n{}",
                        name,
                        "from sys import stdin\n\n")
            },
            SupportedLanguages::Rust => {
                format!("// main.rs for problem: {}\n\n{}",
                        name,
                        "use std::io::{self, BufRead};\n\nfn main() {\n\tlet stdin = io::stdin();\n\n}")
            },
            SupportedLanguages::Java => {
                format!("// main.java for problem: {}\nimport java.util.Scanner;\npublic class {}",
                        name,
                        name)
            },
            SupportedLanguages::Haskell => {
                format!("-- main.hs for problem: {}\n\n{}",
                        name,
                        "readInput = (map read) . words\nmain = interact (writeOutput . solve . readInput)\n-- this is the solve function\nsolve = ")
            },
        }
    }
}
