use std::collections::HashSet;
use std::iter::FromIterator;

// 344
pub fn reverse_string(s: String) -> String {
    return s.chars().rev().collect::<String>();
}

// 349
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let set1: HashSet<i32> = HashSet::from_iter(nums1.iter().cloned());
    let set2 = HashSet::from_iter(nums2.iter().cloned());

    set1.intersection(&set2).cloned().collect()
}
