impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        // making sure a is longest
        if a.len() < b.len() {
            std::mem::swap(&mut a, &mut b);
        }

        let mut carry = 0;
        let mut ret = String::new();

        for (a, b) in a.bytes().rev()
            .zip(b.bytes().rev().chain(std::iter::repeat(b'0')))
            .map(|(a, b)| (a - b'0', b - b'0'))
        {
            match a + b + carry {
                0 => ret.push('0'), // carry cuaranteed to be 0
                1 => { ret.push('1'); carry = 0; }
                2 => { ret.push('0'); carry = 1; }
                3 => { ret.push('1'); carry = 1; }
                _ => unreachable!()
            }
        }
        if carry == 1 { ret.push('1'); }

        ret.chars().rev().collect::<String>()
    }
}
