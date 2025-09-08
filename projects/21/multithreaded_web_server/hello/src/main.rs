// Listen to TCP Connection
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
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
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").expect("Could not parse html file");
    let length = contents.len();

    // The reason for the double CRLF `\r\n` is one new line after header,
    // then second new line to usher in message body.
    let response = format!("{status_line}\r\nContext-Length: {length}\r\n\r\n{contents}");
    stream
        .write_all(response.as_bytes())
        .expect("Could not write response to send buffer");
}
