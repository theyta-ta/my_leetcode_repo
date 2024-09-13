impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut ret = Vec::with_capacity(row_index);

        if row_index == 0 {
            ret.push(1);

            return ret;
        }

        ret.push(1);
        for i in 1..=row_index {
            for j in (1..i).rev() {
                ret[j] += ret[j-1];
            }
            ret.push(1)
        }

        ret
    }
}
