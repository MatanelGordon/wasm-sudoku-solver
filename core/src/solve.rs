use std::vec;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use crate::types::StrResult;
use crate::analyze::{analyze_board, AnalyzedBoard};
use crate::board::Board;

fn inner_solve(analyzed_board: &AnalyzedBoard, rand_thread: &mut ThreadRng) -> StrResult<AnalyzedBoard>{
    todo!()
}

fn to_board(analyzed_board: &AnalyzedBoard) -> StrResult<Board> {
    todo!()
}

pub fn solve (board: &Board) -> StrResult<Board>{
    let analyzed_board = analyze_board(board).ok_or(format!("Could not analyze board"))?;
    let mut rand_thread = rand::thread_rng();
    let solved = inner_solve(&analyzed_board, &mut rand_thread)?;
    to_board(&solved)
}
