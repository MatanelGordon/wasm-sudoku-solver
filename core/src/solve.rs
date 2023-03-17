use crate::analyze::{analyze_board, AnalyzedBoard, AnalyzedCell};
use crate::board::{Board, BoardData};
use crate::types::StrResult;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use std::vec;

fn inner_solve(
    analyzed_board: &AnalyzedBoard,
    rand_thread: &mut ThreadRng,
) -> StrResult<AnalyzedBoard> {
    todo!()
}

fn get_value(cell: &AnalyzedCell) -> usize {
    match cell {
        AnalyzedCell::Value(val) => *val,
        AnalyzedCell::Undetermined(_) => 0,
    }
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
        .map(|row| row.iter().map(get_value).collect())
        .collect();

    Board::from(&data)
}

pub fn solve(board: &Board) -> StrResult<Board> {
    let analyzed_board = analyze_board(board).ok_or(format!("Could not analyze board"))?;
    let mut rand_thread = rand::thread_rng();
    let solved = inner_solve(&analyzed_board, &mut rand_thread)?;
    to_board(&solved)
}
