impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.chars().filter(|c| c.is_ascii_alphanumeric()).map(|c| c.to_ascii_lowercase());
        s.clone().eq(s.rev())
    }
}
