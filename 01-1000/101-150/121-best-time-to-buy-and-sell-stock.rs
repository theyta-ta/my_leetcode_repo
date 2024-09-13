impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.into_iter().fold((i32::MAX, 0), |acc, x| (acc.0.min(x), acc.1.max(x - acc.0))).1
    }
}
