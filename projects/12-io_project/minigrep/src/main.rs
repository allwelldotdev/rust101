use minigrep::Config;
use std::{env, process};

fn main() {
    // Read cli arguments ...
    let args = env::args().skip(1).collect::<Vec<_>>();
    // dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Read file content from filesystem ...
    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
