// Listen to TCP Connection
// Building a web server - Single-threaded
#[allow(unused)]
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // println!("Connection established");
        handle_connection(stream);
    }
}

#[allow(unused)]
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // println!("Request: {http_request:#?}");

    // Validating requests, responding selectively
    let request_line = buf_reader
        .lines()
        .next()
        // There's a bug here: when the browser doesn't make a fresh request to the
        // server, the iterator is empty, `next()` returns `None`. `expect()` fires error message:
        // "No first line request"
        .expect("No first line request")
        .expect("Could not read HTTP buffer (first line)");

    // // if http_request[0] == "GET / HTTP/1.1" {
    // if request_line == "GET / HTTP/1.1" {
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents =
    //         fs::read_to_string("hello.html").expect("Could not parse \"hello.html\" file");
    //     let length = contents.len();

    //     // The reason for the double CRLF `\r\n` is one new line after header,
    //     // then second new line to usher in message-body.
    //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //     stream
    //         .write_all(response.as_bytes())
    //         .expect("Could not write response to send buffer");
    // } else {
    //     let status_line = "HTTP/1.1 400 NOT FOUND";
    //     let contents = fs::read_to_string("404.html").expect("Could not parse \"404.html\" file");
    //     let length = contents.len();

    //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //     stream
    //         .write_all(response.as_bytes())
    //         .expect("Could not write response to send buffer");
    // }

    // --------------

    // Refactoring repetitive code
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 400 NOT FOUND", "404.html")
    };

    let contents =
        fs::read_to_string(filename).expect(&format!("Could not parse \"{filename}\" file"));
    let response = format!(
        "{status_line}\r\nContent-Length: {}\r\n\r\n{contents}",
        contents.len()
    );
    stream
        .write_all(response.as_bytes())
        .expect("Could not write response to send buffer");
}
