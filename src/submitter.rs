// this file should contain all the code needed to replicate submitting from the
// terminal
use std::process::Command;
use std::fs;

use byggis::ByggisErrors;


pub fn commit(path: String) -> Result<(), ByggisErrors> {

    // first try to read from "path" to see if it exists etc

    // tries to read config from path

    let token = match fs::read_to_string(path) {
        Ok(n) => n, 
        Err(_) => { return Err(ByggisErrors::ConfigFileNotFound); },
    };

    // efter detta har vi ett (antagligen) giltigt token att använda





    Ok(())
}

// be användaren om namnet på problemet om det misslyckas att läsa från directory
fn get_problem_name() -> Option<String> {
    // should return the directory name 
    let o = match Command::new("pwd").output() {
        Ok(n) => {
            String::from_utf8_lossy(&n.stdout).to_string()
        }, 
        Err(_) => { return None; }
    };

    match o.split("/").last() {
        Some(n) => Some(n.to_string()),
        _ => None
    }
}

fn get_credentials(path: String) -> Option<(String, String)> {
    // should try and get the credentials from the file in the path
    Some(("".to_string(), "".to_string()))
}

