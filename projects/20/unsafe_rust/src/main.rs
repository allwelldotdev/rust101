//! You can take five actions in unsafe Rust that you can't in safe Rust.
//! These are otherwise known as _unsafe superpowers_. These include:
//!
//! - Dereference a raw pointer
//! - Call an unsafe function or method
//! - Access or modify a mutable static variable
//! - Implement an unsafe trait
//! - Access fields of a union.

fn main() {
    // # Dereferencing a Raw Pointer
    //
    // Different from references and smart pointers, raw pointers:
    //
    // - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers
    // or multiple mutable pointers to the same location.
    // - Aren't guaranteed to point to valid memory.
    // - Are allowed to be null.
    // - Don't implement an automatic cleanup.
    let mut num = 5;

    // We can create raw pointers in safe code (using the raw borrow operators `&raw ...`), but
    // we can't dereference raw pointers outside of an unsafe block.
    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    // Creating a raw pointer whose validity is not certain, using `as` to
    // cast a value instead of using the raw borrow operators.
    let address = 0x012345usize;
    let _r = address as *const i32;

    // ------------

    // # Calling an Unsafe Function or Method
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }

    // Wrapping unsafe code in a safe function - a common abstraction.
    // For example, the `split_at_mut` funtion from the std library requires unsafe code.
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // // Let's try to implement `split_at_mut` using safe Rust - which won't compile.

    // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = values.len();
    //     assert!(mid <= len);
    //     (&mut values[mid..], &mut values[..mid])
    // }

    // Instead, lets use an `unsafe` block, a raw pointer, and some calls to unsafe functions
    // to make the implementation of `split_at_mut` work.
    use std::slice;

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        // Here we create a safe abstraction over unsafe code.
        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]); // Cool!

    // Using `extern` Functions to Call External Code via FFI

    // unsafe {
    //     println!("Absolute value of -3 according to C: {}", abs(-3));
    // }

    println!("Absolute value of -3 according to C (safe): {}", abs(-3));

    // Calling Rust functions from other languages
    // We add `#[unsafe(no_mangle)]` to tell the Rust compiler not to mangle the name of the function.
    #[unsafe(no_mangle)]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C");
    }

    // --------------

    // # Accessing or Modifying a Mutable Static Variable

    println!("`static` Variable is: {HELLO_WORLD}");

    // Subtle differences between constants and immutable static variables are:
    //
    // - Static variables have a fixed address in memory. Using the value will always access the same data.
    // Constants, on the other hand, are allowed to duplicate their data whenever they're used.
    // - Static variables can be mutable.
    // - Accessing and modifying mutable static variables is "unsafe".

    // Let's see how to declare, access, and modify a mutable static variable named `COUNTER`:
    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }

    // With mutable data that is globally accessible, it's difficult to ensure there are no data races,
    // which is why Rust considers mutable static variables to be unsafe.
    // Where possible, it's preferrable to use the concurrency techniques and thread-safe smart pointers
    // we discussed in Chapter 16 so the compiler checks that data access from different threads
    // is done safely.

    // --------------

    // # Implementing an Unsafe Trait
    // A trait is unsafe when at least one of its methods has some invariant that the compiler can't verify.
    #[allow(unused)]
    unsafe trait Foo {
        // methods go here
    }

    /// By using `unsafe impl`, we're promising that we'll uphold the invariants that the compiler can't verify.
    unsafe impl Foo for i32 {
        // method implementations go here
    }

    // Recall `Send` and `Sync` marker traits discussed in Chapter 16: the compiler
    // implements these traits automatically if our types are composed entirely of other types
    // that implement `Send` or `Sync`.
    //
    // If we implement a type that contains a type that does not implement `Send` or `Sync`,
    // such as raw pointers, and we want to mark that type as `Send` or `Sync`, we must
    // use `unsafe`.
    //
    // Rust cannot verify that our type upholds the guarantees that it can be safely
    // sent across multiple threads or accessed from multiple threads; therefore
    // we need to do those checks manually and indicate as such with `unsafe`.

    // --------------

    // Using Miri to check unsafe code
    //
    // When writing unsafe code, you might want to check that what you have written
    // actually is safe and correct. One of the best ways to do that is to use Miri,
    // an official Rust tool for detecting undefined behavior.
    //
    // Whereas the borrow checker is a static tool that works at compile time,
    // Miri is a dynamic tool that works at runtime.
    //
    // It checks your code by running your program, or its test suite, and detecting
    // when you violate the rules it understands about how Rust should work.
    //
    // Check Miri out on github: https://github.com/rust-lang/miri
    // Using Miri requires a nightly build of Rust.
}

// Using `extern` Functions to Call External Code
// The "C" part defines which ABI (application binary interface) the external function uses:
// the ABI defines how to call the function at the assembly level.
// The "C" ABI is the most common and follows the C programming language's ABI.
#[allow(unused)]
unsafe extern "C" {
    // fn abs(input: i32) -> i32;

    // Using the `safe` keyword to say that this specific function
    // is safe to call even though it is in an unsafe extern block
    safe fn abs(input: i32) -> i32;
}

// `static` variables in Rust
#[allow(unused)]
static HELLO_WORLD: &str = "Hello, world";

#[allow(unused)]
static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined behaviour,
/// so you *must* guarantee you only call it from a single thread at a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
