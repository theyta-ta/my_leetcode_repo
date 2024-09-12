fn unique<'a>(iter: impl Iterator<Item = &'a char>) -> bool {
    let mut count = [0; 9];
    iter
        .filter_map(|&c| c.to_digit(10))
        .for_each(|d| count[d as usize - 1] += 1);
        
    count.into_iter().all(|c| c <= 1)
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let block_it = |i, j| board[i..i+3].iter().flat_map(move |block_row| block_row[j..j+3].iter());

        (0..9).step_by(3).all(|i| (0..9).step_by(3).all(|j| unique(block_it(i, j))))
            && board.iter().all(|i| unique(i.iter()))
            && (0..9).all(|i| unique(board.iter().map(|row| &row[i])))
    }
}
