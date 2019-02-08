use std::collections::HashMap;

// 383
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut store = vec![0; 26];

    for c in magazine.chars() {
        let i = c as usize;
        store[i - 97] += 1;
    }

    for c in ransom_note.chars() {
        let i = c as usize;
        if store[i - 97] == 0 {
            return false;
        }
        store[i - 97] -= 1;
    }

    return true;
}

// 387
pub fn first_uniq_char(s: String) -> i32 {
    let mut store = vec![0; 26];

    for c in s.chars() {
        let i = c as usize;
        store[i - 97] += 1;
    }

    for (i, c) in s.chars().enumerate() {
        let store_i = c as usize;
        if store[store_i - 97] == 1 {
            return i as i32;
        }
    }

    return -1;
}

// 389
pub fn find_the_difference(s: String, t: String) -> char {
    let mut store = HashMap::new();
    let mut result = 'a';

    for c in s.chars() {
        *store.entry(c).or_insert(1) += 1;
    }

    for c in t.chars() {
        if store.contains_key(&c) {
            *store.get_mut(&c).unwrap() -= 1;
            if *store.get(&c).unwrap() == 0 {
                store.remove(&c);
            }
        } else {
            result = c;
            break;
        }
    }

    result
}
