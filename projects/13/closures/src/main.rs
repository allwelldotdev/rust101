// Tinkling with closure syntax in Rust

// use std::thread;car

#[allow(unused)] // Can be #![allow(unused)] or #[allow(unused)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let mut list = vec![1, 2, 3];
    // println!("Before defining closure: {list:?}");

    // let mut borrows_mutably = || list.push(7);

    // borrows_mutably();
    // println!("After calling closure: {list:?}");

    // thread::spawn(move || println!("From thread: {list:?}"))
    //     .join()
    //     .unwrap();

    // Looking at sort_by_key std method
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // Pegging a counter placeholder; incremented in `sort_by_key` fn
    // to count how many times closure is called
    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}
