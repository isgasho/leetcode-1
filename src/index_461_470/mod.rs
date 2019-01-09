// 461
pub fn hamming_distance(x: i32, y: i32) -> i32 {
  return format!("{:b}", x ^ y).chars().filter(|x| *x == '1').collect::<String>().len() as i32;
}
