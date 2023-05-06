use crate::board::{Board, BoardData};
use crate::types::{PositionalValue, StrResult};

#[test]
fn create_empty_board() {
    let board = Board::new(9);
    assert!(board.is_ok())
}

#[test]
fn fail_create_empty_board() {
    let board = Board::new(13);
    assert!(board.is_err())
}

#[test]
fn from_defined_vectors() {
    let data: BoardData = vec![
        vec![1, 1, 0, 1],
        vec![1, 1, 0, 1],
        vec![0, 1, 0, 1],
        vec![0, 1, 0, 1],
    ];
    let board = Board::from(&data);
    assert!(board.is_ok())
}

#[test]
fn get_board_at_location() -> StrResult<()> {
    let mut board = Board::new(4).expect("could not create board1");
    let row: usize = 1;
    let col: usize = 1;
    let expected: usize = 1;

    board.set(row, col, expected)?;

    let received = board.at(row, col).expect("could not use at()");
    assert_eq!(*received, expected);
    Ok(())
}

#[test]
fn flat_row_correct_length() {
    let size: usize = 4;
    let board = Board::new(size).expect("could not create board1");
    assert_eq!(board.get_rows_flat().len(), size.pow(2));
}

#[test]
fn board_is_full() {
    let data: BoardData<usize> = vec![
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
    ];
    let board: Board = Board::from(&data).expect("Failed creating board1");
    assert!(board.is_full())
}

#[test]
fn board_is_not_full() {
    let data: BoardData<usize> = vec![
        vec![1, 1, 1, 1],
        vec![1, 1, 0, 1],
        vec![1, 0, 1, 1],
        vec![1, 1, 1, 1],
    ];
    let board: Board = Board::from(&data).expect("Failed creating board1");
    assert!(!board.is_full())
}

#[test]
fn board_filter_specific_value() {
    let mut board = Board::new(4).expect("Failed initializing board1");
    let value = &6;
    let expected: Vec<PositionalValue<&usize>> = vec![
        PositionalValue {
            value,
            row: 1,
            col: 2,
        },
        PositionalValue {
            value,
            row: 0,
            col: 0,
        },
        PositionalValue {
            value,
            row: 0,
            col: 1,
        },
    ];

    for &PositionalValue { row, col, value } in expected.iter() {
        board
            .set(row, col, *value)
            .expect("Failed Setting position value");
    }

    let mut results = board.filter(|v| v == value);

    results.sort_by(|a, b| b.row.cmp(&a.row));

    assert_eq!(&results, &expected);
}

#[test]
fn set_works_on_all_data() {
    let mut board = Board::new(4).expect("Failed initializing board1");
    let row: usize = 1;
    let col: usize = 1;
    let expected: usize = 1;

    board.set(row, col, expected).expect("Could not set value");

    let cell = board.at(row, col).unwrap();
    let row_cell = board.get_row(row).unwrap().get(col).unwrap();
    let col_cell = board.get_col(col).unwrap().get(row).unwrap();

    let square_size = board.get_square_size();
    let inner_square_index = (row % square_size) * square_size + (col % square_size);
    let square = board
        .get_square_of(row, col)
        .unwrap()
        .get(inner_square_index)
        .unwrap();

    assert_eq!(cell, &expected);
    assert_eq!(row_cell, &expected);
    assert_eq!(col_cell, &expected);
    assert_eq!(square, &expected);
}

#[test]
fn set_throws_err_in_invalid_row() {
    let mut board = Board::new(4).expect("Could not create board1");

    let result = board.set(5, 1, 2);

    assert_eq!(result.is_err(), true);
}

#[test]
fn set_throws_err_in_invalid_col() {
    let mut board = Board::new(4).expect("Could not create board1");

    let result = board.set(1, 10, 2);

    assert_eq!(result.is_err(), true);
}
