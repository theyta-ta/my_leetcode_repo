impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut count = 0;
        let mut prev = 'I';
        for c in s.chars().rev() {
            match c {
                'M' => {count+=1000; prev='M'},
                'D' => {count+=500; prev='D'},
                'L' => {count+=50; prev='L'},
                'V' => {count+=5; prev='V'},
                'C' => {count += if prev=='M' || prev=='D' {-100} else {100}},
                'X' => {count += if prev=='C' || prev=='L' {-10} else {10}},
                'I' => {count += if prev=='X' || prev=='V' {-1} else {1}},
                _ => (),
            }
            prev = c;
        }
        count
    }
}
