use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // generate secret random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // take user input: guess number
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert `guess` from String type to u32 - this uses a concept in Rust known as Shadowing
        // let guess: u32 = guess.trim().parse().expect("Please type a number");

        // instead of crashing program when `guess` is non-number, continue
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // compare guess value with secret random number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
