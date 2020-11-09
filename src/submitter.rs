// this file should contain all the code needed to replicate submitting from the terminal
use std::process::Command;
use std::stdin;

use byggis::ByggisErrors;


/*!! program flow !!
 * Handle config file
 * login to kattis / try the username and token
 * Try to submit the thing
 * Handle result from the submission
 */


pub fn commit() -> Result<(), ByggisErrors> {

    // first try to read from "path" to see if it exists etc

    // tries to read config from path

    let standard_path = "~/.kattisrc".to_string();
    // dont know if ~ will expand propperly but heres to hoping

    let result = get_credentials(standard_path);
    let (username, token) = result.unwrap_or_else(|| {
        println!("Could not find credentials in that location.\nEnter path to file: ");

        let mut p = String::new();
        let mut stdin = io::stdin();
        stdin.read_to_string(&mut p);

        get_config(p)
    });

    let (username, token) = match get_credentials(standard_path) {
        Some(x, y) => (x, y),
        None => { return Err(ByggisErrors::ConfigFileNotFound); },
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

fn get_credentials(path: String) -> Option<(String, String> {
    // should try and get the credentials from the file in the path
    // the file in the pathn *should* be a valid kattisrc file or one containing similair
    // content

    let username = "".to_string();
    let token    = "".to_string();

    Some((username, token))
}
