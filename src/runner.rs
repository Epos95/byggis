use std::collections::HashMap;
use std::fs;
use serde_json;
use std::process::{Command, Stdio};
use std::io::prelude::*;
use termion::*;
use byggis::ByggisErrors;
use regex::Regex;
use std::io;

pub fn run_tests() -> Result<(), ByggisErrors> {

    let file = match fs::File::open("./.byggis") {
        Ok(n) => n,
        Err(_) => {return Err(ByggisErrors::ByggisFileNotFound);},
    };

    let tests: HashMap<String, String> = match serde_json::from_reader(file) {
        Ok(n) => n,
        Err(_) => {return Err(ByggisErrors::TestsNotFound);},
    };

    let re = Regex::new(r"\./main\..{1,5}").unwrap();
    let f_vec: Vec<String> = fs::read_dir("./")
        .unwrap()
        .map(|x| x.unwrap()
            .path()
            .display()
            .to_string())
        .filter(|x| re.is_match(x))
        .collect();
    
    // now checks for main without bugging out if it doesnt exist
    if f_vec.is_empty() {
        return Err(ByggisErrors::MainNotFound);
    }

    let mut num: i32 = 1;
    if f_vec.len() > 1 {
        // more than one main file
        println!("  {}Note{}: Detected more than one main file...",
            color::Fg(color::Blue),
            color::Fg(color::Reset));
        println!("   Select main file to use:");

        for (i, f) in f_vec.iter().enumerate() {
            println!("     {}: {}{}{}", 
                i+1, 
                style::Bold,
                f,
                style::Reset);
        }

        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Could not read from stdin");
        n.pop();

        num = n.parse().unwrap_or_else(|_| {
            println!("    {}Error{}: Could not convert to int, defaulting to first option.",
                color::Fg(color::Red),
                color::Fg(color::Reset));
            1
        });

        if num > f_vec.len() as i32 {
            println!("    {}Error{}: Number not an option, defaulting to first option",
                color::Fg(color::Red),
                color::Fg(color::Reset));
            num = 1;
        }
    }

    // check if language is implemented here


    let main_file: &String = &f_vec[(num-1) as usize];

    let language = match main_file.split(".").last().unwrap() {
        "py" => "python",
        "rs" => {
            let p = Command::new("rustc")
                .arg("-A")
                .arg("warnings")
                .arg("main.rs")
                .stdin( Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            
            let o = &p.wait_with_output();
            let stderr = &String::from_utf8_lossy(&o.as_ref().unwrap().stderr);

            if stderr != "" {
                return Err(ByggisErrors::CompileTimeError(stderr.trim().to_string()))
            }

            // executable file name is expected to be "main"
            "rust"
        },
        _ => {
            return Err(ByggisErrors::UnknownLanguage);
        }
    };

    for (s_input, s_output) in tests {

        let mut p;
        if language == "rust" {
            p = Command::new("./main")
                .stdin( Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            
        } else if language == "python" {

            p = Command::new("python")
                .arg("main.py")
                .stdin( Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
        } else {
            panic!("Something went very, very wrong");
        }

        p.stdin
            .as_mut()
            .unwrap()
            .write(s_input.as_bytes())
            .unwrap();

        println!("   Testing {}{}{} against program...", style::Bold, s_input.trim(), style::Reset);
        
        let o = &p.wait_with_output();

        if String::from_utf8_lossy(&o.as_ref().unwrap().stdout) == s_output {
            println!("    Test: {}ok{}\n", color::Fg(color::Green), color::Fg(color::Reset));
        } else {
            println!("    Test: {}failed{}", color::Fg(color::Red), color::Fg(color::Reset));

            if String::from_utf8_lossy(&o.as_ref().unwrap().stderr).trim() != "" {
                // handle runtime errors and pretty print them
                
                println!("     Error:");
                
                for line in String::from_utf8_lossy(&o.as_ref().unwrap().stderr).trim().split("\n") {
                    println!("      {}{}{}", style::Bold, line, style::Reset);
                }

                println!("");
            } else {
                println!("     Output: {}{}{}", 
                    style::Italic, 
                    String::from_utf8_lossy(&o
                        .as_ref()
                        .unwrap()
                        .stdout).trim(), 
                    style::Reset);
                println!("");
            }
        }
    }

    Ok(())
}