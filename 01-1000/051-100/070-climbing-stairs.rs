impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // only 45, can just precompute arr.
        // thats boring tho

        let mut a = 1;
        let mut b = 1;
        for _ in 1..n {
            (b, a) = (a, a+b);
        }
        a
    }
}
