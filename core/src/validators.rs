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

    for row in 0..size {
        for col in 0..size {
            let rows = board.get_row(row).unwrap();
            let cols = board.get_col(col).unwrap();
            let square = board.get_square_of(row, col).unwrap();

            for group in vec![rows, cols, square].into_iter() {
                let is_repeating = group.iter().any(|val| *val > 0 && group.iter().filter(|&x| x == val).count() > 1);
                if is_repeating {
                    return false
                }
            }
        }
    }
    true
}
