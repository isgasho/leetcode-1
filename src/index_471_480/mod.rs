// 476
pub fn find_complement(num: i32) -> i32 {
    return i32::from_str_radix(
        &*format!("{:b}", num)
            .chars()
            .map(|x| if x == '0' { return '1' } else { return '0' })
            .collect::<String>(),
        2,
    )
    .unwrap();
}
