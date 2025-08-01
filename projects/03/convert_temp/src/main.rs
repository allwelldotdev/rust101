// Built a CLI tool that converts temperature from Farenheit to Celsius
// Usage (on bash cli run): `cargo run -q 81`

use std::env;
use std::str::FromStr;

fn main() {
    let mut arg_values = Vec::new();

    // grab value from cli arg
    for arg in env::args().skip(1) {
        arg_values.push(f64::from_str(&arg).expect("error parsing argument"));
        break; // break outta the loop just incase there is more than 1 arg
    }
    // if no arg given output error
    if arg_values.len() == 0 {
        eprintln!("Usage: convert_temp NUMBER");
        std::process::exit(1);
    }

    let temp = arg_values[0];
    let celsius = to_celsius(temp);
    println!("{celsius:.1}C");
}

fn to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}
