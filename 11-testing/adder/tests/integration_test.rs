use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    
    assert_eq!(4, adder::add_two(2));
    assert_eq!(2, adder::add_two(0));
    assert_eq!(8900, adder::add_two(8898));
}
