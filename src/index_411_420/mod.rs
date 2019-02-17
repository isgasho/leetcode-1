// 412
pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut result = vec![];
    let mut i = 1;

    while i <= n {
        if i % 15 == 0 {
            result.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            result.push("Fizz".to_string());
        } else if i % 5 == 0 {
            result.push("Buzz".to_string());
        } else {
            result.push(i.to_string());
        }

        i += 1;
    }

    return result;
}

// 415
pub fn add_strings(num1: String, num2: String) -> String {
    let mut num1 = num1.chars().rev().collect::<String>();
    let mut num2 = num2.chars().rev().collect::<String>();
    let mut result = String::from("");
    if num1.len() < num2.len() {
        let distance = num2.len() - num1.len();
        num1.push_str(&(0..distance).map(|_| "0").collect::<String>());
    } else {
        let distance = num1.len() - num2.len();
        num2.push_str(&(0..distance).map(|_| "0").collect::<String>());
    }

    let mut overflow = 0;
    for i in 0..num1.len() {
        let a = num1.chars().nth(i).unwrap().to_digit(10).unwrap();
        let b = num2.chars().nth(i).unwrap().to_digit(10).unwrap();
        let sum = a + b + overflow;
        overflow = sum / 10;

        result.push_str(&(sum % 10).to_string());
    }
    if overflow != 0 {
        result.push_str(&overflow.to_string());
    }

    result.chars().rev().collect()
}
