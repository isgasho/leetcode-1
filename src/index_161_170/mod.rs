// 167
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l = 0;
    let mut r = numbers.len() - 1;
    let mut result: Vec<i32> = vec![0; 2];

    while l < r {
        let sum = numbers[l] + numbers[r];

        if sum > target {
            r -= 1;
        } else if sum < target {
            l += 1;
        } else {
            result[0] = l as i32 + 1;
            result[1] = r as i32 + 1;
            break;
        }
    }

    result
}

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
