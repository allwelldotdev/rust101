#[allow(unused)]
use std::{
    pin::{Pin, pin},
    time::{Duration, Instant},
};

#[allow(unused)]
use trpl::{Either, Html};

/// To Rust, the page_title `async fn` can be rewritten as a non-async fn with the body of an async block:
///
/// ```
/// fn page_title(url: &str) -> impl Future<Output = Option<String>> {
///     async move {
///         let response_text = trpl::get(url).await.text().await;
///         Html::parse(&response_text)
///             .select_first("title")
///             .map(|title_element| title_element.inner_html())
///     }
/// }
/// ```

#[allow(unused)]
async fn page_title(url: &str) -> (&str, Option<String>) {
    // let response = trpl::get(url).await;
    // let response_text = response.text().await;
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}

fn main() {
    // // Build a web scrapper that takes in a CLI argument of two website URLs,
    // // runs them asynchronously, and returns the url and title of the website that returns first.
    // let args: Vec<_> = std::env::args().skip(1).collect();

    // trpl::run(async {
    //     let title_fut_1 = page_title(&args[0]);
    //     let title_fut_2 = page_title(&args[1]);

    //     let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
    //         Either::Left(left) => left,
    //         Either::Right(right) => right,
    //     };

    //     println!("{url} returned first!");
    //     match maybe_title {
    //         Some(title) => println!("It's page title is: '{}'", title.trim()),
    //         None => println!("It's title could not be parsed."),
    //     }
    // })

    // -----------------

    // Just like with counting up on two separate threads but with async tasks
    trpl::run(async {
        // // Via spawning an async task + running a loop in root scope,
        // // then awaiting the `JoinHandle` return value of the spawned task to
        // // run the spawned task loop to completion.
        // let handle = trpl::spawn_task(async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // });

        // for i in 1..5 {
        //     println!("hi number {i} from second task!");
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }

        // // Similar to await an async task in Python, if the spawned task is not awaited
        // // it will not be forced to end abruptly when the main ends
        // handle.await.unwrap();

        // -------------

        // // Using `futures::join()` function to run two operations to completion:
        // let fut_1 = async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // };

        // let fut_2 = async {
        //     for i in 1..5 {
        //         println!("hi number {i} from second task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // };

        // // Returns Future results sequentially (in fairness), ultimately depends on
        // // if the async runtime guarantees fairness.
        // trpl::join(fut_1, fut_2).await;

        // ------------------

        // Counting up on two tasks using Message Passing (just like we did using threads)
        // let (tx, mut rx) = trpl::channel();

        // let val = String::from("hi");
        // tx.send(val).unwrap();

        // let received = rx.recv().await.unwrap();
        // println!("Got: {received}");

        // -------------

        // // Sending and receiving multiple messages over the async channel and sleeping with an await between each message:
        // let (tx, mut rx) = trpl::channel();

        // let vals = vec![
        //     String::from("hi"),
        //     String::from("from"),
        //     String::from("the"),
        //     String::from("future"),
        // ];

        // // I would've used `into_iter()` iterator here but it does not return an async function.
        // // I'm guessing there'll async equivalents in the `tokio` crate.
        // for val in vals {
        //     // let val_2 = val.clone();
        //     tx.send(val).unwrap();
        //     trpl::sleep(Duration::from_millis(500)).await;
        //     // println!("Value before async mpsc: {val_2}");
        // }

        // // The messages do not arrive at half-second intervals. Instead, they arrive all at once,
        // // 2 seconds (2,000 milliseconds) after we start the program. We'll solve this in the next iteration.
        // while let Some(received) = rx.recv().await {
        //     println!("Got: {received}");
        // }

        // -----------------

        // // Using `futures::join()` to run both futures so message can be received and printed asynchronously:
        // let (tx, mut rx) = trpl::channel();

        // // By adding `move` to the async block, I transfer ownership of `tx` so once the async block
        // // returns `tx` will be dropped and the mpsc channel will be closed, causing `rx` to return `None`
        // // to stop the `while` loop in the `fut_rx` future, thereby returning the `futures::join()` future.
        // let fut_tx = async move {
        //     let vals = vec![
        //         String::from("hi"),
        //         String::from("from"),
        //         String::from("the"),
        //         String::from("future"),
        //     ];

        //     for val in vals {
        //         tx.send(val).unwrap();
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // };

        // let fut_rx = async {
        //     // `rx.recv().await` will return `None` only once the other end of the channel is closed.
        //     // The channel will close only if we call `rx.close` or when the sender side, `tx`, is dropped.
        //     while let Some(received) = rx.recv().await {
        //         println!("Got: {received}");
        //     }
        // };

        // trpl::join(fut_tx, fut_rx).await;

        // --------------------------

        // // Using multiple producers with async blocks
        // let (tx, mut rx) = trpl::channel();

        // let tx1 = tx.clone();
        // let tx1_fut = async move {
        //     let vals = vec![
        //         String::from("hi"),
        //         String::from("from"),
        //         String::from("the"),
        //         String::from("future"),
        //     ];

        //     for val in vals {
        //         tx1.send(val).unwrap();
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // };

        // let rx_fut = async {
        //     while let Some(value) = rx.recv().await {
        //         println!("received: '{value}'");
        //     }
        // };

        // let tx_fut = async move {
        //     let vals = vec![
        //         String::from("more"),
        //         String::from("messages"),
        //         String::from("for"),
        //         String::from("you"),
        //     ];

        //     for val in vals {
        //         tx.send(val).unwrap();
        //         trpl::sleep(Duration::from_millis(1500)).await;
        //     }
        // };

        // // The key is in the order the futures are awaited, not created.
        // trpl::join3(tx1_fut, tx_fut, rx_fut).await;

        // --------------

        // // Using multiple producers with async blocks; with the `futures::join_all()` function
        // // Uncovering the `Unpin` trait and using dynamic dispatching with `Vec<Pin<&mut dyn Future<Output = ()>>>`
        // let (tx, mut rx) = trpl::channel();

        // let tx1 = tx.clone();
        // let tx1_fut = pin!(async move {
        //     let vals = vec![
        //         String::from("hi"),
        //         String::from("from"),
        //         String::from("the"),
        //         String::from("future"),
        //     ];

        //     for val in vals {
        //         tx1.send(val).unwrap();
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // });

        // let rx_fut = pin!(async {
        //     while let Some(value) = rx.recv().await {
        //         println!("received: '{value}'");
        //     }
        // });

        // let tx_fut = pin!(async move {
        //     let vals = vec![
        //         String::from("more"),
        //         String::from("messages"),
        //         String::from("for"),
        //         String::from("you"),
        //     ];

        //     for val in vals {
        //         tx.send(val).unwrap();
        //         trpl::sleep(Duration::from_millis(1500)).await;
        //     }
        // });

        // // No need for `Box::pin()` because we don't need to store futures in the heap because we don't need them elsewhere outside this scope.
        // // Therefore, better to use `pin!()` macro.
        // // let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
        // //     vec![Box::pin(tx1_fut), Box::pin(tx_fut), Box::pin(rx_fut)];
        // // How to enunciate `Vec<Pin<&mut dyn Future<Output = ()>>>`?
        // // Say, `futures` is a `Vec` containing pinned mutable references to the dynamic future type, whose output is a unit type.
        // let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, tx_fut, rx_fut];

        // // Difference between `futures::join_all()` and other `join` methods and `join!()` macro:
        // // `futures::join_all()`: accepts a vector (growable collection) of futures + futures with similar return types. Has some quirks, uses `Unpin` trait.
        // // Others: accept a particular (determined, ungrowable) number of futures + futures with different return types.
        // trpl::join_all(futures).await;

        // ----------------

        // // Yielding back control to the runtime.
        // // The code below shows that async can be useful even for compute-bound tasks.
        // // A form of cooperative multitasking, where each future has the power to determine when it hands over control via await points.
        // // Each future therefore also has the responsibility to avoid blocking for too long!
        // // In some Rust-based embedded operating systems, this is the only kind of multitasking!
        // let one_ns = Duration::from_nanos(1);
        // let start = Instant::now();
        // async {
        //     for _ in 1..1000 {
        //         trpl::sleep(one_ns).await;
        //     }
        // }
        // .await;
        // let time = Instant::now() - start;
        // println!(
        //     "'sleep' version finished after {} seconds.",
        //     time.as_secs_f32()
        // );

        // let start = Instant::now();
        // async {
        //     for _ in 1..1000 {
        //         trpl::yield_now().await;
        //     }
        // }
        // .await;
        // let time = Instant::now() - start;
        // println!(
        //     "'yield' version finished after {} seconds.",
        //     time.as_secs_f32()
        // );

        // ------------

        // Building our own abstractions
        // building a timeout function with the async building blocks we've already implemented
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished!"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => println!("Timeout after {} seconds", duration.as_secs()),
        }
    })
}

// Building our own abstractions
// building a timeout function with the async building blocks we've already implemented
#[allow(unused)]
async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
