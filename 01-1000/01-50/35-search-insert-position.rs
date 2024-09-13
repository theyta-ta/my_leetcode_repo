// just use the inbuilt binary_search method
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + ((right - left) >> 1);
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as _
    }
}
