// Creating a reference cycle to illustrate how Rust can allow memory leaks caused by
// the wrong use of `Rc<T>` and `RefCell<T>` smart pointers.

use crate::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

#[allow(unused)]
#[derive(Debug)]
// A kind of Linked List that references itself;
// indirected by the Rc<T> smart pointer.
enum List {
    Cons(i32, RefCell<Rc<Self>>),
    Nil,
}

#[allow(unused)]
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<Self>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // This code creates a list in `a` and a list in `b` that points to the list in `a`.
    // Then it modifies the list in `a` to point to `b`, creating a reference cycle.
    // There are `println!` statements along the way to show what the reference counts are at various points in this process.

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b) // Reference cycle is established here!
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // b rc count = 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // a rc count = 2

    // // Uncomment the next line to see that we have a cycle;
    // // it will overflow the stack. Causing a "stack overflow".
    // println!("a next item = {:?}", a.tail());
}
// <----- StackEnd

// When the part of Rust runtime that manages memory reaches "StackEnd"
// (meaning after the `main` fn stack frame and it's contents have been deallocated),
// after dropping `b` first, then `a` variables along with their Rc<T> pointers
// the data `b` and `a` pointed to in the heap would stil have Rc<T> strong counts of 1 and 1 respectively.
// Meaning, the data will remain in the heap, unattended and abandoned causing a memory leak.
