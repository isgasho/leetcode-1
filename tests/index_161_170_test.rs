extern crate leetcode;

use leetcode::index_161_170::*;

// 167
#[test]
#[ignore]
fn two_sum_works() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
}

// 169
#[test]
#[ignore]
fn majority_element_works() {
    assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
}
