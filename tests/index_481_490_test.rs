extern crate leetcode;

use leetcode::index_481_490::*;

// 485
#[test]
fn find_max_consecutive_ones_works() {
    assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
}
