impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut x = x; let mut n = n as i64;
        if n == 0 { return 1.; }
        if n < 0 {
            x = 1.0 / x;
            n = -n;
        } 

        let mut ret = 1.;
        while n > 0 {
            if n & 1 == 1 {
                ret *= x;
            }
            x *= x;
            n >>= 1;
        }

        ret
    }
}
