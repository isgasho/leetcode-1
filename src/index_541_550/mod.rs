// 541
pub fn reverse_str(s: String, k: i32) -> String {
    let mut result = String::from("");
    let k = k as usize;

    let mut i = 0;
    loop {
        let from = k * i;
        let to = k * (i + 1);

        if i % 2 == 0 {
            if to < s.len() {
                result.push_str(&s[from..to].chars().rev().collect::<String>());
            } else {
                result.push_str(&s[from..s.len()].chars().rev().collect::<String>());
                break;
            }
        } else {
            if to < s.len() {
                result.push_str(&s[from..to]);
            } else {
                result.push_str(&s[from..s.len()]);
                break;
            }
        }

        i += 1;
    }

    result
}
