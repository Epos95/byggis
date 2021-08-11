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
$ byggis new {namn på problem}
```

Manually create a main file to keep the code in.
```bash
$ touch main.{extension}
```
Supported languages:
* Python
* Rust

To test your code from main.* against the test cases from kattis
```bash
$ byggis run
```

## TODO
- [x] Implement rust
- [ ] Description of the problem from Kattis
- [ ] Better help messages
- [ ] Implement C
- [ ] Implement C++

# Contribute
Currently the only supported languages are python and Rust. If your preffered language is not supported, please contact me for more information etc.

# Licens
[MIT](https://choosealicense.com/licenses/mit/)
