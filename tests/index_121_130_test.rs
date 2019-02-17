extern crate leetcode;

use leetcode::index_121_130::*;

// 121
#[test]
#[ignore]
fn max_profit_121_works() {
    assert_eq!(max_profit_121(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(max_profit_121(vec![1, 2, 3, 4, 5]), 4);
}

// 122
#[test]
#[ignore]
fn max_profit_122_works() {
    assert_eq!(max_profit_122(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(max_profit_122(vec![1, 2, 3, 4, 5]), 4);
}
