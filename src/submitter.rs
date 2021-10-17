#![allow(unused_variables)]
#![allow(unused_mut)]

// this file should contain all the code needed to replicate submitting from the terminal
use dirs::home_dir;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

use std::path::Path;

use regex::Regex;

use byggis::ByggisErrors;

use reqwest::{cookie::Cookie, header::HeaderValue, Method, Request, Url};

use crate::supported_languages::SupportedLanguages;

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
                None => {
                    return Err(ByggisErrors::ConfigFileNotFound);
                }
            }
        }
    };

    println!("[DEBUG]\tusername: {}\ttoken: {}\n", username, token);

    // efter detta har vi ett (antagligen) giltigt token att använda

    // vi behöver fortfarande behålla cookies från inloggningen
    let r = match login(username, token).await {
        Ok(n) => {
            if n.status().is_success() {
                n
            } else if n.status().is_client_error() {
                return Err(ByggisErrors::InvalidToken);
            } else {
                return Err(ByggisErrors::NetworkError);
            }
        }
        Err(_) => {
            return Err(ByggisErrors::NetworkError);
        }
    };

    // get all files in directory
    let re = Regex::new(r"\./main\..{1,5}").unwrap();
    let filenames: Vec<String> = fs::read_dir("./")
        .unwrap()
        .map(|x| x.unwrap().path().display().to_string())
        .filter(|x| re.is_match(x))
        .collect();

    // get the main file
    let main_file = byggis::get_mainfile(filenames)?;
    let language = SupportedLanguages::from_string(main_file
                                                   .split(".")
                                                   .last()
                                                   .unwrap()
                                                   .to_string())
        .unwrap();

    // get the problem id (the name of the directory)
    let problemid = std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        .split("/")
        .last()
        .unwrap();

    // get cookies from "headers"
    let cookie_pair = r
        .headers()
        .get("set-cookie")
        .expect("No cookies :(")
        .to_str()
        .unwrap()
        .split(";")
        .next()
        .unwrap()
        .split("=")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let data = HashMap::new();
    data.insert("submit", "true");
    data.insert("submit_ctr", "2");
    data.insert("language", &language.guess());
    data.insert("mainclass", MAINCLASS); // mainclass is what?
    data.insert("problem", problemid); // problemid is easy, just directory name
    data.insert("tag", ""); // idk what a tag is
    data.insert("script", "true");

    // create request
    let client = reqwest::Client::new();
    let req = client
        .post("https://open.kattis.com/submit") // kattis link here
        .header(cookie_pair[0].as_str(), cookie_pair[1])
        .header("User-Agent", "kattis-cli-submit")
        .form(&data) // "data" from submit.py goes here, takes a hashmap
        .body(); // content of the selected main file goes here

    // send request
    let mut request = Request::new(
        Method::POST,
        Url::parse("https://open.kattis.com/submit").unwrap(),
    );

    Ok(())
}

/// Gets kattis credentials from a specified path. Can fail.
fn get_credentials(path: PathBuf) -> Option<(String, String)> {
    let config = match fs::read_to_string(path) {
        Ok(n) => n,
        Err(_) => {
            return None;
        }
    };

    let mut username: String = "".to_string();
    let mut token: String = "".to_string();

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

async fn login(user: String, token: String) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let p = [
        ("user", user),
        ("script", "true".to_string()),
        ("token", token),
    ];

    client
        .post("https://open.kattis.com/login")
        .form(&p)
        .send()
        .await
}
