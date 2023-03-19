use crate::analyze::{analyze_board, to_board, AnalyzedBoard};
use crate::board::Board;
use crate::types::StrResult;

fn inner_solve(analyzed_board: &AnalyzedBoard) -> StrResult<AnalyzedBoard> {
    todo!()
}

pub fn solve(board: &Board) -> StrResult<Board> {
    let analyzed_board = analyze_board(board).ok_or(format!("Could not analyze board"))?;
    let solved = inner_solve(&analyzed_board)?;
    to_board(&solved)
}
