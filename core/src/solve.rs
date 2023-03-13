use crate::board::Board;
use crate::types::StrResult;
use std::vec;

pub fn solve(board: &Board) -> StrResult<Vec<Board>> {
    let some_board = Board::<usize>::new(4)?;
    return Ok(vec![some_board]);
}
