impl Solution {
    pub fn my_sqrt(n: i32) -> i32 {
        if n <= 1 { return n; }
        let n = n as u32;

        let update = |x0| (x0 + n/x0) >> 1;
        let mut x0: u32 = 1 << n.ilog2()/2 + 1;
        let mut x1: u32 = update(x0);
        while x1 < x0 {
            x0 = x1;
            x1 = update(x0);
        }

        x0 as i32
    }
}
