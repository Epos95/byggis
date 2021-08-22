use std::{
    collections::HashMap,
    io,
    io::prelude::*,
    fs,
};
use reqwest;
use serde_json;
use byggis::*;
use crossterm::style::*;
use select::{
    document::Document,
    predicate::{
        Name,
        Class,
    },
    node::Node,
};

pub async fn create_new(name: String) -> Result<String, ByggisErrors> {

    let op = get_from_string(&name).await;
    let html = match op {
        Ok(response) => {
            if response.status() == 404 {
                return Err(ByggisErrors::ProblemNotFound);
            }
            response.text().await.unwrap()
        },
        Err(_) => {
            return Err(ByggisErrors::NetworkError);
        }
    };

    let document = Document::from(html.as_str());
    let tests = get_samples(document.clone());
    let description = get_description(document);

    let dotbyggis = DotByggis {
        tests,
        description,
    };

    if fs::create_dir(&name).is_err() {
        return Err(ByggisErrors::DirectoryNotCreated);
    }

    let file = match fs::File::create(&format!("{}/.byggis", &name)) {
        Ok(n) => n,
        Err(_) => {return Err(ByggisErrors::ByggisFileNotCreated);},
    };

    if let Err(_) = serde_json::to_writer(file, &dotbyggis) {
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

        let contents = lang.get_contents(&name);

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
    let mut hmap: HashMap<String, String> = HashMap::new();

    let c: Vec<Node> = document.find(Name("pre")).collect();

    // This can be fixed, check TODOs in main.rs
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

fn get_description(document: Document) -> Vec<String> {
    let node = document.find(Class("problembody")).next().unwrap();

    // TODO: We need to parse the description here
    println!("Encountered a TODO in creator.rs::get_description");
    
    let x = node.find(Name("p")).map(|x| {
        // remove the dollar signs
        x.text().chars().filter(|x| x != &'$').collect::<String>()

        // convert leq to proper sign
        .replace("\\leq", "<=")

    }).collect::<Vec<String>>();

    x
}
