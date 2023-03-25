use crate::analyze::{analyze_board, to_board, AnalyzedCell};
use crate::board::Board;
use crate::infer::{infer_all, infer_positions, InferredPosition};
use crate::types::StrResult;
use crate::update::{update_board, update_positions};

pub fn simple_solve(board: &Board) -> StrResult<Board> {
    println!("{}", board);
    let mut analyzed_board = analyze_board(board)?;
    println!("{}", &analyzed_board);
    let mut positions: Vec<InferredPosition> = infer_all(&analyzed_board);
    let mut curr_searched_positions: Vec<(usize, usize)> = Vec::new();

    while positions.len() > 0 {
        println!("positions: {:?}", &positions);

        for pos in positions.iter() {
            analyzed_board.set(pos.row, pos.col, AnalyzedCell::Value(pos.value))?;
        }

        let inf_pos: Vec<(usize, usize)> = positions.iter().map(|x| (x.row, x.col)).collect();
        println!("inferred: {:?}", &inf_pos);

        let updated_pos = update_positions(&mut analyzed_board, &inf_pos)?;
        println!("updated from infer: {:?}", &updated_pos);

        curr_searched_positions.clear();
        curr_searched_positions.extend(&inf_pos);
        curr_searched_positions.extend(&updated_pos);

        positions = infer_positions(&analyzed_board, &curr_searched_positions);

        println!("{}", &analyzed_board);
    }

    return Board::new(9);
}
