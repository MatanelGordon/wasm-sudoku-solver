use crate::board::{Board, BoardData};
use crate::types::StrResult;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum AnalyzedCell {
    Value(usize),
    Undetermined(Vec<usize>),
}

pub type AnalyzedBoardData = BoardData<AnalyzedCell>;
pub type AnalyzedBoard = Board<AnalyzedCell>;

struct BoardPosition {
    pub row: usize,
    pub col: usize,
    pub value: usize,
}

fn infer_square_reduction(analyzed_board: &AnalyzedBoard) -> Vec<BoardPosition> {
    let size = analyzed_board.get_size();
    let square_size = analyzed_board.get_square_size();
    let mut square_inferring: Vec<BoardPosition> = vec![];
    //narrowing down the options using smart inferring
    for square_row in 0..analyzed_board.get_square_size() {
        for square_col in 0..analyzed_board.get_square_size() {
            let square = analyzed_board.get_square(square_row, square_col).unwrap();

            let flatten_options = square.iter().flat_map(|a| match a {
                AnalyzedCell::Value(val) => vec![*val],
                AnalyzedCell::Undetermined(options) => options.to_vec(),
            });

            let single_repeating_values = (1..=size)
                .into_iter()
                .filter(|&n| flatten_options.clone().filter(|&val| val == n).count() == 1);

            let enumerated_square_options = square
                .iter()
                .enumerate()
                .filter(|&(_, analyzed)| matches!(analyzed, AnalyzedCell::Undetermined(_)));

            single_repeating_values
                .clone()
                .map(|value| {
                    // find the square index which value belongs to.
                    let chosen_index = enumerated_square_options
                        .clone()
                        .filter(|&(_, analyzed)| {
                            if let AnalyzedCell::Undetermined(options) = analyzed {
                                return options.iter().find(|&&opt| opt == value).is_some();
                            }
                            false
                        })
                        .map(|x| x.0)
                        .next()
                        .unwrap();
                    return (chosen_index, value);
                })
                .for_each(|(index, value)| {
                    let inner_row = index / square_size;
                    let inner_col = index % square_size;
                    let row = square_row * square_size + inner_row;
                    let col = square_col * square_size + inner_col;

                    square_inferring.push(BoardPosition { value, row, col })
                });
        }
    }
    return square_inferring;
}

pub fn is_full_board(board: &AnalyzedBoard) -> bool {
    board
        .get_rows_flat()
        .into_iter()
        .find(|&cell| matches!(cell, &AnalyzedCell::Undetermined(_)))
        .is_none()
}

pub fn to_board(analyzed_board: &AnalyzedBoard) -> StrResult<Board> {
    let data = analyzed_board.get_rows_flat();
    let has_undetermined = data
        .iter()
        .find(|&&val| matches!(val, &AnalyzedCell::Undetermined(_)))
        .is_some();

    if has_undetermined {
        return Err(format!(
            "Could not convert to numerical board: Found Undetermined items"
        ));
    }

    let data: BoardData<usize> = analyzed_board
        .get_rows()
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| match cell {
                    AnalyzedCell::Value(val) => *val,
                    AnalyzedCell::Undetermined(_) => 0,
                })
                .collect()
        })
        .collect();

    Board::from(&data)
}

pub fn analyze_cell(board: &Board, row: usize, col: usize) -> Option<AnalyzedCell> {
    let value_ref = board.at(row, col)?;
    let value = *value_ref;

    if value > board.get_size() {
        return None;
    }

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
        return None;
    }

    if possible_options.len() == 1 {
        return Some(AnalyzedCell::Value(possible_options[0]));
    }

    Some(AnalyzedCell::Undetermined(possible_options))
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

    let mut analyzed_board = Board::from(&board_size_data).ok()?;

    infer_square_reduction(&analyzed_board)
        .into_iter()
        .for_each(|x| {
            analyzed_board.set(x.row, x.col, AnalyzedCell::Value(x.value));
        });

    return Some(analyzed_board);
}
