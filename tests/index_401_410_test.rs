extern crate leetcode;

use leetcode::index_401_410::*;

// 405
#[test]
fn sum_of_left_leaves_works() {
    assert_eq!(sum_of_left_leaves(None), 0);
}

// 409
#[test]
fn longest_palindrome_works() {
    assert_eq!(longest_palindrome(String::from("abccccdd")), 7);
}
