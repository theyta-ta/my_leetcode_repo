use std::collections::HashMap;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() <= 1 { return points.len() as _; }
        let mut max: i32 = 2;
        for (idx, i) in points.iter().enumerate() {
            let mut slope_counts = HashMap::new();
            for j in &points[(idx+1)..] {
                let delta_x = (j[0] - i[0]) as f64;
                let delta_y = (j[1] - i[1]) as f64;
                let slope = if delta_x == 0. {
                    i64::MAX
                } else {
                    ((delta_y * 1_000_000_000_000.) / delta_x) as i64
                };
                slope_counts.entry(slope).and_modify(|x: &mut i32| {
                    max = (*x+1).max(max);
                    *x += 1;
                }).or_insert(2);
            }
            //println!("{:?}", slope_counts);
        }

        max
    }
}
