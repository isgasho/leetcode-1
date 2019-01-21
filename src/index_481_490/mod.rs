use std::cmp;

// 485
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut cur = 0;
    let mut max = 0;

    for num in nums {
        if num == 0 {
            max = cmp::max(max, cur);
            cur = 0;
        } else {
            cur += 1;
        }
    }

    cmp::max(max, cur)
}
