// Using TCP Connection
// Building a web server - Multi-threaded
use hello::ThreadPool;
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // // To handle every request concurrenly, we could spawn a new thread for each request.
    // // Though, this won't be the lasting solution. It's a backdoor for DoS attacks.
    // // This method eventually overwhelms the system because we'll making new threads
    // // without any limit.
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     thread::spawn(|| {
    //         handle_connection(stream);
    //     });
    // }

    // ----------

    // Creating a finite number of threads: using a thread pool
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    // Grab first line of request message
    let request_line = buf_reader
        .lines()
        .next()
        .expect("No first line request")
        .expect("Could not read http buffer");

    // Simulating a slow response
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            println!("Sleeping for 5 secs");
            thread::sleep(Duration::from_secs(5));
            println!("Finished sleeping");
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents =
        fs::read_to_string(filename).expect(&format!("Could not read \"{}\" file", filename));

    let response = format!(
        "{status_line}\r\nContent-Length: {}\r\n\r\n{contents}",
        contents.len()
    );

    stream
        .write_all(response.as_bytes())
        .expect("Could not write response to send buffer");
}
