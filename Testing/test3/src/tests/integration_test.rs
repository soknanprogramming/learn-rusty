use test3;
mod common;

#[test]
pub fn it_add_two(){
    common::setup();
    assert_eq!(4, test3::add(2, 2))
}
