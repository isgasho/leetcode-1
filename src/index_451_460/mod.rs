// 453
pub fn min_moves(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let min_value = *nums.iter().min().unwrap() as i32;
    let length = nums.len() as i32;

    sum - min_value * length
}

// 455
pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    let mut g = g;
    let mut s = s;

    g.sort();
    s.sort();
    let g_len = g.len();
    let s_len = s.len();
    let mut g_i = 0;
    let mut s_i = 0;

    while g_i < g_len && s_i < s_len {
        if g[g_i] <= s[s_i] {
            g_i += 1;
        }
        s_i += 1;
    }

    g_i as i32
}
