impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for x in digits.iter_mut().rev() {
            match *x {
                9 => *x = 0,
                _ => {
                    *x += 1;
                    return digits;
                }
            }
        }
        digits.insert(0, 1);
        digits
    }
}
