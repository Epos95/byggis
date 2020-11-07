
pub enum HelpTypes {
    Program,
    New,
}

pub fn show_help(c: HelpTypes) {
    match c {
        HelpTypes::Program => {
            println!("Byggis\t-   A build system for Kattis\n\nbyggis new (name of problem)\n\tCreate a directory hosting the kattis problems\n\nbyggis run\n\tRun tests\n");
        },
        HelpTypes::New => {
            println!("Create a new directory for solving kattis problems\nSyntax: \n\tbyggis new name");
        }
    }
}