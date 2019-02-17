extern crate leetcode;

use leetcode::index_281_290::*;

// 283
#[test]
#[ignore]
fn move_zeroes_works() {
    assert_eq!(move_zeroes(&mut vec![0, 1, 0, 3, 12]), vec![1, 3, 12, 0, 0]);
}
