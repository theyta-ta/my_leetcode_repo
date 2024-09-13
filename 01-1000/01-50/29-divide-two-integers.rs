impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        // only 32 bit is evil
        if divisor == i32::MIN { return (dividend == i32::MIN) as i32; }
        if dividend == i32::MIN {
            let a = Self::divide(dividend + divisor.abs(), divisor);
            return a.checked_sub(divisor.signum()).unwrap_or(a);
        }
        // handling negatives sounds like a pain. im gonna pussy out with my pussy out
        match (!dividend.is_negative(), !divisor.is_negative()) {
            (false, false) => {
                divisor = -divisor; dividend = -dividend;
            },
            (false, true) | (true, false) => return -Self::divide(-dividend, divisor),
            (true, true) => (),
        }

        let mut ret = 0;
        // repeatedly find power of 2 less than dividend / divisor.
        // then take that pow of 2 * divisor from dividend
        // when dividend < divisor we are done! :D
        // but we cant multiply... we *can* bitshift though!!
        // and a * 2^b == a << b
        // so yay! :D
        while dividend >= divisor {
            // think of the binary. we have some `0`s infront of a 1 in dividend
            // and less or equal `0`s in the front in divisor
            // if we shift by the difference... will it work? no!
            // if dividend is 0b100 and divisor 0b011, then after shifting we get 0b110 > 0b100
            // so uh-oh! simplest solution is to just check if its larger and take 1 if it is
            let mut k = divisor.leading_zeros() - dividend.leading_zeros();
            k -= ((divisor << k) > dividend) as u32;
            dividend -= divisor << k;
            ret += 1 << k; // could use | instead of +. same difference
        }

        ret
    }
}
