use std::cmp::Ordering::{Less, Greater, Equal};

impl Solution {
    pub fn survived_robots_healths(positions: Vec<i32>, mut healths: Vec<i32>, directions: String) -> Vec<i32> {
        let len = positions.len();
        let mut stack = Vec::with_capacity(len);
        let mut idx: Vec<usize> = (0..len).collect();
        idx.sort_unstable_by(|&i, &j| positions[i].cmp(&positions[j]));

        for i in idx {
            if directions.as_bytes()[i] == b'R' {
                stack.push(i); continue;
            }

            // dir[i] = 'L'

            while !stack.is_empty() && healths[i] > 0 {
                let j = stack.pop().unwrap();
                match healths[j].cmp(&healths[i]) {
                    Less => {
                        healths[j] = 0; healths[i] -= 1;
                    },
                    Equal => {
                        healths[i] = 0; healths[j] = 0;
                    },
                    Greater => {
                        healths[i] = 0; healths[j] -= 1;
                        stack.push(j);
                    },
                }
            }
        }

        healths.into_iter().filter(|&x| x != 0).collect::<Vec<i32>>()
    }
}
