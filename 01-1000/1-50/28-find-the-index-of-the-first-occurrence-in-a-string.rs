impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(n) => n as _,
            None => -1i32,
        }
    }
}
