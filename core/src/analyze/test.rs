use crate::analyze::{analyze_board, analyze_cell, to_board, AnalyzedCell};
use crate::board::{Board, BoardData};
use crate::types::StrResult;

#[test]
fn determine_value() {
    let mut board = Board::new(9).expect("Failed setting up board1");
    let row: usize = 2;
    let col: usize = 2;
    let value: usize = 4;
    board
        .set(row, col, value)
        .expect("Could not set in determine_value");
    let analyzed = analyze_cell(&board, row, col).expect("Failed analyzing cell");
    assert_eq!(analyzed, AnalyzedCell::Value(value));
}

#[test]
fn convert_to_board() {
    let board_data: BoardData<usize> = vec![vec![6; 9]; 9];
    let board = Board::from(&board_data).expect("Failed to create board1");
    let analyzed_board = analyze_board(&board).expect("Failed to analyze board1");
    let new_board = to_board(&analyzed_board).expect("Failed to run to_board");
    assert_eq!(new_board.get_rows(), &board_data);
}

#[test]
fn fail_to_board() {
    let empty_board = Board::new(9).expect("Failed to create board1");
    let analyzed_board = analyze_board(&empty_board).expect("Failed to analyze board1");
    assert!(to_board(&analyzed_board).is_err());
}

#[test]
fn update_board_after_creation() -> StrResult<()> {
    let mut board = Board::new(4).expect("Could not create board1");
    board.set(1, 1, 1)?;
    board.set(1, 3, 3)?;
    board.set(0, 2, 2)?;

    /*
    Must infer X is 2 due to row inferring
    +---+---+---+---+
    | 0 | 0 | 2 | 0 |
    +---+---+---+---+
    | 0 | 1 | X | 3 |
    +---+---+---+---+
    | 0 | 0 | 0 | 0 |
    +---+---+---+---+
    | 0 | 0 | 0 | 0 |
    +---+---+---+---+

    Should be:
    X = 4
     */

    let analyzed = analyze_board(&board).expect("Could not analyze board1");

    let x = analyzed.at(1, 2).unwrap();

    assert_eq!(x.get_value(), Some(4));
    Ok(())
}
