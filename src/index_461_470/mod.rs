// 461
pub fn hamming_distance(x: i32, y: i32) -> i32 {
    return format!("{:b}", x ^ y)
        .chars()
        .filter(|x| *x == '1')
        .collect::<String>()
        .len() as i32;
}

// 463
pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let mut island = 0;
    let mut repeat = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, &item) in row.iter().enumerate() {
            if item == 1 {
                island += 1;
                repeat += if i != 0 && grid[i - 1][j] == 1 { 1 } else { 0 };
                repeat += if j != 0 && grid[i][j - 1] == 1 { 1 } else { 0 };
            }
        }
    }

    return island * 4 - repeat * 2;
}
