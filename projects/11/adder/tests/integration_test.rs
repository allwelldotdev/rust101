// use adder::add_three;

mod common;

#[test]
fn test_add_three() {
    common::setup();

    let result = adder::add_three(3);
    assert_eq!(result, 6);
}
