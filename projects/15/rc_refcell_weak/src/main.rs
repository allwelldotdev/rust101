// `Rc<T>` examples...
// Using a cons (short for construct function) list to implement a kind of Linked List; a way to create Linked Lists in the Lisp Programming Language.

#[allow(unused)]
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused)]
fn main() {
    // // `Rc<T>` examples...
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating 'a' = {}", Rc::strong_count(&a));

    // let b = Cons(3, Rc::clone(&a));
    // println!("count after creating 'b' = {}", Rc::strong_count(&a));

    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("count after creating 'c' = {}", Rc::strong_count(&a));
    // }

    // println!(
    //     "count after 'c' goes out of scope = {}",
    //     Rc::strong_count(&a)
    // );

    // `RefCell<T>` examples...
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    // Checking mutated multiple-owned `RefCell { value = 5 }` formerly should now be `RefCell { value = 15 }`
    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

    // Checking Reference-counted strong count of `value` and `a`
    println!("`value` strong count = {}", Rc::strong_count(&value));
    println!("`a` strong count = {}", Rc::strong_count(&a));
}
