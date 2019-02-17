extern crate leetcode;

use leetcode::index_261_270::*;

// 268
#[test]
#[ignore]
fn missing_number_works() {
    assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    assert_eq!(missing_number(vec![3, 0, 1]), 2);
}
