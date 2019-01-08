extern crate leetcode;

use leetcode::index1_10::*;

#[test]
fn two_sum_works() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![2, 5, 5, 11], 10), vec![1, 2]);
}
