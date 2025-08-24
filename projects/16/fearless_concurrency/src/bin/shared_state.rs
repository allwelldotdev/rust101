use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    // // Checking out the Mutex API in a single-threaded env
    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {m:?}"); // Returns: m = Mutex { data: 6, poisoned: false, .. }

    // -----------

    // Sharing Mutex<T> between threads, using Arc<T> instead of Rc<T>.
    // Because Rc<T> doesn't implement the `Send` trait, but Arc<T> does.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });

    println!("Result: {}", *counter.lock().unwrap());
    println!("Result in Debug Mode: {:?}", counter);
}
