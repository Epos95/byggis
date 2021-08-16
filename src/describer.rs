use std::{
    fs,
};

use serde_json;
use crossterm::style::*;

use byggis::{
    DotByggis,
    ByggisErrors,
};

pub fn describe() -> Result<(), ByggisErrors> {
    // get .byggis file to read tests from
    let dot_byggis = match fs::File::open("./.byggis") {
        Ok(n) => n,
        Err(_) => { return Err(ByggisErrors::ByggisFileNotFound); },
    };

    let dot_byggis: DotByggis = match serde_json::from_reader(dot_byggis) {
        Ok(n) => n,
        Err(_) => { return Err(ByggisErrors::TestsNotFound); },
    };
    
    println!(" Description:");
    for line in dot_byggis.description {
        println!("  {}\n", line);
    }

    Ok(())
}