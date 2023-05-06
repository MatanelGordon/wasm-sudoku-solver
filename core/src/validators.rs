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
    let square_size = board.get_square_size();

    for axis in 0..size{
        let rows = board.get_row(axis).unwrap();
        let cols = board.get_col(axis).unwrap();
        let square = board.get_square(axis / square_size, axis % square_size).unwrap();

        for group in vec![rows, cols, square].into_iter() {
            let is_repeating = group.iter().any(|val| *val > 0 && group.iter().filter(|&x| x == val).count() > 1);
            if is_repeating {
                return false
            }
        }
    }

    true
}
