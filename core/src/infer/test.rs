use crate::analyze::analyze_board;
use crate::board::Board;
use crate::infer::{infer_square_of, InferredPosition};
use crate::types::StrResult;

#[test]
fn should_infer_square() -> StrResult<()> {
    let mut board = Board::new(9)?;
    let expected = InferredPosition {
        row: 0,
        col: 2,
        value: 6,
    };

    let expected1 = InferredPosition {
        row: 0,
        col: 1,
        value: 7,
    };

    // setting up to infer (0,2) -> 6
    {
        board.set(5, 0, 6)?;
        board.set(8, 1, 6)?;
        board.set(2, 2, 1)?;
        board.set(1, 2, 2)?;
    }

    //setting up to infer (0,1) -> 7
    {
        board.set(2, 7, 7)?;
        board.set(1, 5, 7)?;
        board.set(0, 0, 3)?;
    }

    let analyzed = analyze_board(&board)?;

    let inferred = infer_square_of(&analyzed, 0, 2)?;

    assert_eq!(inferred.len(), 2);

    assert_eq!(inferred.first(), Some(&expected));
    assert_eq!(inferred.get(1), Some(&expected1));

    Ok(())
}
