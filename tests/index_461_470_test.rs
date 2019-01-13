extern crate leetcode;

use leetcode::index_461_470::*;

#[test]
fn hamming_distance_works() {
    assert_eq!(hamming_distance(1, 4), 2);
}

#[test]
fn island_perimeter_works() {
    assert_eq!(
        island_perimeter(vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0]
        ]),
        16
    );
    assert_eq!(island_perimeter(vec![vec![1]]), 4);
    assert_eq!(island_perimeter(vec![vec![1, 0]]), 4);
    assert_eq!(island_perimeter(vec![vec![0, 1]]), 4);
    assert_eq!(island_perimeter(vec![vec![0, 0, 0, 1]]), 4);
}
