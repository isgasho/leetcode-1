extern crate leetcode;

use leetcode::index_341_350::*;

// 344
#[test]
fn reverse_string_works() {
    assert_eq!(reverse_string("hello".to_string()), "olleh");
    assert_eq!(
        reverse_string("A man, a plan, a canal: Panama".to_string()),
        "amanaP :lanac a ,nalp a ,nam A"
    )
}

// 349
#[test]
fn intersection_works() {
    assert_eq!(intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
}
