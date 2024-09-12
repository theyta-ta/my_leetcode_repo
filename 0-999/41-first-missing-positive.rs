// NOT the most efficient solution
// it would be more efficient to loop through nums, 
// place all elems in the right place
// then just check the start of the array
// this way you have to do only as many swaps as elems in 1..=nums.len()
// rather than my solution which will do many swaps - even though 
// it stops as early as possible
impl Solution {
    // we want to try and make as much of 1..=n at the start of nums
    // every time we find an element that cannot go there
    // i.e. <=0, >n, or a duplicate, we put it at the end of this slice
    // then decrease n by 1. 
    // n starts as the length of nums.
    // once we've made as much as we can, just return n + 1
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mut max_possible = nums.len(); 
        let mut i = 0;
        while i < max_possible {
            if nums[i] as usize == i+1 { i += 1; continue; }

            if nums[i] <= 0 || nums[i] as usize > max_possible {
                max_possible -= 1; // wouldve loved to've done `--max_possible` lol
                nums.swap(i, max_possible);
            } else {
                let curr = nums[i].clone();
                nums.swap(i, curr as usize - 1);
                if nums[i] == curr { // num is duplicate
                    // if this was longer than 2 lines i'd've made a macro
                    // wouldn't've made a closure because ownership then gets annoying
                    max_possible -= 1;
                    nums.swap(i, max_possible);
                }
            }
        }

        1+i as i32
    }
}
