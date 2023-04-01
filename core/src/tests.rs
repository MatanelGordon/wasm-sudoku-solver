#[cfg(test)]
pub mod board_suite {
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
        let mut board = Board::new(4).expect("could not create board");
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
        let board = Board::new(size).expect("could not create board");
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
        let board: Board = Board::from(&data).expect("Failed creating board");
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
        let board: Board = Board::from(&data).expect("Failed creating board");
        assert!(!board.is_full())
    }

    #[test]
    fn board_invalid_numerical() {
        let mut board = Board::new(4).expect("Failed initializing board");
        board.set(0, 1, 100).expect("Failed setting value");
        assert!(!board.is_valid_numerical());
    }

    #[test]
    fn board_valid_numerical() {
        let board = Board::new(4).expect("Failed initializing board");
        assert!(board.is_valid_numerical());
    }

    #[test]
    fn board_filter_specific_value() {
        let mut board = Board::new(4).expect("Failed initializing board");
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
        let mut board = Board::new(4).expect("Failed initializing board");
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
        let mut board = Board::new(4).expect("Could not create board");

        let result = board.set(5, 1, 2);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn set_throws_err_in_invalid_col() {
        let mut board = Board::new(4).expect("Could not create board");

        let result = board.set(1, 10, 2);

        assert_eq!(result.is_err(), true);
    }
}

#[cfg(test)]
pub mod analyze_suite {
    use crate::analyze::{analyze_board, analyze_cell, to_board, AnalyzedCell};
    use crate::board::{Board, BoardData};
    use crate::types::StrResult;

    #[test]
    fn determine_value() {
        let mut board = Board::new(9).expect("Failed setting up board");
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
        let board = Board::from(&board_data).expect("Failed to create board");
        let analyzed_board = analyze_board(&board).expect("Failed to analyze board");
        let new_board = to_board(&analyzed_board).expect("Failed to run to_board");
        assert_eq!(new_board.get_rows(), &board_data);
    }

    #[test]
    fn fail_to_board() {
        let empty_board = Board::new(9).expect("Failed to create board");
        let analyzed_board = analyze_board(&empty_board).expect("Failed to analyze board");
        assert!(to_board(&analyzed_board).is_err());
    }

    #[test]
    fn update_board_after_creation() -> StrResult<()> {
        let mut board = Board::new(4).expect("Could not create board");
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

        let analyzed = analyze_board(&board).expect("Could not analyze board");

        let x = analyzed.at(1, 2).unwrap();

        assert_eq!(x.get_value(), Some(4));
        Ok(())
    }
}

#[cfg(test)]
pub mod infer_suite {
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

        let mut analyzed = analyze_board(&board)?;

        let inferred = infer_square_of(&analyzed, 0, 2)?;

        assert_eq!(inferred.len(), 2);

        assert_eq!(inferred.first(), Some(&expected));
        assert_eq!(inferred.get(1), Some(&expected1));

        Ok(())
    }
}

#[cfg(test)]
pub mod solve_suite {}

#[cfg(test)]
pub mod my_tests {
    use crate::board::Board;
    use crate::solve::simple_solve;
    use crate::types::StrResult;

    #[test]
    fn main_test() -> StrResult<()> {
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

        println!("{}", &solved);
        Ok(())
    }
}
