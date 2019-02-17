extern crate leetcode;

use leetcode::index_411_420::*;

// 412
#[test]
#[ignore]
fn fizz_buzz_works() {
    assert_eq!(
        fizz_buzz(15),
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ]
    )
}

// 415
#[test]
#[ignore]
fn add_strings_work() {
    assert_eq!(
        add_strings("12".to_string(), "24".to_string()),
        "36".to_string()
    );
    assert_eq!(
        add_strings("6913259244".to_string(), "71103343".to_string()),
        "6984362587".to_string()
    );
}
