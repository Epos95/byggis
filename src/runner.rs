use std::collections::HashMap;
use std::fs;
use serde_json;
use std::process::{Command, Stdio};
use std::io::prelude::*;
use termion::*;
use byggis::ByggisErrors;

pub fn run_tests() -> Result<(), ByggisErrors> {

    let file = match fs::File::open("./.byggis") {
        Ok(n) => n,
        Err(_) => {return Err(ByggisErrors::ByggisFileNotFound);},
    };

    let tests: HashMap<String, String> = match serde_json::from_reader(file) {
        Ok(n) => n,
        Err(_) => {return Err(ByggisErrors::TestsNotFound);},
    };

    let mut f_vec: Vec<String> = fs::read_dir("./")
        .unwrap()
        .map(|x| x.unwrap()
            .path()
            .display()
            .to_string())
        .collect();
    
    for f in f_vec.iter_mut() {
        f.split_off(7).truncate(7);
    }
    
    if !f_vec.contains(&"./main.".to_string()) {
        return Err(ByggisErrors::MainNotFound);
    }

    // check file ending here or something 
    // for different languages you should implement 
    // the selection logic here

    for (s_input, s_output) in tests {
        let mut p = Command::new("python")
            .arg("main.py")
            .stdin( Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Could not spawn process for main");

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
                println!("     Error:");
                
                for line in String::from_utf8_lossy(&o.as_ref().unwrap().stderr).trim().split("\n") {
                    println!("      {}{}{}", style::Bold, line, style::Reset);
                }
                println!("");
            } else {
                println!("     Output: {}{}{}", style::Italic, String::from_utf8_lossy(&o.as_ref().unwrap().stdout).trim(), style::Reset);
                println!("");
            }
        }
    }

    Ok(())
}