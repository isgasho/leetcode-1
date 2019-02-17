extern crate leetcode;

use leetcode::index_171_180::*;

// 171
#[test]
#[ignore]
fn title_to_number_works() {
    assert_eq!(title_to_number(String::from("AB")), 28);
    assert_eq!(title_to_number(String::from("ZY")), 701);
}
