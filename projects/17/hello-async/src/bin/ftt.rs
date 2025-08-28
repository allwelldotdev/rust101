// Combining the use of (Async) Futures, Tasks, and Threads.
// They can work together: the choice isn’t threads or async but rather threads and async.

use std::{thread, time::Duration};

fn main() {
    // Create an async channel
    let (tx, mut rx) = trpl::channel();

    // Spawn parallel thread
    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Start async runtime with async channel Receiver
    trpl::run(async {
        while let Some(count) = rx.recv().await {
            println!("Count: {count}");
        }
    });
}
