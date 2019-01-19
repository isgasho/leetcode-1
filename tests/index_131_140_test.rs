extern crate leetcode;

use leetcode::index_131_140::*;

// 136
#[test]
fn single_number_works() {
    assert_eq!(single_number(vec![2, 2, 1]), 1);
    assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
}
