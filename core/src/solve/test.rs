use crate::board::Board;
use crate::solve::simple_solve;
use crate::types::StrResult;

#[test]
fn should_solve_example_board() -> StrResult<()> {
    let mut board = Board::new(9)?;
    {
        board.set(0, 2, 1)?;
        board.set(1, 6, 1)?;
        board.set(7, 1, 1)?;
        board.set(5, 1, 2)?;
        board.set(6, 3, 3)?;
        board.set(7, 2, 3)?;
        board.set(3, 2, 4)?;
        board.set(4, 3, 4)?;
        board.set(8, 8, 4)?;
        board.set(0, 1, 5)?;
        board.set(4, 8, 5)?;
        board.set(6, 7, 5)?;
        board.set(7, 4, 5)?;
        board.set(0, 4, 7)?;
        board.set(3, 1, 7)?;
        board.set(6, 8, 7)?;
        board.set(1, 3, 8)?;
        board.set(2, 0, 8)?;
        board.set(3, 7, 8)?;
        board.set(2, 4, 9)?;
        board.set(4, 6, 9)?;
        board.set(8, 0, 9)?;
    }

    let solved = simple_solve(&board)?;

    assert_eq!(solved.is_full(), true);
    Ok(())
}

#[test]
fn should_solve_empty_board() -> StrResult<()> {
    let board = Board::new(9)?;

    let solved = simple_solve(&board)?;

    assert_eq!(solved.is_full(), true);
    Ok(())
}
