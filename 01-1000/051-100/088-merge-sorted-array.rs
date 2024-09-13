impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut p1, mut p2) = (m as usize, n as usize);
        for i in (0..m+n).rev() {
            if p1 <= 0 {
                nums1[i as usize] = nums2[p2 - 1];
                p2 -= 1;
            } else if p2 <= 0 {
                nums1[i as usize] = nums1[p1 - 1];
                p1 -= 1;
            } else {
                if nums1[p1 - 1] > nums2[p2 - 1] {
                    nums1[i as usize] = nums1[p1 - 1];
                    p1 -= 1;
                } else {
                    nums1[i as usize] = nums2[p2 - 1];
                    p2 -= 1; 
                }
            }
        }
    }
}
