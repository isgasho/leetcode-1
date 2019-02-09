// 268
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();

    for (i, num) in nums.iter().enumerate() {
        if i as i32 != *num {
            return *num - 1;
        }
    }

    return nums.len() as i32;
}
