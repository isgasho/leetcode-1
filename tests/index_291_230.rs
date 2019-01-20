extern crate leetcode;

use leetcode::index_291_300::*;

// 292
#[test]
fn can_win_nim_works() {
    assert_eq!(can_win_nim(4), false);
    assert_eq!(can_win_nim(5), true);
}
