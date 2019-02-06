// 453
pub fn min_moves(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let min_value = *nums.iter().min().unwrap() as i32;
    let length = nums.len() as i32;

    sum - min_value * length
}
