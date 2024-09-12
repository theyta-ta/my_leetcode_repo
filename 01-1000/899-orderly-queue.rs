impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut s = s;
        if k > 1 {
            let mut s = s.chars().collect::<Vec<char>>();
            s.sort_unstable();
            return s.into_iter().collect::<String>();
        }

        let mut min: (char, Vec<usize>) = ('z', Vec::new());
        for (i, c) in s.chars().enumerate() {
            if c > min.0 {
                continue;
            } else if c == min.0 {
                min.1.push(i);
            } else {
                min.0 = c;
                min.1 = vec![i];
            }
        }

        println!{"{:?}", min};

        let mut l_count = 0;
        let mut ret_min = s.clone();
        let mut s = s.chars().collect::<Vec<char>>();

        for idx in min.1 {
            let dist = idx - l_count;
            l_count  = idx;
            s.rotate_left(dist);
            let st = s.iter().collect::<String>();
            if st < ret_min {
                ret_min = st.clone();
            }
        }

        ret_min
    }
}
