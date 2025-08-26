#[allow(unused)]
use trpl::{ReceiverStream, Stream, StreamExt, run};

#[allow(unused)]
use std::{pin::pin, time::Duration};

fn main() {
    trpl::run(async {
        // // Calling the `next()` method on Streams by bringing the `tokio_stream::StreamExt` trait into scope
        // let values = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        // let iter = values.iter().map(|n| n * 2);
        // let mut stream = trpl::stream_from_iter(iter);

        // while let Some(value) = stream.next().await {
        //     println!("The value was: {value}");
        // }

        // -----------

        // // Using other methods provided by the `StreamExt` trait: the `filter()` method
        // let values = 1..100;
        // let iter = values.map(|n| n * 2);
        // let stream = trpl::stream_from_iter(iter);

        // let mut filtered = stream.filter(|n| n % 3 == 0 || n % 5 == 0);

        // while let Some(value) = filtered.next().await {
        //     println!("The value was: {value}");
        // }

        // -----------

        // // Composing Streams: building a little stream of messages as a stand-in for a stream of
        // // data we might see from a WebSocket or another real-time communication protocol
        // let mut messages = get_messages();

        // while let Some(message) = messages.next().await {
        //     println!("{message}");
        // }

        // ---------

        // // Adding a timeout that applies to every item in the stream:
        // let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        // while let Some(result) = messages.next().await {
        //     match result {
        //         Ok(message) => println!("{message}"),
        //         Err(reason) => eprintln!("Problem: {reason:?}"),
        //     }
        // }

        // -----------

        // Merging Streams
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100)) // Slows polling to throttle rate
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(item) => println!("{item}"),
                Err(reason) => println!("{reason:?}"),
            }
        }
    });
}

#[allow(unused)]
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    // messages.iter().for_each(|a| {
    //     tx.send(format!("Message: '{a}'")).unwrap();
    // });

    // Creating a variable delay to the messages we send:
    trpl::spawn_task(async move {
        for (index, message) in messages.iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            // tx.send(format!("Message: '{message}'")).unwrap();
            // Instead of calling `unwrap()`, let's handle possible errors:
            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

#[allow(unused)]
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    // Because this is all wrapped in the task created by `trpl::spawn_task()`,
    // all of it--including the infinite loop--will get cleaned up along with the runtime.
    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            // tx.send(count).unwrap();
            // Handle possible send errors:
            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval '{count}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}
