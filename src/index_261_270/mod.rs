// 263
pub fn is_ugly(num: i32) -> bool {
    if num == 0 {
        return false;
    }
    let mut num = num;
    let factors = vec![2, 3, 5];

    for f in factors {
        while num % f == 0 {
            num = num / f;
        }
    }

    num == 1
}

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
