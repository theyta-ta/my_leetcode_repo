impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        (1..=(n as u64)).fold(1, |acc, x| acc*x*(2*x-1) % 1_000_000_007) as _
    }
}
