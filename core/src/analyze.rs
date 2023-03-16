use crate::board::{Board, BoardData};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum AnalyzedCell {
    Invalid,
    Value(usize),
    Undetermined(Vec<usize>),
}

pub type AnalyzedBoardData = BoardData<AnalyzedCell>;
pub type AnalyzedBoard = Board<AnalyzedCell>;

pub fn analyze_cell(board: &Board, row: usize, col: usize) -> Option<AnalyzedCell> {
    let value_ref = board.at(row, col)?;
    let value = *value_ref;

    if value > 0 {
        return Some(AnalyzedCell::Value(value));
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

    let possible_options = all_options
        .difference(&occupied_options)
        .map(|val| *val)
        .collect::<Vec<_>>();

    if possible_options.len() == 0 {
        return Some(AnalyzedCell::Invalid);
    }

    if possible_options.len() == 1 {
        return Some(AnalyzedCell::Value(possible_options[0]));
    }

    Some(AnalyzedCell::Undetermined(possible_options))
}

pub fn is_invalid_analyze(analyzed_board: &AnalyzedBoard) -> bool {
    analyzed_board.get_rows_flat().iter().find(|val| ***val == AnalyzedCell::Invalid ).is_some()
}

pub fn analyze_board(board: &Board) -> Option<AnalyzedBoard> {
    let size = board.get_size();
    let mut board_size_data: AnalyzedBoardData = vec![];

    for row in 0..size {
        let mut row_list: Vec<AnalyzedCell> = vec![];
        for col in 0..size {
            let cell = analyze_cell(board, row, col)?;
            row_list.push(cell);
        }
        board_size_data.push(row_list);
    }

    return Board::from(&board_size_data).ok();
}
