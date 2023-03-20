#[cfg(test)]
pub mod board_suite {
    use crate::board::{Board, BoardData};
    use crate::types::PositionalValue;

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
    fn get_board_at_location() {
        let mut board = Board::new(4).expect("could not create board1");
        let row: usize = 1;
        let col: usize = 1;
        let expected: usize = 1;

        board.set(row, col, expected);

        let received = board.at(row, col).expect("could not use at()");
        assert_eq!(*received, expected);
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
        board.set(0, 1, 100);
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
            board.set(row, col, *value);
        }

        let mut results = board.filter(|v| v == value);

        results.sort_by(|a, b| b.row.cmp(&a.row));

        assert_eq!(&results, &expected);
    }

    #[test]
    fn set_works_on_all_data(){
        let mut board = Board::new(4).expect("Failed initializing board");
        let row:usize = 1;
        let col: usize = 1;
        let expected: usize = 1;


        board.set(row,col, expected);

        let cell = board.at(row, col).unwrap();
        let row_cell = board.get_row(row).unwrap().get(col).unwrap();
        let col_cell = board.get_col(col).unwrap().get(row).unwrap();

        let square_size = board.get_square_size();
        let inner_square_index = (row % square_size) * square_size + (col % square_size);
        let square = board.get_square_of(row, col).unwrap().get(inner_square_index).unwrap();

        println!("row: {row}, col: {col}, ss: {square_size},  index: {inner_square_index}, {:?}", board.get_square_of(row, col).unwrap());

        assert_eq!(cell, &expected);
        assert_eq!(row_cell, &expected);
        assert_eq!(col_cell, &expected);
        assert_eq!(square, &expected);
    }
}

#[cfg(test)]
pub mod analyze_suite {
    use crate::analyze::{analyze_board, analyze_cell, AnalyzedCell, to_board};
    use crate::board::{Board, BoardData};

    #[test]
    fn determine_value() {
        let mut board = Board::new(9).expect("Failed setting up board");
        let row: usize = 2;
        let col: usize = 2;
        let value: usize = 4;
        board.set(row, col, value);
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
}

#[cfg(test)]
pub mod infer_suite {
    use crate::analyze::{analyze_board, AnalyzedCell};
    use crate::board::Board;
    use crate::infer::{BoardPosition, infer_square_reduction, uniq_positions};


    #[test]
    fn uniq_positions_work() {
        let positions = vec![
            BoardPosition {
                row: 0,
                col: 1,
                value: 0,
            },
            BoardPosition {
                row: 0,
                col: 0,
                value: 1,
            },
            BoardPosition {
                row: 0,
                col: 1,
                value: 3,
            },
        ];

        let uniq = uniq_positions(&positions, None);

        assert_eq!(uniq.get(0).unwrap(), &BoardPosition {
            row: 0,
            col: 0,
            value: 1,
        });

        assert_eq!(uniq.get(1).unwrap(), &BoardPosition {
            row: 0,
            col: 1,
            value: 0,
        });
    }


    #[test]
    fn square_inferring() {
        let mut board = Board::new(9).expect("Could not create board");
        board.set(5, 0, 6);
        board.set(8, 1, 6);
        board.set(0, 4, 6);
        board.set(2, 7, 6);
        // should infer that (1,2) is 6 using square inferring
        /*
        +---+---+---+---+---+---+---+---+---+
        | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 0 | 0 |
        +---+---+---+---+---+---+---+---+---+
        | 0 | 0 | X | 0 | 0 | 0 | 0 | 0 | 0 |
        +---+---+---+---+---+---+---+---+---+
        | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 |
        +---+---+---+---+---+---+---+---+---+
        | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
        +---+---+---+---+---+---+---+---+---+
        | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
        +---+---+---+---+---+---+---+---+---+
        | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
        +---+---+---+---+---+---+---+---+---+
        | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
        +---+---+---+---+---+---+---+---+---+
        | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
        +---+---+---+---+---+---+---+---+---+
        | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
        +---+---+---+---+---+---+---+---+---+
        */

        let mut analyzed = analyze_board(&board).expect("could not analyze board");

        infer_square_reduction(&analyzed, 0, 0)
            .into_iter()
            .for_each(|BoardPosition { row, col, value }| {
                analyzed.set(row, col, AnalyzedCell::Value(value));
            });

        assert_eq!(analyzed.at(1, 2).unwrap(), &AnalyzedCell::Value(6));
    }

    #[test]
    fn update_board_after_creation() {
        let mut board = Board::new(4).expect("Could not create board");
        board.set(1, 1, 1);
        board.set(1, 3, 3);
        board.set(0, 2, 2);

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
    }
}

#[cfg(test)]
pub mod solve_suite {}

#[cfg(test)]
pub mod my_tests {
    use rand::Rng;

    #[test]
    fn main_test() {
        let mut rng = rand::thread_rng();

        let n1: usize = rng.gen();
        println!("Hello there {n1}");
    }
}
