use std::collections::HashMap;
use select::document::Document;
use select::predicate::Name;
use select::node::Node;
use reqwest;
use std::fs;
use serde_json;
use byggis::ByggisErrors;

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

    // FIX THIS DONT USE UNWRAP
    let html = op.unwrap().text().await.unwrap();
    let document = Document::from(html.as_str());
    let hmap = get_samples(document);

    if let Err(_) = fs::create_dir(&name) {
        return Err(ByggisErrors::DirectoryNotCreated);
    }

    let file = match fs::File::create(&format!("{}/.byggis", &name)) {
        Ok(n) => n,
        Err(_) => {return Err(ByggisErrors::ByggisFileNotCreated);},
    };

    if let Err(_) = serde_json::to_writer(file, &hmap) {
        return Err(ByggisErrors::ByggisFileNotCreated);
    }

    Ok(name)
}

async fn get_as_string(problem: &String) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!("https://open.kattis.com/problems/{}", problem);
    let res = reqwest::get(&url);

    res.await
}

fn get_samples(document: Document) -> HashMap<String, String> {
    let mut hmap: HashMap<String, String> = HashMap::new();

    let c: Vec<Node> = document.find(Name("pre")).collect();

    let mut counter = 1;
    for thing in c.iter().step_by(2) {
        hmap.insert(thing.text(), c[counter].text());
        counter += 2;
    }

    hmap
}