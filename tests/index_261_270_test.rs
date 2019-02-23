extern crate leetcode;

use leetcode::index_261_270::*;

// 263
#[test]
#[ignore]
fn is_ugly_works() {
    assert_eq!(is_ugly(8), true);
    assert_eq!(is_ugly(14), false);
}

// 268
#[test]
#[ignore]
fn missing_number_works() {
    assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    assert_eq!(missing_number(vec![3, 0, 1]), 2);
}
