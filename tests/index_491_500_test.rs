extern crate leetcode;

use leetcode::index_491_500::*;

// 496
#[test]
fn next_great_element_works() {
    assert_eq!(
        next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
        vec![-1, 3, -1]
    );
}

// 500
#[test]
fn find_words_works() {
    assert_eq!(
        find_words(vec![
            "Hello".to_string(),
            "Alaska".to_string(),
            "Dad".to_string(),
            "Peace".to_string()
        ]),
        vec!["Alaska".to_string(), "Dad".to_string()]
    );
}
