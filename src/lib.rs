use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DotByggis {
    pub tests: HashMap<String, String>,
    pub description: Vec<String>
}

pub enum ByggisErrors {
    NetworkError,
        // reflects errors connecting to kattis
    DirectoryNotCreated,
        // reflects error scenario where directory could not be created
    ByggisFileNotCreated, 
        // reflects error scenario where byggis file cannot be created
    ProblemNotFound,
        // reflects 404 from kattis when searching for the problem
    ByggisFileNotFound,
        // scenario where byggis file is not found
    TestsNotFound,
        // scenario where the tests in the file is not found 
    MainNotFound,
        // reflects scenario where main file cant be found in the directory
    CompileTimeError(String),
        // reflects scenario where the users code cant compile
    UnknownLanguage,
        // reflects scenario where the file is in a unimplemented language
    ConfigFileNotFound,
        // scenario where byggis cannot find a config file to read token from
    InvalidToken,
        // scenario where kattis refuses the given token
    FileReadingError,
        // scenario where byggis cannot read from the token file
    MainFailure,
        // scenario where byggis cannot create the main file

}

pub use enum_iterator::IntoEnumIterator;
#[derive(Debug, IntoEnumIterator, PartialEq)]
pub enum SupportedLanguages {
    Rust,
    Java,
    Python,
    Haskell,
}
 
impl SupportedLanguages {
    pub fn from_string(s: String) -> Option<Self> {
        match s.as_str() {
            "rust" | "rs"    => Some(SupportedLanguages::Rust),
            "python" | "py"  => Some(SupportedLanguages::Python),
            "java"           => Some(SupportedLanguages::Java),
            "haskell" | "hs" => Some(SupportedLanguages::Haskell),
            _                => None
        }
    }

    pub fn name(&self) -> String {
        match &self {
            SupportedLanguages::Rust    => String::from("rust"),
            SupportedLanguages::Python  => String::from("python"),
            SupportedLanguages::Java    => String::from("java"),
            SupportedLanguages::Haskell => String::from("haskell")
        }
    }

    pub fn extension(&self) -> String {
        match &self {
            SupportedLanguages::Rust    => String::from("rs"),
            SupportedLanguages::Python  => String::from("py"),
            SupportedLanguages::Java    => String::from("java"),
            SupportedLanguages::Haskell => String::from("hs")
        }
    }
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