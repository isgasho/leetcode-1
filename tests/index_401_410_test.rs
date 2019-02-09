extern crate leetcode;

use leetcode::index_401_410::*;

// 401
#[test]
fn read_binary_watch_works() {
    assert_eq!(
        read_binary_watch(1),
        vec!["0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"]
    );
}

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
