use std::collections::HashMap;

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
