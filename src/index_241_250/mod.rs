use std::collections::HashMap;

// 242
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut store = HashMap::new();

    for c in t.chars() {
        *store.entry(c).or_insert(0) += 1;
    }

    for c in s.chars() {
        if !store.contains_key(&c) || *store.get(&c).unwrap() == 0 {
            return false;
        } else {
            *store.get_mut(&c).unwrap() -= 1;
        }
    }

    return true;
}
