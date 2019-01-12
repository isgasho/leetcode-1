extern crate leetcode;

use leetcode::index_491_500::*;

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
