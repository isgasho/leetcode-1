extern crate leetcode;

use leetcode::index_381_390::*;

// 383
#[test]
fn can_construct_works() {
    assert_eq!(can_construct(String::from("aa"), String::from("aab")), true);
    assert_eq!(can_construct(String::from("aa"), String::from("ab")), false);
}

// 387
#[test]
fn first_uniq_char_works() {
    assert_eq!(first_uniq_char(String::from("leetcode")), 0);
    assert_eq!(first_uniq_char(String::from("loveleetcode")), 2);
}

// 389
#[test]
fn find_the_difference_works() {
    assert_eq!(
        find_the_difference("abcd".to_string(), "abcde".to_string()),
        'e'
    );
}
