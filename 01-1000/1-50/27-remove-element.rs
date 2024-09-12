// lmaooo you dont actually have to remove shit!
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut ptr = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[ptr] = nums[i];
                ptr += 1;
            }
        }
        ptr as _
    }
}
