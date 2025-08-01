// Built a CLI tool that generates the nth Fibonacci number
// Usage (on bash cli run): `cargo run -q 5`

use std::env;
use std::str::FromStr;

fn main() {
    let mut arg_values = Vec::new();

    // grab value from cli arg
    for arg in env::args().skip(1) {
        arg_values.push(u32::from_str(&arg).expect("error parsing arg"));
        break;
    }
    // if no arg given output error
    if arg_values.len() == 0 {
        eprintln!("Usage: fib NUMBER [1 arg]");
        std::process::exit(1);
    }

    let number = arg_values[0];
    println!("{}", fib(number));
}

fn fib(n: u32) -> u32 {
    // debug fib calls
    println!("fib({n}) called...");

    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}
