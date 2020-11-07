use std::collections::HashMap;
use std::fs;
use serde_json;
use std::process::{Command, Stdio};
use std::io::prelude::*;
use termion::*;
use byggis::ByggisErrors;
use regex::Regex;
use std::io;

// NOTE: Should probably split this into multiple functions for easier reading and stuff
pub fn run_tests() -> Result<(), ByggisErrors> {

    // get .byggis file to read tests from
    let dot_byggis = match fs::File::open("./.byggis") {
        Ok(n) => n,
        Err(_) => { return Err(ByggisErrors::ByggisFileNotFound); },
    };

    // reads and error handles tests from .byggis file
    // TODO: rewrite how the hashmap is designed to be something akin to:
    //      {
    //           test_inputs : {
    //               []
    //           },
    //           test_outputs : {
    //                []
    //           },
    //       }
    // to let this serialize well with serde we should prolly serialize it into 
    // a custom struct, however this is not necesary until we work on commiting
    // The DotByggis struct in lib.rs can be used for this 

    let tests: HashMap<String, String> = match serde_json::from_reader(dot_byggis) {
        Ok(n) => n,
        Err(_) => { return Err(ByggisErrors::TestsNotFound); },
    };


    // use regex to put the files in a vector for easy access
    let re = Regex::new(r"\./main\..{1,5}").unwrap();
    let f_vec: Vec<String> = fs::read_dir("./")
        .unwrap()
        .map(|x| x.unwrap()
            .path()
            .display()
            .to_string())
        .filter(|x| re.is_match(x))
        .collect();


    // error handling for if the folder is empty, e.g no files found
    if f_vec.is_empty() {
        return Err(ByggisErrors::MainNotFound);
    }


    // get and parse the input from the user
    let mut num: i32 = 1;
    if f_vec.len() > 1 {
        // handles more than one main file
        println!("  {}Note{}: Detected more than one main file...",
            color::Fg(color::Blue),
            color::Fg(color::Reset));
        println!("   Select main file to use:");


        // prints out the files in a nice manner
        for (i, f) in f_vec.iter().enumerate() {
            println!("     {}: {}{}{}",
                i+1,
                style::Bold,
                f,
                style::Reset);
        }


        // read from stdin to n to be used as a option in selection process
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Could not read from stdin");
        n.pop();


        // parse the input from n/stdin into a clean integer
        num = n.parse().unwrap_or_else(|_| {
            println!("    {}Error{}: Could not convert to int, defaulting to first option.",
                color::Fg(color::Red),
                color::Fg(color::Reset));
            1
        });


        // error checking operation, defaults to first option
        if num > f_vec.len() as i32 {
            println!("    {}Error{}: Number not an option, defaulting to first option",
                color::Fg(color::Red),
                color::Fg(color::Reset));
            num = 1;
        }
    }


    // gets the file name from the vector of names based on the inputed index
    let main_file: &String = &f_vec[(num-1) as usize];


    // get the used language and compile/setup for running the file
    let language = match main_file.split(".").last().unwrap() {
        "py" => "python",
        "rs" => {
            // create a process for compiling rust file and check output
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


            "rust"
        },
        _ => {
            return Err(ByggisErrors::UnknownLanguage);
        }
    };


    // run the file against the tests
    for (s_input, s_output) in tests {

        // spawn process and execute file
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


        // feed the process the input from the file
        p.stdin
            .as_mut()
            .unwrap()
            .write(s_input.as_bytes())
            .unwrap();

        println!("   Testing {}{}{} against program..", style::Bold, s_input.trim(), style::Reset);

        let o = &p.wait_with_output();

        // print out the test results
        if String::from_utf8_lossy(&o.as_ref().unwrap().stdout) == s_output {
            println!("    Test: {}ok{}\n", color::Fg(color::Green), color::Fg(color::Reset));
        } else {
            println!("    Test: {}failed{}", color::Fg(color::Red), color::Fg(color::Reset));

            // handle runtime errors and pretty print them
            //  NOTE: This does not get handled by main.rs since its a 
            //        recoverable error, e.g we still want the program to finish 
            //        safely after this happens
            if String::from_utf8_lossy(&o.as_ref().unwrap().stderr).trim() != "" {

                println!("     Error:");

                for l in String::from_utf8_lossy(&o.as_ref().unwrap().stderr).trim().split("\n") {
                    println!("      {}{}{}", style::Bold, l, style::Reset);
                }

                println!("");
            } else {
                // prints output of the test
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
