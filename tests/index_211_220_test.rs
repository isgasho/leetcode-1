extern crate leetcode;

use leetcode::index_211_220::*;

// 217
#[test]
#[ignore]
fn contains_duplicate_works() {
    assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
}
