impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut p1 = 0;
        let mut p2 = height.len() - 1;
        let mut max = 0;

        let water = |i: usize, j: usize| {(height[i].min(height[j])) * (j - i) as i32}; //i < j
        while p1 != p2 {
            max = max.max(water(p1, p2));
            if height[p1] < height[p2] {
                p1 += 1;
            } else {
                p2 -= 1;
            } 
        }
        max
    }
}
