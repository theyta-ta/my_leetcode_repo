impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut mins = [i32::MAX, i32::MAX];
        for num in nums {
            if num <= mins[0] {
                mins[0] = num;
            } else if num <= mins[1] {
                mins[1] = num;
            } else {
                return true;
            }
        }
        false
    }
}
