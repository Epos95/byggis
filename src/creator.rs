use crate::supported_languages::SupportedLanguages;
use byggis::*;
use crossterm::style::*;
use reqwest;
use scraper::{Html, Selector};
use serde_json;
use std::{collections::HashMap, fs, io, io::prelude::*};

/// ## Entry function for creating a new byggis directory.
/// ```
/// pub async fn create_new(name: String) -> Result<String, ByggisErrors>
/// ```
///
/// Creates a new byggis directory with a specified problem id (`name`).
/// Returns either the problem id (`name`) or a `ByggisErrors` telling where it all went horribly wrong.
pub async fn create_new(name: String) -> Result<String, ByggisErrors> {
    let op = get_from_string(&name).await;
    let html = match op {
        Ok(response) => {
            if response.status() == 404 {
                return Err(ByggisErrors::ProblemNotFound);
            }
            response.text().await.unwrap()
        }
        Err(_) => {
            return Err(ByggisErrors::NetworkError);
        }
    };

    let document = Html::parse_document(html.as_str());
    let tests = get_samples(document.clone());
    let description = get_description(document);

    let dotbyggis = DotByggis { tests, description };

    if fs::create_dir(&name).is_err() {
        return Err(ByggisErrors::DirectoryNotCreated);
    }

    let file = match fs::File::create(&format!("{}/.byggis", &name)) {
        Ok(n) => n,
        Err(_) => {
            return Err(ByggisErrors::ByggisFileNotCreated);
        }
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
            println!("    {}: {}", i + 1, lang.name());
        }

        println!("  Choose a language. [n]");
        buffer = String::new();
        stdin.read_line(&mut buffer).expect("IO error.");

        let n = buffer.trim_end().parse().unwrap_or(999) - 1;

        let langs = SupportedLanguages::into_enum_iter().collect::<Vec<SupportedLanguages>>();

        let lang = langs.get(n).unwrap_or(&SupportedLanguages::Python);

        let contents = lang.get_contents(&name);

        let mut file = match fs::File::create(&format!("{}/main.{}", name, lang.extension())) {
            Ok(n) => n,
            Err(_) => {
                return Err(ByggisErrors::MainFailure);
            }
        };

        match file.write_all(contents.as_bytes()) {
            Ok(_) => {}
            Err(_) => {
                return Err(ByggisErrors::MainFailure);
            }
        }
    }

    Ok(name)
}

/// ### Gets the page of a specified kattis problem.
/// ```
/// async fn get_from_string(problem: &String) -> Result<reqwest::Response, reqwest::Error>
/// ```
async fn get_from_string(problem: &String) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!("https://open.kattis.com/problems/{}", problem);
    let res = reqwest::get(&url);

    res.await
}

/// ### Parses a HTML document and gets the sample inputs/outputs.
/// ```
/// fn get_samples(document: Html) -> HashMap<String, String>
/// ```
fn get_samples(document: Html) -> HashMap<String, String> {
    let mut hmap: HashMap<String, String> = HashMap::new();

    // should result in:
    // "tests":{"9\n-13\n":"4\n","10\n6\n":"1\n"}

    let selector = Selector::parse("pre").unwrap();
    let samples = document
        .select(&selector)
        .map(|r| r.inner_html())
        .collect::<Vec<String>>();

    // This can be fixed, check TODOs in main.rs
    if samples.len() % 2 != 0 {
        println!(
            "    {}{}",
            "BIG ERROR".red(),
            ": This kattis problem has a weird problem definition\
                  which caused the webscraping part to shit the bed.\n Do not use \
                  byggis with this problem."
        );
        panic!();
    }

    let mut counter = 1;
    for thing in samples.iter().step_by(2) {
        hmap.insert((*(thing.clone())).to_string(), samples[counter].clone());
        counter += 2;
    }

    hmap
}

/// ### Gets the description of a problem from a specified HTML document.
/// ```
/// fn get_description(document: Html) -> Vec<String>
/// ```
/// This function should prepare a description for parsing by the describer module by interpreting various signs and beautifying the description.
fn get_description(document: Html) -> Vec<String> {
    let s = Selector::parse(r#"div[class="problembody""#).unwrap();
    let mut samples = document.select(&s);
    let s = Selector::parse("p").unwrap();

    samples
        .next()
        .unwrap()
        .select(&s)
        .map(|thing| {
            let re = regex::Regex::new(r"\s+").unwrap();

            let e = thing
                .inner_html()
                .replace("\t", "")
                .replace("\n", "")
                .replace("<span class=\"tex2jax_process\">$", "")
                .replace("$</span>", "");

            re.replace_all(&e, " ")
                .replace("\\leq", "<=")
                .replace("\\not", "!")
                .replace("\\le", "<=")
        })
        .collect()
}
