use std::{
    fs,
    io,
    io::prelude::*,
    time::Instant,
};
use serde_json;
use byggis::ByggisErrors;
use regex::Regex;
use crossterm::style::*;
use byggis::DotByggis;
use crate::supported_languages::SupportedLanguages;

// TODO: Needs to split up, like what the fuck is this
pub fn run_tests(test_time: bool) -> Result<(), ByggisErrors> {

    // get .byggis file to read tests from
    let dot_byggis = match fs::File::open("./.byggis") {
        Ok(n) => n,
        Err(_) => { return Err(ByggisErrors::ByggisFileNotFound); },
    };

    let dot_byggis: DotByggis = match serde_json::from_reader(dot_byggis) {
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

    let mut file_index: i32 = 1;

    // if more than one main file detected...
    if f_vec.len() > 1 {
        println!("  {}: Detected more than one main file...",
            "Note".blue());
        println!("   Select main file to use:");

        // prints out the files in a nice manner
        for (i, f) in f_vec.iter().enumerate() {
            println!("     {}: {}",
                i+1,
                f.to_string().bold());
        }

        // read from stdin to n to be used as a option in selection process
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Could not read from stdin");
        n.pop();

        // parse the input from n/stdin into a clean integer
        file_index = n.replace("\n","").replace("\r", "").parse().unwrap_or_else(|_| {
            println!("    {}: Could not convert to int, defaulting to first option.",
                "Error".red());
            1
        });

        // error checking operation, defaults to first option
        if file_index > f_vec.len() as i32 {
            println!("    {}: File_Indexber not an option, defaulting to first option",
                "Error".red());
            file_index = 1;
        }
    }

    // gets the file name from the vector of names based on the inputed index
    let main_file: &String = &f_vec[(file_index-1) as usize];

    let language: SupportedLanguages = SupportedLanguages::from_string(main_file.split(".").last().unwrap().to_string()).unwrap();

    // get the used language and compile/setup for running the file
    language.compile()?;

    // run the file against the tests
    for (s_input, s_output) in dot_byggis.tests {

        // spawn process and execute file
        let mut p = language.get_process();

        // start timing 
        let now = Instant::now();

        // feed the process the input from the file
        p.stdin
            .as_mut()
            .unwrap()
            .write(s_input.as_bytes())
            .unwrap();

        // gets the stderr and stdout of the program so we can test them.
        let o = p.wait_with_output();
        let stdout = String::from_utf8_lossy(&o.as_ref().unwrap().stdout);
        let stderr = String::from_utf8_lossy(&o.as_ref().unwrap().stderr);
        let output_string = stdout.trim();
        let stderr_string = stderr.trim();

        if output_string.replace("\r", "") == s_output && stderr_string.is_empty() {
            // Tests completed and ok!
            println!("  Test result: {}", "ok".green());

            println!("   Test case:");
            for s in s_input.split("\n") {
                println!("     {}", s.trim().italic());
            }

            println!("    Test took {} seconds to finish.", now.elapsed().as_secs_f32());

            if now.elapsed().as_secs_f32() > 1.0 && !test_time {
                println!("\n   {}: Time ran out", "Warning".yellow());
                println!("    Your program took too long to finish and ");
                println!("    might get rejected by kattis due to it. ");
            }
            println!();

        } else if !stderr_string.is_empty() {
            // Runtime error found!
            println!("  Test result: {}", "failed".red());

            println!("   Test case:");
            for s in s_input.split("\n") {
                println!("     {}", s.trim().italic());
            }

            println!("    Runtime error:");
            for l in stderr_string.split("\n") {
                println!("      {}", l.bold());
            }
            println!();
        } else {
            // wrong answer
            println!("  Test result: {}", "failed".red());

            println!("   Test case:");
            for s in s_input.split("\n") {
                println!("     {}", s.trim().italic());
            }

            println!("   Program output: ");

            for l in output_string.split("\n") {
                println!("     {}", l.bold());
            }

            println!();
        }
    }

    Ok(())
}
