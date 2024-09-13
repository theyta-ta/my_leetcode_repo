impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut ret: Vec<Vec<_>> = Vec::with_capacity(num_rows);
        ret.push(vec![1]);

        for i in 1..num_rows {
            let mut push = vec![1; i + 1];

            //push[0] = 1; push[i] = 1;

            for j in 1..i {
                push[j] = ret[i-1][j-1] + ret[i-1][j];
            }

            ret.push(push);
        }

        ret
    }
}
