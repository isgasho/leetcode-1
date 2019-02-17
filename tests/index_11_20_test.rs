extern crate leetcode;

use leetcode::index_11_20::*;

// 13
#[test]
#[ignore]
fn roman_to_int_works() {
    assert_eq!(roman_to_int(String::from("IV")), 4);
    assert_eq!(roman_to_int(String::from("III")), 3);
}
