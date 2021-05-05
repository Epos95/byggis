use std::collections::HashMap;
use select::document::Document;
use select::predicate::Name;
use select::node::Node;
use reqwest;
use std::fs;
use serde_json;
use byggis::ByggisErrors;
use termion::*;
use std::io;

pub fn get_supported_languages() -> Vec<String> {
    vec!["rust", "python", "java"]
        .iter()
        .map(|x| x.to_string())
        .collect()
}

pub async fn create_new(name: String) -> Result<String, ByggisErrors> {

    let op = get_as_string(&name).await;
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

    // maybe ask user if they want to create a main file here

    println!("  Create main file? [y/n]");
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("IO errror.");

    if buffer.to_lowercase() == "y" {
        let langs = byggis::get_supported_languages();
        println!("   Supported languages: ");
        for (i, lang) in langs.iter().enumerate() {
            println!("    {}: {}", i+1, lang);
        }

        println!("  Choose a language. [n]");
        buffer = String::new();
        stdin.read_line(&mut buffer).expect("IO errror.");
        let selected = match langs.get(buffer.parse::<usize>().unwrap_or(999)) {
            Some(x) => x,
            None => {
                println!("  {}Error{}: Invalid selection, aborting...",
                        color::Fg(color::Red),
                        color::Fg(color::Reset));
                // TODO: return a error here, untill then we panic!
                panic!();
            }
        };

    }






    Ok(name)
}

async fn get_as_string(problem: &String) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!("https://open.kattis.com/problems/{}", problem);
    let res = reqwest::get(&url);

    res.await
}

fn get_samples(document: Document) -> HashMap<String, String> {
    // you know what: 
    // YES this function will shit the bed if it encounters <pre> tags in any 
    //     place that isnt the input/output spec. If the question does that then
    //     fuck me 


    // you should play around with the select library too learn it a bit better
    // rewrite according to the spec from runner.rs
    let mut hmap: HashMap<String, String> = HashMap::new();
    let c: Vec<Node> = document.find(Name("pre")).collect();


    if c.len() % 2 != 0 {
        println!("    {}BIG ERROR{}: This kattis problem has a weird problem definition\
                  which caused the webscraping part to shit the bed.\n     Do not use \
                  byggis with this problem.",
                 color::Fg(color::Red),
                 color::Fg(color::Reset));
        println!("Oopsie woopsie, we did a fucky wucky. We are working wevy hard to fix this error");
        panic!();
    }

    let mut counter = 1;
    for thing in c.iter().step_by(2) {
        hmap.insert(thing.text(), c[counter].text());
        counter += 2;
    }


    hmap
}
