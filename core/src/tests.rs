#[cfg(test)]
pub mod tests {
    use crate::board::{Board, BoardData};

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
        let data: BoardData = vec![vec![1,1,0,1],vec![1,1,0,1],vec![0,1,0,1],vec![0,1,0,1]];
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

        println!("{:?}", board);

        let received = board.at(row,col).expect("could not use at()");
        assert_eq!(received, expected);
    }

    // #[test]
    // fn analyze_cell_test(){
    //     let data: BoardData = vec![vec![1,2,3,0],vec![3,0,0,0],vec![0,0,0,0],vec![0,0,0,0]];
    //     let board = Board::from(&data).expect("Failed creating board");
    //
    //     let cell = analyze_cell(&board, 1,1);
    // }
}
