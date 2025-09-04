// # Advanced Functions and Closures

// ## Function Pointers
// This technique is useful when you want to pass a function you've already defined
// rather than defining a new closure.

fn add_one(x: i32) -> i32 {
    x + 1
}

// type F = fn(i32) -> i32;
// fn do_twice(f: F, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[allow(unused)]
#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");

    // Using functions or closures as an argument for a parameter that implements
    // the trait bound of either `Fn`, `FnMut`, or `FnOnce`
    let list_of_numbers = vec![1, 2, 3];
    // Using a closure:
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("List of strings: {list_of_strings:?}");
    // OR using a function:
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("List of strings: {list_of_strings:?}");

    // Recall about Enums in Rust (in Chapter 6), that the name of each enum variant that
    // we define also becomes an initializer function.
    // We can use these initializer functions as function pointers that implement the
    // closure traits, which means we can specify the initializer functions as args for
    // methods that take closures.
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("List of statuses: {list_of_statuses:?}");

    // ------------------
    println!("{}", "-".repeat(10));

    // ## Returning Closures
    // Each closure is its own distinct type. Therefore, if you need to work with functions
    // that have the same signature but different implementations, you will need to use a
    // trait object for them.
    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    handlers.iter().enumerate().for_each(|(idx, handler)| {
        let output = handler(5);
        println!("Returning closures {}: {}", idx + 1, output);
    })
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}
