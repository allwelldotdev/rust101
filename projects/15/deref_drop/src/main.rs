// Rust does deref coercion when it find types and trait implementations in three places:
// 1. From `&T` to `&U` when `T: Deref<Target = U>`
// 2. From `&mut T` to `&mut U` when `T: DerefMut<Target = U>`
// 3. From `&mut T` to `&U` when `T: Deref<Target = U>`

use deref_drop::{AccessLogger, CustomSmartPointer};

fn main() {
    // Deref trait examples
    let n = AccessLogger(-1);
    let x = *n + 1;
    let _n2 = n;
    println!("{} {}", x, *n);

    println!("---");

    // Drop trait examples
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    let e = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomSmartPointers created!");
    drop(e);
    println!("CustomSmartPointer `some data` dropped before the end of main!");
}
