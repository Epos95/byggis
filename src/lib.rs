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
    FileReadingError,
        // scenario where byggis cannot read from the token file
    MainFailure,
        // scenario where byggis cannot create the main file

}

pub use enum_iterator::IntoEnumIterator;
#[derive(Debug, IntoEnumIterator, PartialEq)]
pub enum SupportedLanguages {
    Rust,
    Java,
    Python,
    Haskell,
}
 
impl SupportedLanguages {
    pub fn from_string(s: String) -> Option<Self> {
        match s.as_str() {
            "rust" | "rs"    => Some(SupportedLanguages::Rust),
            "python" | "py"  => Some(SupportedLanguages::Python),
            "java"           => Some(SupportedLanguages::Java),
            "haskell" | "hs" => Some(SupportedLanguages::Haskell),
            _         => None
        }
    }

    pub fn name(&self) -> String {
        match &self {
            SupportedLanguages::Rust    => String::from("rust"),
            SupportedLanguages::Python  => String::from("python"),
            SupportedLanguages::Java    => String::from("java"),
            SupportedLanguages::Haskell => String::from("haskell")
        }
    }

    pub fn extension(&self) -> String {
        match &self {
            SupportedLanguages::Rust    => String::from("rs"),
            SupportedLanguages::Python  => String::from("py"),
            SupportedLanguages::Java    => String::from("java"),
            SupportedLanguages::Haskell => String::from("hs")
        }
    }

    //pub fn get_contents()
}