fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn main() {
    println!("Hello, world!");

    let result = gcd(48, 18); // gcd has to come before function call. Because in Rust it's not hoisted to the top of the module like in JavaScript.
    println!("{}", result);
}

// in Rust, you can run tests together within your source file
// and call `cargo test` in the terminal to run all tests. Cool!
#[test]
fn test_gcd() {
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
