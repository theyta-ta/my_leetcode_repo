impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.windows(2).fold(0, |accu, wind| accu + (wind[1] - wind[0]).max(0))
    }
}
