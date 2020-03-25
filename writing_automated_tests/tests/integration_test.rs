use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setupt();
    assert_eq!(4, adder::add_two(2));
}