use std::collections::VecDeque;

impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        // translate by `location`
        // use `atan2` to get angle of polar. no need for length
        // sort by angle
        // find max subarray
        let mut temp: (Vec<_>, Vec<_>) = points.into_iter()
            .map(|x| (x[0] - location[0], x[1] - location[1]))
            .partition(|&x| x == (0, 0));

        let mut angles: Vec<_> = temp.1.into_iter()
            .map(|(x, y)| f64::atan2(y as f64, x as f64))
            .collect();
        
        let at_feet = temp.0.len();

        angles.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let angle = f64::to_radians(angle as f64);

        let mut deq = VecDeque::new();
        if angles.is_empty() { return at_feet as i32; }
        deq.push_back(angles[0]);
        let mut max_len = 1;

        for ang in angles[1..].iter().cloned().chain(
            angles.iter()
                .take_while(|&&x| x <= angle / 2.)
                .map(|x| x + std::f64::consts::TAU)
        ) {
            if ang - deq.front().unwrap() <= angle {
                deq.push_back(ang);
                max_len = max_len.max(deq.len());
            } else {
                while let Some(n) = deq.pop_front() {
                    if ang - n <= angle {
                        deq.push_front(n);
                        break
                    }
                }
                deq.push_back(ang);
            }
        }

        (max_len + at_feet) as i32
    }
}
