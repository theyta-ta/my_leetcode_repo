impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim()
            .rsplit_once(" ")
            .unwrap_or_else(|| ("", &s.trim())).1
            .len() as _
    }
}
