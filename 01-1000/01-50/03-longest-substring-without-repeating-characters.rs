use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut letters: HashSet<u8> = HashSet::new();
        let mut max = 0;

        while r < s.len() {
            let curr = s.as_bytes()[r];
            if letters.insert(curr) {
                r += 1;
                max = max.max(r - l);
            } else {
                for b in &s.as_bytes()[l..] {
                    l += 1;
                    if *b == curr { break; }
                    letters.remove(b);
                }
                r += 1;
            }
        }

        max as i32
    }
}
