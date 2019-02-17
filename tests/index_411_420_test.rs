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
