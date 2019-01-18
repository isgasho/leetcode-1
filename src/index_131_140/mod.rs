use std::collections::HashMap;

// 136
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut store = HashMap::new();

    for num in nums {
        if store.get(&num) == None {
            store.insert(num, 1);
        } else {
            store.remove(&num);
        }
    }

    return *store.keys().next().unwrap();
}
