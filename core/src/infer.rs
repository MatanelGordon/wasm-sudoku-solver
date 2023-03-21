use crate::analyze::{AnalyzedBoard, AnalyzedCell};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct BoardPosition {
    pub row: usize,
    pub col: usize,
    pub value: usize,
}

fn is_same_position<'a>(a: &'a BoardPosition, b: &'a BoardPosition) -> bool {
    return a.row == b.row && a.col == b.col;
}

fn get_single_repeating_values(cells: &Vec<AnalyzedCell>) -> Vec<(usize, usize)> {
    if cells.iter().all(|cell| cell.is_value()) {
        return vec![];
    }

    let flatten_options = cells.iter().flat_map(|a| match a {
        AnalyzedCell::Value(_) => vec![],
        AnalyzedCell::Undetermined(opt) => opt.to_vec(),
    });

    flatten_options
        .clone()
        .collect::<HashSet<usize>>()
        .into_iter()
        .filter(|&x| flatten_options.clone().filter(|&x1| x == x1).count() == 1)
        .map(|single_value| {
            let chosen_index = cells
                .iter()
                .position(|cell| {
                    if let AnalyzedCell::Undetermined(options) = cell {
                        return options.iter().find(|&&opt| opt == single_value).is_some();
                    }
                    false
                })
                .unwrap();
            return (chosen_index, single_value);
        })
        .collect()
}

pub fn is_valid_infer(positions: &Vec<BoardPosition>) -> bool {
    positions
        .iter()
        .find(|pos| {
            positions
                .iter()
                .filter(|pos1| is_same_position(pos, pos1))
                .count()
                > 1
        })
        .is_none()
}

pub fn uniq_positions(arr: &Vec<BoardPosition>, size: Option<usize>) -> Vec<BoardPosition> {
    let mut cloned = arr.to_vec();

    cloned.sort_by(|a, b| {
        ((a.row as isize - b.row as isize) * size.unwrap_or(10) as isize + a.col as isize
            - b.col as isize)
            .cmp(&0)
    });
    cloned.dedup_by(|a, b| a.row == b.row && a.col == b.col);

    cloned
}

pub fn infer_square_reduction(
    board: &AnalyzedBoard,
    square_row: usize,
    square_col: usize,
) -> Vec<BoardPosition> {
    let square = board.get_square(square_row, square_col);
    let square_size = board.get_square_size();

    if square.is_none() {
        return vec![];
    }

    return get_single_repeating_values(square.unwrap())
        .into_iter()
        .map(|(index, value)| {
            let inner_row = index / square_size;
            let inner_col = index % square_size;
            let row = square_row * square_size + inner_row;
            let col = square_col * square_size + inner_col;
            return BoardPosition { row, col, value };
        })
        .collect();
}

pub fn infer_row_reduction(rows: &Vec<AnalyzedCell>, index: usize) -> Vec<BoardPosition> {
    get_single_repeating_values(rows)
        .into_iter()
        .map(|(curr_index, value)| BoardPosition {
            row: index,
            col: curr_index,
            value,
        })
        .collect()
}

pub fn infer_col_reduction(cols: &Vec<AnalyzedCell>, index: usize) -> Vec<BoardPosition> {
    get_single_repeating_values(cols)
        .into_iter()
        .map(|(curr_index, value)| BoardPosition {
            col: index,
            row: curr_index,
            value,
        })
        .collect()
}

pub fn infer_square_reduction_all(analyzed_board: &AnalyzedBoard) -> Vec<BoardPosition> {
    let square_size = analyzed_board.get_square_size();

    (0..square_size)
        .flat_map(move |row| {
            (0..square_size).flat_map(move |col| infer_square_reduction(analyzed_board, row, col))
        })
        .collect()
}

pub fn infer_axis_reduction(analyzed_board: &AnalyzedBoard) -> Vec<BoardPosition> {
    let size = analyzed_board.get_size();
    let mut reductions: Vec<BoardPosition> = vec![];

    for x in 0..size {
        let cols = analyzed_board.get_col(x).unwrap();
        let rows = analyzed_board.get_row(x).unwrap();
        reductions.extend(infer_col_reduction(cols, x));
        reductions.extend(infer_row_reduction(rows, x));
    }

    reductions
}
