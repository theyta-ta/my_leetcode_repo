impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut allowed_mask: u32 = 0;
        for b in allowed.as_bytes() { allowed_mask |= 1 << (b - b'a'); }
        words.into_iter()
            .filter(|w| w.as_bytes()
                .iter()
                .all(|c| allowed_mask == allowed_mask | 1 << (c - b'a'))
            ).count() as i32
    }
}
