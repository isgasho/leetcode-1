use std::collections::HashMap;

// 496
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut store = HashMap::new();
    let mut result = vec![-1i32; nums1.len()];

    for (index, num) in nums2.iter().enumerate() {
        store.insert(num, index);
    }

    for (index, num1) in nums1.iter().enumerate() {
        let from = store.get(&num1).unwrap() + 1;
        for num2 in &nums2[from..] {
            if *num2 > *num1 {
                result[index] = *num2;
                break;
            }
        }
    }

    return result;
}

// 500
pub fn find_words(words: Vec<String>) -> Vec<String> {
    let keyboard_rows = vec!["qwertyuiop", "asdfghjkl", "zxcvbnm"];
    let mut result_words = vec![];

    for word in words {
        let word_copy = word.clone();
        let word = word.to_lowercase();
        let first_char = word.chars().next().unwrap();
        let mut target_row = "";
        for row in &keyboard_rows {
            if row.contains(first_char) {
                target_row = *row;
                break;
            }
        }
        let mut in_one_line = true;

        for c in word.chars() {
            if !target_row.contains(c) {
                in_one_line = false;
                break;
            }
        }

        if in_one_line {
            result_words.push(word_copy)
        }
    }

    return result_words;
}
