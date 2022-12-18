use std::fs::read_to_string;

use rayon::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "my-grep")]
struct GrepArgs {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "FILE")]
    path: Vec<String>,
}

fn main() {
    run(GrepArgs::from_args());
}

fn grep(state: &GrepArgs, content: String, filename: &str) {
    for line in content.lines() {
        if line.contains(state.pattern.as_str()) {
            println!("{}: {}", filename, line);
        }
    }
}

fn run(state: GrepArgs) {
    state
        .path
        .par_iter()
        .for_each(|file| match read_to_string(file) {
            Ok(content) => grep(&state, content, &file),
            Err(reason) => println!("{}", reason),
        })
}
