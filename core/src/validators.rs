use std::collections::HashSet;

use crate::board::{Board, BoardData};

pub fn is_square(num: usize) -> bool {
    let num_f32 = num as f32;
    let sqrt = num_f32.sqrt();
    return sqrt == sqrt.floor();
}

pub fn is_square_matrix<T>(data: &BoardData<T>) -> bool {
    let len: usize = data.len();

    data.iter().all(|l| l.len() == len)
}

pub fn is_valid_sudoku(board: &Board) -> bool {
    let size = board.get_size();
    let mut all_options = Vec::<usize>::new();

    for row in 0..size {
        for col in 0..size {
            let rows = board.get_row(row).unwrap();
            let cols = board.get_col(col).unwrap();
            let square = board.get_square_of(row, col).unwrap();

            all_options.clear();
            all_options.extend(rows);
            all_options.extend(cols);
            all_options.extend(square);

            let uniq = all_options
                .iter()
                .filter(|&&val| val > 0)
                .collect::<HashSet<_>>();
            let is_repeating = uniq
                .iter()
                .all(|&num| all_options.iter().filter(|&val| val == num).count() > 3);

            if is_repeating {
                return false;
            }
        }
    }

    true
}
