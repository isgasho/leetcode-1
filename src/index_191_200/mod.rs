use std::cmp;

// 198
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    if nums.len() == 1 {
        return nums[0];
    }
    if nums.len() == 2 {
        return cmp::max(nums[0], nums[1]);
    }
    let mut store = vec![0; nums.len()];
    store[0] = nums[0];
    store[1] = cmp::max(nums[0], nums[1]);

    for i in 2..nums.len() {
        store[i] = if nums[i] + store[i - 2] > store[i - 1] {
            nums[i] + store[i - 2]
        } else {
            store[i - 1]
        }
    }

    store[store.len() - 1]
}
