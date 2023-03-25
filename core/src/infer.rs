use crate::analyze::{AnalyzedBoard, AnalyzedCell};

// first number is the index, second number is the value.
pub type InferType = (usize, usize);

#[derive(PartialEq, Debug, Clone)]
pub struct InferredPosition<T = usize> {
    pub row: usize,
    pub col: usize,
    pub value: T,
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

    results.sort_by(|&(i1), (i2)| i1.cmp(i2));
    results.dedup_by(|(i1), (i2)| i1 == i2);

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

pub fn infer_square_of(board: &AnalyzedBoard, row: usize, col: usize) -> Vec<InferredPosition> {
    let (s_row, s_col) = board.get_square_position_of(row, col);
    let square = board.get_square(s_row, s_col);

    if square.is_none() {
        return Vec::new();
    }

    infer_group(square.unwrap())
        .into_iter()
        .map(|(flattened_index, value)| {
            let s_size = board.get_square_size();
            let inner_row = flattened_index / s_size;
            let inner_col = flattened_index % s_size;

            InferredPosition {
                value,
                row: s_row + inner_row,
                col: s_col + inner_col,
            }
        })
        .collect()
}
