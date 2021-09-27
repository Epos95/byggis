#![allow(unused_variables)]
#![allow(unused_mut)]

// this file should contain all the code needed to replicate submitting from the terminal
use std::process::Command;
use std::fs;
use std::io;
use dirs::home_dir;
use std::path::PathBuf;

use std::path::Path;

use byggis::ByggisErrors;

/*
 * Handle config file
 * login to kattis / try the username and token
 * Try to submit the thing
 * Handle result from the submission
*/

pub async fn commit() -> Result<(), ByggisErrors> {

    // first try to read from "path" to see if it exists etc

    // tries to read config from path
    let mut standard_path: PathBuf = home_dir().unwrap();
    standard_path.push(".kattisrc");

    let result = get_credentials(standard_path);

    let (username, token) = match result {
        Some((x, y)) => (x, y),
        None => {
            println!("Could not find credentials in that location.\nEnter path to file: ");

            let mut p = String::new();
            let mut stdin = io::stdin();
            stdin.read_line(&mut p).unwrap();
            p.pop();

            match get_credentials(Path::new(&p).to_path_buf()) {
                Some((x, y)) => (x, y),
                None => { return Err(ByggisErrors::ConfigFileNotFound); }
            }
        }
    };

    println!("\n[DEBUG]\nusername: {}\ntoken: {}\n", username, token);

    // efter detta har vi ett (antagligen) giltigt token att använda

    // vi behöver fortfarande behålla cookies från inloggningen
    let headers = match login(username, token).await {
        Ok(n) => {
            if n.status().is_success() {
                n.headers().clone()
            } else if n.status().is_client_error() {
                return Err(ByggisErrors::InvalidToken);
            } else {
                return Err(ByggisErrors::NetworkError);
            }
        },
        Err(_) => { return Err(ByggisErrors::NetworkError); }
    };

    let problem_name = match get_problem_name() {
        Some(s) => s,
        None => {
            println!("Could not read directory name.\nWhat is the problems problem id?");

            let mut p = String::new();
            let mut stdin = io::stdin();
            stdin.read_line(&mut p).unwrap();
            p.pop();
            p
        }
    };

    // try and find main file

    Ok(())
}

/// Gets the problem id of the current problem, on failure to read it will ask the user for input.
fn get_problem_name() -> Option<String> {
    // should return the directory name
    // TODO: Fix this to be windows compatible/not dumb by not relying on pwd lmfao

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

/// Gets kattis credentials from a specified path. Can fail.
fn get_credentials(path: PathBuf) -> Option<(String, String)> {
    let config = match fs::read_to_string(path) {
        Ok(n) => n,
        Err(_) => { return None; } 
    };

    let mut username: String = "".to_string();
    let mut token:    String = "".to_string();

    for line in config.split("\n") {
        if line.contains("token: ") {
            token = line.split(" ").last()?.to_string();
        }

        if line.contains("username: ") {
            username = line.split(" ").last()?.to_string();
        }
    }

    Some((username, token))
}

// NOTE: Shouldnt we maybe save the cookies recieved from here?
async fn login(user: String, token: String) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let p = [
        ("user", user),
        ("script", "true".to_string()),
        ("token", token)
    ];

    client.post("https://open.kattis.com/login")
        .form(&p)
        .send()
        .await
}
