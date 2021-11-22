
// this file should contain all the code needed to replicate submitting from the terminal
use dirs::home_dir;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

use pyo3::prelude::*;
use pyo3::types::PyModule;

use std::path::Path;

use regex::Regex;

use byggis::ByggisErrors;

use crate::supported_languages::SupportedLanguages;

/*
 * Handle config file
 * login to kattis / try the username and token
 * Try to submit the thing
 * Handle result from the submission
*/

pub async fn submit() -> Result<(), ByggisErrors> {
    // first try to read from "path" to see if it exists etc

    // tries to read config from path
    let mut standard_path: PathBuf = home_dir().unwrap();
    standard_path.push(".kattisrc");

    // read credentials from config file
    let result = get_credentials(standard_path);
    let (username, token) = match result {
        Some((x, y)) => (x, y),
        None => {
            println!("Could not find credentials in that location.\nEnter path to file: ");

            let mut p = String::new();
            let stdin = io::stdin();
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

    // only for debugging
    println!("[DEBUG]\tusername: {}\ttoken: {}\n", username, token);

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

    // convert the files tail to a language
    let language_guess = SupportedLanguages::from_string(main_file
                                                   .split(".")
                                                   .last()
                                                   .unwrap()
                                                   .to_string())
        .unwrap().guess();

    // get the problem id (the name of the directory)
    let problemid = std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
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

    // build the data hashmap
    let form_data: HashMap<&str, &str> = vec![
        ("submit", "true"),
        ("submit_ctr", "2"),
        ("language", &language_guess),
        ("mainclass", ""), // mainclass is what?
        ("problem", problemid.split("/").last().unwrap()), // problemid is easy, just directory name
        ("script", "true"),
    ].iter().map(|x| *x).collect();

    // use pyo3 to use the requests library natively cuz reqwest is kinda bad(ly documented)
    Python::with_gil(|py| {
        let thing = PyModule::from_code(py, r#"
def submit(data, cookies, file):
    import requests
    print(cookies)
    import os

    headers = {'User-Agent': 'kattis-cli-submit'}

    sub_files = []
    with open(file) as sub_file:
        sub_files.append(('sub_file[]',
                          (os.path.basename(file),
                           sub_file.read(),
                           'application/octet-stream')))

    #r = requests.post(
            #"https://open.kattis.com/",
            #data=data,
            #cookies={cookies[0], cookies[1]},
            #files=sub_files,
            #headers=headers)

    #print(r)

"#, "test.py", "test").unwrap();


        thing.call_method1("submit", (form_data, cookie_pair, main_file)).unwrap();
    });



    Ok(())
}

/// Gets kattis credentials from a specified path. Can panic.
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

//fn
