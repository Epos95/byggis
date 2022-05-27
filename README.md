# Build tool for Kattis

Byggis is a tool to make solving [kattis](https://open.kattis.com) problems easier through your terminal and a proper testing system.

## Installation
Build from scratch via GitHub or download with cargo. [(how to install cargo)](https://doc.rust-lang.org/cargo/getting-started/installation.html)
```bash
$ cargo install byggis
```

## Usage 
### Start a new kattis solution: 
```bash
$ byggis new <PROBLEM ID> 
```

In the kattis url: `https://open.kattis.com/problems/aa` the problem id will be `aa` so the byggis command will be 
```bash
$ byggis new aa
```

Byggis will then prompt if you want to create a main file with a starter code snippet in it.

Supported languages:
* Python  (first class support)
* Rust    (almost first class support)
* Java    (second class support)
* Haskell (not even sure if it works tbh)

Byggis will then create a new folder named `<PROBLEM ID>` where your main file exists.

### To test your code against the test cases from kattis just do:
```bash
$ byggis run
```
if there are multiple main files, byggis will ask you which one to use.

### To view your problems description with byggis use:
```bash
$ byggis describe
```
This will print the problems description in the terminal and reduce the need to alt tab.

## TODO before 1.0 release
- [ ] File uploading
	- [ ] Proper login to kattis
	- [ ] send request with file
	- [ ] read response html doc to get the results from kattis
- [ ] rewrite webscraping things to use [scraper]("https://docs.rs/scraper/latest/scraper/")
	- [ ] rewrite [creator.rs](src/creator.rs)
	- [ ] implement response reading for fileuploading with scraper
- [ ] Rewrite [ByggisErrors](src/lib.rs) to use proper error enum functionalites from "rust for rustaceans.
- [x] Update the clap version


## TODO
- [x] Implement rust
- [x] Better help messages
- [x] Description of the problem from Kattis
- [ ] Implement C
- [ ] Implement C++
- [ ] Implement submissions
- [ ] Write tests

## Contribute
If your desired language is not supported, please put in a pull request.
Implementing a new language should be easy and only requires editing the [supported_languages.rs]("https://github.com/Epos95/byggis/blob/master/src/supported_languages.rs") file and should require minimal rust knowledge.

## license
[MIT](https://choosealicense.com/licenses/mit/)
