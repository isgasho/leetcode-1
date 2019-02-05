extern crate leetcode;

use leetcode::index_121_130::*;

// 122
#[test]
fn max_profit_works() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
}
