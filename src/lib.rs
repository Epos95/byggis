pub enum ByggisErrors {
    NetworkError,
        // reflects errors connecting to kattis
    DirectoryNotCreated,
        // reflects error scenario where directory could not be created
    ByggisFileNotCreated, 
        // reflects error scenario where byggis file cannot be created
    ProblemNotFound,
        // reflects 404 from kattis when searching for the problem
    ByggisFileNotFound,
        // scenario where byggis file is not found
    TestsNotFound,
        // scenario where the tests in the file is not found 
    MainNotFound,
        // reflects scenario where main file cant be found in the directory
    CompileTimeError(String),
        // reflects scenario where the users code cant compile
    UnknownLanguage,
        // reflects scenario where the file is in a unimplemented language
    ConfigFileNotFound,
        // scenario where byggis cannot find a config file to read token from
    InvalidToken,
        // scenario where kattis refuses the given token
}


pub struct DotByggis {
    test_inputs : Vec<String>,
    test_outputs : Vec<String>
    // problem_description : Vec<String>
}



