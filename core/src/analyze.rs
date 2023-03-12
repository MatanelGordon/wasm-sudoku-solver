use std::collections::HashSet;
use crate::board::Board;
use crate::types::StrResult;

#[derive(Debug, Clone)]
pub enum AnalyzedCell {
    Invalid,
    Determined(usize),
    Undetermined(Vec<usize>),
    Default,
}

pub type AnalyzedBoard = Vec<Vec<AnalyzedCell>>;

pub fn analyze_cell(board: &Board, row: usize, col: usize) -> Option<AnalyzedCell> {
    let value = board.at(row, col)?;

    if value > 0{
        return Some(AnalyzedCell::Determined(value));
    }

    let size = board.get_size();
    let row_list = board.get_row(row)?;
    let col_list = board.get_col(col)?;
    let square_list = board.get_square(row, col)?;
    let mut all_axis_options: Vec<usize> = Vec::new();
    let all_options: HashSet<usize> = (1..=size).collect::<HashSet<_>>();

    all_axis_options.extend(row_list);
    all_axis_options.extend(col_list);
    all_axis_options.extend(square_list);

    let occupied_options: HashSet<usize> = all_axis_options
        .into_iter()
        .filter(|v| *v != 0)
        .collect::<HashSet<_>>();

    let possible_options = all_options.difference(&occupied_options).map(|val| *val).collect::<Vec<_>>();

    if possible_options.len() == 0 {
        return Some(AnalyzedCell::Invalid);
    }

    if possible_options.len() == 1{
        return Some(AnalyzedCell::Determined(possible_options[0]));
    }

    Some(AnalyzedCell::Undetermined(possible_options))
}


