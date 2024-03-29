use serde_json;
use std::fs;

use byggis::{ByggisErrors, DotByggis};

pub fn describe() -> Result<(), ByggisErrors> {
    // get .byggis file to read tests from
    let raw_dot_byggis = match fs::File::open("./.byggis") {
        Ok(n) => n,
        Err(_) => {
            return Err(ByggisErrors::ByggisFileNotFound);
        }
    };

    let dot_byggis_content: DotByggis = match serde_json::from_reader(raw_dot_byggis) {
        Ok(n) => n,
        Err(_) => {
            return Err(ByggisErrors::TestsNotFound);
        }
    };

    println!(" Problem description:");
    for line in dot_byggis_content.description {
        println!("  {}\n", line);
    }

    Ok(())
}
