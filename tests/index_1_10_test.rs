extern crate leetcode;

use leetcode::index_1_10::*;

// 1
#[test]
#[ignore]
fn two_sum_works() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![2, 5, 5, 11], 10), vec![1, 2]);
}

// 9
#[test]
#[ignore]
fn is_palindrome_works() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(10), false);
    assert_eq!(is_palindrome(0), true);
}
