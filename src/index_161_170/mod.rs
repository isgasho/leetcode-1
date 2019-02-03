// 169
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut counteract = 0;

    for num in nums {
        if counteract == 0 {
            result = num;
            counteract = 1;
        } else {
            counteract += if num == result { 1 } else { -1 }
        }
    }

    result
}
