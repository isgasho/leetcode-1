// 283
pub fn move_zeroes(nums: &mut Vec<i32>) -> Vec<i32> {
    let mut index = 0;

    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[index] = nums[i];
            index += 1;
        }
    }

    while index < nums.len() {
        nums[index] = 0;
        index += 1;
    }

    nums.to_vec()
}
