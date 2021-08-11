# Build tool for Kattis

Byggis is a tool to make solving [kattis](https://open.kattis.com) problems easier through your terminal and a proper testing system.

## Installation
Build from scratch via GitHub or download with cargo.
```bash
$ cargo install byggis
```

## Usage 
Create a new folder for your problem and download the test cases from Kattis.
```bash
$ byggis new <PROBLEM ID>
```

Byggis will then prompt you if you want to create a main file with a starter code snippet in it.

Supported languages:
* Python
* Rust
* Java
* Haskell

To test your code against the test cases from kattis just do:
```bash
$ byggis run
```
if there are multiple main files, byggis will ask you which one to use.

## TODO
- [x] Implement rust
- [x] Better help messages
- [ ] Description of the problem from Kattis
- [ ] Implement C
- [ ] Implement C++
- [ ] Implement submissions
- [ ] Write tests

## Contribute
Currently the only supported languages are python and Rust. If your prefered language is not supported, please put in a pull request.

## license
[MIT](https://choosealicense.com/licenses/mit/)
