//! To implement `Weak<T>` smart pointers, we'll create a tree data structure: a node with child nodes.

//! ```
//! use std::{cell::RefCell, rc::Rc};
//!
//! #[derive(Debug)]
//! struct Node {
//!     value: i32,
//!     children: RefCell<Vec<Rc<Self>>>,
//! }
//!
//! fn main() {
//!     let leaf = Rc::new(Node {
//!         value: 3,
//!         children: RefCell::new(vec![]),
//!     });
//!
//!     let branch = Rc::new(Node {
//!         value: 5,
//!         children: RefCell::new(vec![Rc::clone(&leaf)]),
//!     });
//! }
//! ```

//! We can get from `branch` to `leaf` but can't get from `leaf` to `branch`
//! because `leaf` does not have any relationship or link to branch. We will create that link using a `Weak<T>` pointer.
//! Using a `Rc<T>` will create a reference cycle from `leaf` to `branch` and vice versa. We avoid that by
//! adding a `Node::parent` propery with a `Weak<T>` pointer value.

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[allow(unused)]
#[derive(Debug)]
struct Node {
    value: i32,
    kind: NodeKind,
    parent: RefCell<Weak<Self>>,
    children: RefCell<Vec<Rc<Self>>>,
}

impl Node {
    fn new(value: i32, kind: NodeKind, child: Option<Rc<Self>>) -> Self {
        let children = RefCell::new(Vec::new());
        if let Some(child) = child {
            children.borrow_mut().push(child);
        }

        Self {
            value,
            kind,
            parent: RefCell::new(Weak::new()),
            children,
        }
    }

    fn set_parent(&self, parent: &Rc<Self>) {
        *self.parent.borrow_mut() = Rc::downgrade(parent);
    }
}

#[derive(Debug)]
enum NodeKind {
    Leaf,
    Branch,
}

// #[allow(unused)]
fn main() {
    let leaf = Rc::new(Node::new(3, NodeKind::Leaf, None));

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node::new(5, NodeKind::Branch, Some(Rc::clone(&leaf))));

        // *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        leaf.set_parent(&branch); // Similar to the statement above

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
