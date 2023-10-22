# Rust CLI Binary with SQLite
#### By Daniel Medina
![build-release workflow](https://github.com/medinardaniel/rust-cli-sqlite/actions/workflows/rust_build_release.yml/badge.svg)
![lint workflow](https://github.com/medinardaniel/rust-cli-sqlite/actions/workflows/rust_lint.yml/badge.svg)
![test workflow](https://github.com/medinardaniel/rust-cli-sqlite/actions/workflows/rust_test.yml/badge.svg)

## Project Description
In this project, I perform CRUD operations using Rust in a SQLite database HateCrimesDB.

I perform the following queries:
* [C] Create the following new record in the dataset:

Record { state: "North Carolina", agency_type: "City", agency_name: "Durham Police Department", race: "Hispanic", religion: "Atheist", sexual_orientation: "Straight", ethnicity: "Hispanic", disability: "Physical", gender: "Male", gender_identity: "Cisgender", q1: "Yes", q2: "No", q3: "No", q4: "No", population: "" }
* [R] Read the record I just inserted from the dataset
* [U] Update the record field "q1" from "Yes" to "No"
* [D] Delete the created record

Testing:

To run tests, execute "cargo test" command on the terminal.

How to run:

Once in the rust-cli-sqlite directory, execute "cargo run" command on the terminal.

Project Dependencies:
* reqwest = "0.11"
* rusqlite = "0.25.0"

### Instructions: Downloading the Rust Binary
1. Go to the "Actions" tab on the GitHub repo
2. On the left-hand side of the screen, select "Rust Build and Release" under "Actions"
3. Select the most recent workflow run
4. Scroll to the bottom and click on the rust-cli-sqlite-binary to download

### Use of GitHub Copilot
As a brand new Rustacean, I leaned into GitHub copilot to guide me throughout the development process. I prompted GitHub Copitlot autocomplete by writing prompts through commentary. In addition, I used GithHub Copilot Chat to help me write some of the CRUD operation functions in lib.rs and ensure I was able to call them from my main.rs file.

