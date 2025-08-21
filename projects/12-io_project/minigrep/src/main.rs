use minigrep::Config;
use std::{env, process};

fn main() {
    // Read cli arguments as an iterator
    // instead of collecting into `Vec<String>` ...

    let config = Config::build(env::args().skip(1)).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Read file content from filesystem ...
    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
