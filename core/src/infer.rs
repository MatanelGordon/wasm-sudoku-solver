use std::collections::HashSet;
use crate::analyze::{AnalyzedBoard, AnalyzedCell};
use std::fmt::{Display, Formatter};

// first number is the index, second number is the value.
pub type InferType = (usize, usize);

#[derive(PartialEq, Debug, Clone)]
pub struct InferredPosition<T = usize> {
    pub row: usize,
    pub col: usize,
    pub value: T,
}

impl<T> Display for InferredPosition<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{}) -> {}", self.row, self.col, self.value)
    }
}

fn infer_group(group: &Vec<AnalyzedCell>) -> Vec<InferType> {
    let known: Vec<usize> = group.iter().filter_map(|x| x.get_value()).collect();

    let flattened: Vec<usize> = group
        .iter()
        .filter_map(|x| x.get_undetermined())
        .flatten()
        .filter(|&x| !known.contains(x))
        .map(|x| *x)
        .collect();

    let is_valid_infer = flattened.iter().chain(known.iter()).collect::<HashSet<_>>().len() == group.len();

    if !is_valid_infer {
        panic!("Invalid Infer");
    }

    let single_repeating_values: Vec<usize> = flattened
        .iter()
        .filter(|&val| flattened.iter().filter(|&val1| val == val1).count() == 1)
        .map(|x| *x)
        .collect();

    let mut results: Vec<InferType> = single_repeating_values
        .into_iter()
        .map(|val| {
            let index = group
                .iter()
                .position(|cell| {
                    cell.get_undetermined()
                        .unwrap_or(&Vec::new())
                        .contains(&val)
                })
                .unwrap();

            (index, val)
        })
        .collect();

    results.sort_by(|&(i1, ..), (i2, ..)| i1.cmp(i2));
    results.dedup_by(|(i1, ..), (i2, ..)| i1 == i2);

    // recursive search after found solutions - are there any other solutions after setting this value?
    if results.len() > 0 {
        let mut cloned = group.to_vec();

        // no need to worry about updating the Undetermined options
        // i already filter known numbers from the flattened
        for &(i, v) in results.iter() {
            cloned[i] = AnalyzedCell::Value(v);
        }

        results.extend(infer_group(&cloned));
    }

    results
}

pub fn infer_row(board: &AnalyzedBoard, index: usize) -> Vec<InferredPosition> {
    let row = board.get_row(index);

    if row.is_none() {
        return Vec::new();
    }

    infer_group(row.unwrap())
        .into_iter()
        .map(|(col, value)| InferredPosition {
            col,
            value,
            row: index,
        })
        .collect()
}

pub fn infer_col(board: &AnalyzedBoard, index: usize) -> Vec<InferredPosition> {
    let col = board.get_col(index);

    if col.is_none() {
        return Vec::new();
    }

    infer_group(col.unwrap())
        .into_iter()
        .map(|(row, value)| InferredPosition {
            row,
            value,
            col: index,
        })
        .collect()
}

pub fn infer_square(board: &AnalyzedBoard, row: usize, col: usize) -> Vec<InferredPosition> {
    println!("Inferring square {row},{col}");
    let square = board.get_square(row, col);

    if square.is_none() {
        return Vec::new();
    }

    println!("Square: {:?}", &square);

    infer_group(square.unwrap())
        .into_iter()
        .map(|(flattened_index, value)| {
            let s_size = board.get_square_size();
            let inner_row = flattened_index / s_size;
            let inner_col = flattened_index % s_size;

            InferredPosition {
                value,
                row: row * s_size + inner_row,
                col: col * s_size + inner_col,
            }
        })
        .collect()
}

pub fn infer_square_of(board: &AnalyzedBoard, row: usize, col: usize) -> Vec<InferredPosition> {
    let (s_row, s_col) = board.get_square_position_of(row, col);
    infer_square(board, s_row, s_col)
}

pub fn infer_all(board: &AnalyzedBoard) -> Vec<InferredPosition> {
    let mut positions: Vec<InferredPosition> = Vec::new();
    let size = board.get_size();
    let s_size = board.get_square_size();

    for i in 0..size {
        let s_row = i / s_size;
        let s_col = i % s_size;

        positions.extend(infer_row(board, i));
        positions.extend(infer_col(board, i));
        // positions.extend(infer_square(board, s_row, s_col));
    }

    positions.sort_by(|a, b| {
        ((a.row as isize - b.row as isize) * s_size as isize + a.col as isize - b.col as isize)
            .cmp(&0)
    });
    positions.dedup_by(|a, b| a.row == b.row && a.col == b.col);

    positions
}

pub fn infer_positions(
    board: &AnalyzedBoard,
    positions: &Vec<(usize, usize)>,
) -> Vec<InferredPosition> {
    let mut all_inferred: Vec<InferredPosition> = Vec::new();
    let mut cached_rows: Vec<usize> = Vec::new();
    let mut cached_cols: Vec<usize> = Vec::new();
    let mut cached_squares: Vec<usize> = Vec::new();
    let square_size = board.get_square_size();

    for &(row, col) in positions.iter() {
        if !cached_rows.contains(&row) {
            cached_rows.push(row);
            all_inferred.extend(infer_row(board, row));
        }

        if !cached_cols.contains(&col) {
            cached_cols.push(col);
            all_inferred.extend(infer_col(board, col));
        }

        let square_index = row * square_size + col;
        if !cached_squares.contains(&square_index) {
            cached_squares.push(square_index);
            all_inferred.extend(infer_square_of(board, row, col));
        }
    }

    all_inferred.sort_by(|a, b| {
        ((a.row as isize - b.row as isize) * square_size as isize + a.col as isize - b.col as isize)
            .cmp(&0)
    });
    all_inferred.dedup_by(|a, b| a.row == b.row && a.col == b.col);

    return all_inferred;
}