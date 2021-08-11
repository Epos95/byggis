use std::collections::HashMap;
use select::document::Document;
use select::predicate::Name;
use select::node::Node;
use reqwest;
use std::fs;
use serde_json;
use byggis::*;
use crossterm::style::*;
use std::io;

use std::io::prelude::*;
pub async fn create_new(name: String) -> Result<String, ByggisErrors> {

    let op = get_from_string(&name).await;
    match &op {
        Ok(response) => {
            if response.status() == 404 {
                return Err(ByggisErrors::ProblemNotFound);
            } 
        },
        Err(_) => {
            return Err(ByggisErrors::NetworkError);
        }
    }

    // this might be a bit of a bad design since the above match statment 
    // basically already unwraps op (also dont use unwrap) 
    let html = op.unwrap().text().await.unwrap();
    let document = Document::from(html.as_str());
    let hmap = get_samples(document);

    if fs::create_dir(&name).is_err() {
        return Err(ByggisErrors::DirectoryNotCreated);
    }

    let file = match fs::File::create(&format!("{}/.byggis", &name)) {
        Ok(n) => n,
        Err(_) => {return Err(ByggisErrors::ByggisFileNotCreated);},
    };

    if let Err(_) = serde_json::to_writer(file, &hmap) {
        return Err(ByggisErrors::ByggisFileNotCreated);
    }

    println!("  Create main file? [y/n]");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("IO error.");

    if buffer.to_lowercase().chars().next().unwrap_or('n') == 'y' {
        for (i, lang) in SupportedLanguages::into_enum_iter().enumerate() {
            println!("    {}: {}", i+1, lang.name());
        }

        println!("  Choose a language. [n]");
        buffer = String::new();
        stdin.read_line(&mut buffer).expect("IO error.");

        let n = buffer.trim_end().parse().unwrap_or(999)-1;

        let langs = SupportedLanguages::into_enum_iter()
            .collect::<Vec<SupportedLanguages>>();
        
        let lang = langs
            .get(n)
            .unwrap_or(&SupportedLanguages::Python);

        let contents = get_contents(lang.clone(), name.clone());

        let mut file = match fs::File::create(&format!("{}/main.{}", name, lang.extension())) {
            Ok(n)  => n,
            Err(_) => { return Err(ByggisErrors::MainFailure); },
        };

        match file.write_all(contents.as_bytes()) {
            Ok(_)  => {}
            Err(_) => { return Err(ByggisErrors::MainFailure); },
        }
    }

    Ok(name)
}

async fn get_from_string(problem: &String) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!("https://open.kattis.com/problems/{}", problem);
    let res = reqwest::get(&url);

    res.await
}

fn get_samples(document: Document) -> HashMap<String, String> {
    // you know what: 
    // YES this function will shit the bed if it encounters <pre> tags in any 
    // place that isnt the input/output spec but if f the question does that
    // then tbh fuck me


    // you should play around with the select library too learn it a bit better
    // rewrite according to the spec from runner.rs
    let mut hmap: HashMap<String, String> = HashMap::new();
    let c: Vec<Node> = document.find(Name("pre")).collect();

    if c.len() % 2 != 0 {
        println!("    {}{}",
                 "BIG ERROR".red(),
                 ": This kattis problem has a weird problem definition\
                  which caused the webscraping part to shit the bed.\n Do not use \
                  byggis with this problem.");
        panic!("Oopsie woopsie, we did a fucky wucky. We are working wevy hard to fix this error");
    }

    let mut counter = 1;
    for thing in c.iter().step_by(2) {
        hmap.insert(thing.text(), c[counter].text());
        counter += 2;
    }

    hmap
}

fn get_contents(lang: &SupportedLanguages, name: String) -> String {
    match lang {
        SupportedLanguages::Python => {
            format!("# main.py for problem: {}\n{}",
                    name,
                    "from sys import stdin\n\n")
        },
        SupportedLanguages::Rust => {
            format!("// main.rs for problem: {}\n\n{}",
                    name,
                    "use std::io::{self, BufRead};\n\nfn main() {\n\tlet stdin = io::stdin();\n\n}")
        },
        SupportedLanguages::Java => {
            format!("// main.java for problem: {}\nimport java.util.Scanner;\npublic class {}",
                    name,
                    name)
        },
        SupportedLanguages::Haskell => {
            format!("-- main.hs for problem: {}\n\n{}",
                    name,
                    "readInput = (map read) . words\nmain = interact (writeOutput . solve . readInput)\n-- this is the solve function\nsolve = ")
        },
    }
}