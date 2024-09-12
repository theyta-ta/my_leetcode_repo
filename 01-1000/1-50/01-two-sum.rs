use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (idx, num) in nums.into_iter().enumerate() {
            let idx = idx as i32; // fuck you leetcode
            if let Some(&other_idx) = map.get(&num) {
                return vec![idx, other_idx];
            }
            let _ = map.insert(target - num, idx);
        }
        unreachable!();
    }
}
