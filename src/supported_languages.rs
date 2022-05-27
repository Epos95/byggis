use byggis::ByggisErrors;
use enum_iterator::IntoEnumIterator;
use std::process::{Child, Command, Stdio};

#[derive(Debug, IntoEnumIterator, PartialEq)]
/// Enum describing the different kinds of supported languages.
/// To implement a new language we just have to add things to `SupportedLanguages`
/// and the language will be useable.
pub enum SupportedLanguages {
    Python,
    Rust,
    Haskell,
    Java,
}

impl SupportedLanguages {
    /// Create a langauge from a string.
    pub fn from_string(s: String) -> Option<Self> {
        match s.as_str() {
            "rust" | "rs" => Some(SupportedLanguages::Rust),
            "python" | "py" => Some(SupportedLanguages::Python),
            "java" => Some(SupportedLanguages::Java),
            "haskell" | "hs" => Some(SupportedLanguages::Haskell),
            _ => None,
        }
    }

    /// Get the name of the language.
    pub fn name(&self) -> String {
        match &self {
            SupportedLanguages::Rust => String::from("rust"),
            SupportedLanguages::Python => String::from("python"),
            SupportedLanguages::Java => String::from("java"),
            SupportedLanguages::Haskell => String::from("haskell"),
        }
    }

    /// Get the extension of the language.
    pub fn extension(&self) -> String {
        match &self {
            SupportedLanguages::Rust => String::from("rs"),
            SupportedLanguages::Python => String::from("py"),
            SupportedLanguages::Java => String::from("java"),
            SupportedLanguages::Haskell => String::from("hs"),
        }
    }

    /// Get the file contents for a language.
    pub fn get_contents(&self, name: &String) -> String {
        match self {
            SupportedLanguages::Python => {
                format!(
                    "# main.py for problem: {}\n{}",
                    name, "from sys import stdin\n\n"
                )
            }
            SupportedLanguages::Rust => {
                format!(
                    "// main.rs for problem: {}\n\n{}",
                    name,
                    "use std::io::{self, BufRead};\n\nfn main() {\n\tlet stdin = io::stdin();\n\n}"
                )
            }
            SupportedLanguages::Java => {
                format!(
                    "// main.java for problem: {}\nimport java.util.Scanner;\npublic class {} {{\n\n}}",
                    name, name
                )
            }
            SupportedLanguages::Haskell => {
                format!("-- main.hs for problem: {}\n\n{}",
                        name,
                        "readInput = (map read) . words\nmain = interact (writeOutput . solve . readInput)\n-- this is the solve function\nsolve = ")
            }
        }
    }

    /// Spawns and executes process for program execution.
    pub fn get_process(&self) -> Child {
        match self {
            SupportedLanguages::Rust => Command::new("./main")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap(),
            SupportedLanguages::Python => Command::new("python")
                    .arg("main.py")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("Failed to spawn python process!\tIf this happens you probably dont have 'python' availible, try aliasing 'python3' to alias or make sure python is installed propperly."),
            SupportedLanguages::Java => Command::new("java")
                .arg("main.class")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap(),
            SupportedLanguages::Haskell => Command::new("./main")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap(),
        }
    }

    /// Compiles the selected language according to how
    /// kattis will compile it when testing the code.
    pub fn compile(&self) -> Result<(), ByggisErrors> {
        match self {
            SupportedLanguages::Python => Ok(()),
            SupportedLanguages::Rust => {
                let p = Command::new("rustc")
                    .arg("-A")
                    .arg("warnings")
                    .arg("main.rs")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap();

                let o = &p.wait_with_output();
                let stderr = &String::from_utf8_lossy(&o.as_ref().unwrap().stderr);

                if stderr != "" {
                    Err(ByggisErrors::CompileTimeError(stderr.trim().to_string()))
                } else {
                    Ok(())
                }
            }
            SupportedLanguages::Java => {
                let p = Command::new("javac")
                    .arg("main.java")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap();

                let o = &p.wait_with_output();
                let stderr = &String::from_utf8_lossy(&o.as_ref().unwrap().stderr);

                if stderr != "" {
                    Err(ByggisErrors::CompileTimeError(stderr.trim().to_string()))
                } else {
                    Ok(())
                }
            }
            SupportedLanguages::Haskell => {
                let p = Command::new("ghc")
                    .arg("main.hs")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap();

                let o = &p.wait_with_output();
                let stderr = &String::from_utf8_lossy(&o.as_ref().unwrap().stderr);

                if stderr != "" {
                    return Err(ByggisErrors::CompileTimeError(stderr.trim().to_string()));
                } else {
                    Ok(())
                }
            }
        }
    }

    pub fn guess(&self) -> String {
        match self {
            SupportedLanguages::Python => "Python 3",
            SupportedLanguages::Rust => "Rust",
            SupportedLanguages::Java => "Java",
            SupportedLanguages::Haskell => "Haskell",
        }
        .to_string()
    }
}
