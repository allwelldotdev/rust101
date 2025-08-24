#[allow(unused)]
use std::{thread, time::Duration};

fn main() {
    // // Both the main and spawned thread run in parallel.
    // // The spawned thread is not joined, therefore it is orphaned from the main thread and doesn't finish printing out.
    // thread::spawn(|| {
    //     for i in 0..10 {
    //         println!("hi, number {i} from the spawned thread!");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 0..5 {
    //     println!("hi, number {i} from the main thread!");
    //     thread::sleep(Duration::from_millis(1));
    // }

    // ---------

    // // Now, spawned thread is joined to main thread.
    // // Causing the main thread to wait for the spawned thread to finish before returning.
    // let handle = thread::spawn(|| {
    //     for i in 0..10 {
    //         println!("hi, number {i} from the spawned thread!");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 0..5 {
    //     println!("hi, number {i} from the main thread!");
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap();

    // ---------

    // // Using `move` closures with threads.
    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("Here's a vector {v:?}");
    // });

    // // Meaning, I cannot use `v` in the main thread anymore. The code below will be rejected by the borrow checker.
    // // println!("Former v = {v:?}");

    // handle.join().unwrap();

    // ---------

    // If I wanted to pass a (borrow) shared reference of `v` to the spawned thread instead,
    // I can only do it within a scoped thread. See next implementation:
    let mut v = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|s| {
        s.spawn(|| {
            println!("Hello from the first scoped thread!");
            // We can borrow `v` here:
            println!("Returns a borrow of vector {:?}", &v);
        });
        s.spawn(|| {
            println!("Hello from the second scoped thread!");
            // We can even mutably borrow `x` here
            // because no other threads are using it
            x += v[0] + v[2];
        });
        println!("Hello from the main thread!");
    });

    // After the scope, we can modify and access our variables again:
    v.push(4);
    assert_eq!(x, v.len());
}
