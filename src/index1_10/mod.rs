use std::collections::HashMap;

// 1
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_index = HashMap::new();

    for (index, num) in nums.iter().enumerate() {
        num_index.insert(num, index as i32);
    }

    for (index, num) in nums.iter().enumerate() {
        let diff = target - num;

        if num_index.contains_key(&diff) && *num_index.get(&diff).unwrap() != index as i32 {
            return vec![index as i32, *num_index.get(&diff).unwrap()];
        }
    }

    return vec![0, 0];
}
