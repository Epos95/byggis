use crate::supported_languages::SupportedLanguages;
use byggis::ByggisErrors;
use byggis::DotByggis;
use crossterm::style::*;
use regex::Regex;
use serde_json;
use std::{fs, io::prelude::*, time::Instant};

// TODO: Needs to split up, like what the fuck is this
pub fn run_tests(test_time: bool) -> Result<(), ByggisErrors> {
    // get .byggis file to read tests from
    let dot_byggis = match fs::File::open("./.byggis") {
        Ok(n) => n,
        Err(_) => {
            return Err(ByggisErrors::ByggisFileNotFound);
        }
    };

    let dot_byggis: DotByggis = match serde_json::from_reader(dot_byggis) {
        Ok(n) => n,
        Err(_) => {
            return Err(ByggisErrors::TestsNotFound);
        }
    };

    // use regex to put the files in a vector for easy access
    let re = Regex::new(r"\./main\..{1,5}").unwrap();
    let filenames: Vec<String> = fs::read_dir("./")
        .unwrap()
        .map(|x| x.unwrap().path().display().to_string())
        .filter(|x| re.is_match(x))
        .collect();

    let main_file = byggis::get_mainfile(filenames)?;

    let language: SupportedLanguages =
        SupportedLanguages::from_string(main_file.split(".").last().unwrap().to_string()).unwrap();

    // get the used language and compile/setup for running the file
    language.compile()?;

    // run the file against the tests
    for (s_input, s_output) in dot_byggis.tests {
        // spawn process and execute file
        let mut p = language.get_process();

        // start timing
        let now = Instant::now();

        // feed the process the input from the file
        p.stdin.as_mut().unwrap().write(s_input.as_bytes()).unwrap();

        // gets the stderr and stdout of the program so we can test them.
        let o = p.wait_with_output();
        let stdout = String::from_utf8_lossy(&o.as_ref().unwrap().stdout);
        let stderr = String::from_utf8_lossy(&o.as_ref().unwrap().stderr);
        let output_string = stdout.trim();
        let stderr_string = stderr.trim();

        if output_string.replace("\r", "") == s_output.replace("\n", "") && stderr_string.is_empty()
        {
            // Tests completed and ok!
            println!("  Test result: {}", "ok".green());

            println!("   Test case:");
            for s in s_input.split("\n") {
                println!("     {}", s.trim().italic());
            }

            println!(
                "    Test took {} seconds to finish.",
                now.elapsed().as_secs_f32()
            );

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
