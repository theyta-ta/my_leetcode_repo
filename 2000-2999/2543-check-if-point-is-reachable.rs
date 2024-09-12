impl Solution {
    pub fn is_reachable(mut m: i32, mut n: i32) -> bool {
        // reachable iff coprime
        // coprime iff gcd == 1
        // so just return gcd == 1
        m >>= m.trailing_zeros();
        n >>= n.trailing_zeros();

        loop {
            if m > n {
                std::mem::swap(&mut m, &mut n);
            }
            // m <= n
            n -= m;
            if n == 0 {
                return m == 1;
            }
            n >>= n.trailing_zeros();
        }
    }
}
