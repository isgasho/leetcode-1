use std::cmp;

// 121
pub fn max_profit_121(prices: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut max_cur = 0;

    for i in 1..prices.len() {
        max_cur += prices[i] - prices[i - 1];
        max_cur = cmp::max(0, max_cur);
        max = cmp::max(max, max_cur);
    }

    max
}

// 122
pub fn max_profit_122(prices: Vec<i32>) -> i32 {
    let mut profit = 0;

    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            profit += prices[i] - prices[i - 1];
        }
    }

    profit
}
