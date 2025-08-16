// Propagating Errors
// Instead of handling an error within a fn you can return the error to the calling code

use std::fs::File;
use std::io::{self, Read};

// Propagating Errors using `match`
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// Propagating Errors using `?` operator
fn read_username_from_file() -> Result<String, io::Error> {
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // Shortening the code:
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    // Shortening the code further:
    // fs::read_to_string("hello.txt")
}

fn main() {
    let r = read_username_from_file();
    println!("{:?}", r);
}
