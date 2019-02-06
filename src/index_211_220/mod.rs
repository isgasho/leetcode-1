// 217
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    if nums.len() == 0 {
        return false;
    }
    let mut nums = nums;
    nums.sort();
    let length = nums.len() - 1;

    for i in 0..length {
        if nums[i + 1] == nums[i] {
            return true;
        }
    }

    false
}
