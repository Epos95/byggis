
pub enum HelpTypes {
    Program,
    New,
}

pub fn show_help(c: HelpTypes) {
    match c {
        HelpTypes::Program => {
            println!("This is the general help for this program");
        },
        HelpTypes::New => {
            println!("Create a new directory for solving kattis problems");
        }
    }
}