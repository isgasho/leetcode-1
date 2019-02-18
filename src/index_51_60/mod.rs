use std::cmp;

// 53
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut tmp_array = Vec::with_capacity(nums.len());
    tmp_array.push(nums[0]);
    let mut max = tmp_array[0];

    for i in 1..nums.len() {
        let current_max = if tmp_array[i - 1] > 0 {
            nums[i] + tmp_array[i - 1]
        } else {
            nums[i]
        };
        tmp_array.push(current_max);
        max = cmp::max(max, current_max);
    }

    max
}
