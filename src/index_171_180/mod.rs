// 171
pub fn title_to_number(s: String) -> i32 {
    let mut result = 0;
    let base: u32 = 26;

    for (i, c) in s.chars().rev().enumerate() {
        let i = i as u32;
        let c = c as u32;
        result += (c - 64) * base.pow(i)
    }

    result as i32
}
