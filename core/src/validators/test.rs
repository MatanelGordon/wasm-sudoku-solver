use crate::board::Board;
use crate::types::StrResult;
use crate::validators::is_valid_sudoku;

#[test]
fn should_invalid_board() -> StrResult<()> {
    let mut board = Board::new(9)?;
    board.set(0, 0, 1)?;
    board.set(0, 1, 1)?;
    let is_valid = is_valid_sudoku(&board);

    assert_eq!(is_valid, false);
    Ok(())
}
