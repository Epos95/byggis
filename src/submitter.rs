// this file should contain all the code needed to replicate submitting from the
// terminal
use byggis::ByggisErrors;

pub fn commit(path: String) -> Result<(), ByggisErrors> {

    // first try to read from "path" to see if it exists etc



    Ok(())
}

fn get_problem_name() -> Option<String> {
    // should return the directory name 
    Some("".to_string())
}

fn get_credentials(path: String) -> Option<(String, String)> {
    // should try and get the credentials from the file in the path
    Some(("".to_string(), "".to_string()))
}

