use std::collections::HashMap;

// 13
pub fn roman_to_int(s: String) -> i32 {
    let mut store = HashMap::new();
    store.insert('M', 1000);
    store.insert('D', 500);
    store.insert('C', 100);
    store.insert('L', 50);
    store.insert('X', 10);
    store.insert('V', 5);
    store.insert('I', 1);

    let mut result = 0;
    let mut pre = 'I';

    for c in s.chars().rev() {
        match (store.get(&c), store.get(&pre)) {
            (Some(c_value), Some(pre_value)) => {
                if c_value < pre_value {
                    result -= c_value;
                } else {
                    result += c_value;
                }
                pre = c;
            }
            (_, _) => break,
        }
    }

    result
}
