// TIME:  O(N^2)
// SPACE: O(1)
impl Solution {
    // for each char `c` keep on moving to the right till you find a
    // char distinct from `c`. this is a palendrome "blovk". if the 
    // char to the left of c is equal to the char to the right of the
    // block then we can extend the block. repeatedly extend till we
    // cant extend any more, we just ret the block with max length
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let mut l = 0;
        let mut max_len = 1;
        let mut max_start = 0;
        while l <= s.len() {
            let mut r = l;
            while r+1 < s.len() && s[r+1] == s[r] { r += 1; }
            // now s[l] == s[l+1] == ... == s[r]
            let store = r+1; // what to set l to next loop
            while l > 0 && r+1 < s.len() && s[l-1] == s[r+1] {
                l -= 1; r += 1;
            }
            let length = r-l+1;
            if length > max_len {
                max_len = length; max_start = l;
            }

            l = store;
        }

        // SAFETY: by the constraints given by this problem we have `s` is valid ascii
        // this means any subslice of bytes is valid UTF-8
        unsafe { String::from_utf8_unchecked(s[max_start..(max_start+max_len)].to_vec()) }
    }
}
