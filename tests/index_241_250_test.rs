extern crate leetcode;

use leetcode::index_241_250::*;

// 242
#[test]
fn is_anagram_works() {
    assert_eq!(
        is_anagram(String::from("anagram"), String::from("nagaram")),
        true
    );
    assert_eq!(is_anagram(String::from("rat"), String::from("car")), false);
    assert_eq!(
        is_anagram(String::from("aacc"), String::from("ccac")),
        false
    );
}
