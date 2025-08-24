#[allow(unused)]
use std::{sync::mpsc, thread, time::Duration};
#[allow(unused)]
use strum::{EnumIter, IntoEnumIterator};

#[allow(unused)]
#[derive(EnumIter)]
enum ClientMessage {
    Incr,
    Get,
    Quit,
}
enum ServerMessage {
    Get(usize),
}

fn main() {
    // // Sending a message from the spawned thread to a main thread via channels.
    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || {
    //     let val = String::from("hi!");
    //     tx.send(val).unwrap();
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {received}");

    // --------

    // // Sending (transmitting) multiple values and seeing the receiver waiting.
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     vals.into_iter().for_each(|s| {
    //         tx.send(s).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     });
    // });

    // // Here, the main thread waits to receive values from the spawned thread.
    // rx.iter().for_each(|received| {
    //     println!("Got: {received}");
    // });

    // -----------

    // // Sending message with multiple transmitters to one receiver.
    // let tx1 = tx.clone();
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     vals.into_iter().for_each(|s| {
    //         tx1.send(s).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     });
    // });

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("messages"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];

    //     vals.into_iter().for_each(|s| {
    //         tx.send(s).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     });
    // });

    // rx.iter().for_each(|r| {
    //     println!("Got: {r}");
    // });

    // ------------------

    // Another cool example of concurrent message passing using a client/server concept.
    let (server_tx, client_rx) = mpsc::channel();
    let (client_tx, server_rx) = mpsc::channel();

    // Spin up the "server" thread
    let server = thread::spawn(move || {
        let mut n = 0;
        loop {
            match server_rx.recv().unwrap() {
                ClientMessage::Quit => break,
                ClientMessage::Incr => n += 1,
                ClientMessage::Get => server_tx.send(ServerMessage::Get(n)).unwrap(),
            }
        }
    });

    // Iterated through enum using 3rd party crate called "strum"
    // Send message thru threads to server
    ClientMessage::iter().for_each(|msg| {
        client_tx.send(msg).unwrap();
    });

    let ServerMessage::Get(n) = client_rx.recv().unwrap();
    println!("n = {}", n);

    server.join().unwrap();
}
