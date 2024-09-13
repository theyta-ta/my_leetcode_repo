impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        enum Side {
            Top, Bottom, Left, Right
        }

        let mut cur_side = Side::Top;

        // going to slowly circle in, reducing the bounds
        let mut top_bound = 0;
        let mut bottom_bound = matrix.len();
        let mut left_bound = 0;
        let mut right_bound = matrix[0].len();

        let mut ret = Vec::with_capacity(bottom_bound * right_bound);

        while (top_bound < bottom_bound) && (left_bound < right_bound) {
            match cur_side {
                Side::Top => {
                    ret.extend_from_slice(&matrix[top_bound][left_bound..right_bound]);
                    top_bound += 1;
                    cur_side = Side::Right;
                },
                Side::Bottom => {
                    ret.extend(matrix[bottom_bound - 1][left_bound..right_bound].iter().rev());
                    bottom_bound -= 1;
                    cur_side = Side::Left;
                },
                Side::Left => {
                    let col_iter = (top_bound..bottom_bound).rev().map(|n| matrix[n][left_bound]);
                    ret.extend(col_iter);
                    left_bound += 1;
                    cur_side = Side::Top;
                },
                Side::Right => {
                    let col_iter = (top_bound..bottom_bound).map(|n| matrix[n][right_bound - 1]);
                    ret.extend(col_iter);
                    right_bound -= 1;
                    cur_side = Side::Bottom;
                },
            }
        }

        ret
    }
}
