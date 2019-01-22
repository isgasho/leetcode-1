extern crate leetcode;

use leetcode::index_381_390::*;

// 389
#[test]
fn find_the_difference_works() {
    assert_eq!(
        find_the_difference("abcd".to_string(), "abcde".to_string()),
        'e'
    );
}
