// 198
pub fn rob(nums: Vec<i32>) -> i32 {
    let mut store = vec![0; nums.len()];
    store[0] = nums[0];
    store[1] = nums[1];

    for i in 2..nums.len() {
        store[i] = if nums[i] + store[i - 2] > store[i - 1] {
            nums[i] + store[i - 2]
        } else {
            store[i - 1]
        }
    }

    store[store.len() - 1]
}
