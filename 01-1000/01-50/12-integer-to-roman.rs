// hell
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num as usize;
        let mut ret = String::new();
        ret.push_str(&"M".repeat(num/1000));
        num %= 1000;
        let d = num/100;
        match d {
            0..=3 => ret.push_str(&"C".repeat(d)),
            4 => ret.push_str(&"CD"),
            5..=8 => {
                ret.push('D');
                ret.push_str(&"C".repeat(d-5));
            }
            _ => ret.push_str("CM")
        }
        num %= 100;
        let d = num/10;
        match d {
            0..=3 => ret.push_str(&"X".repeat(d)),
            4 => ret.push_str(&"XL"),
            5..=8 => {
                ret.push('L');
                ret.push_str(&"X".repeat(d-5));
            }
            _ => ret.push_str("XC")
        }
        num %= 10;
        let d = num;
        match d {
            0..=3 => ret.push_str(&"I".repeat(d)),
            4 => ret.push_str(&"IV"),
            5..=8 => {
                ret.push('V');
                ret.push_str(&"I".repeat(d-5));
            }
            _ => ret.push_str("IX")
        }
        ret.to_string()
    }
}
