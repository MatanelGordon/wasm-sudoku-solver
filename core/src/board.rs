use std::cell::RefCell;
use std::collections::HashMap;
use crate::types::StrResult;
use crate::validators::{is_square, is_square_matrix};

pub type BoardData = Vec<Vec<usize>>;

pub enum ValidMatrixOptions {
    NonSquareMatrix,
    NonSquareSize(usize),
    Ok(usize),
}

#[derive(Debug, Clone)]
pub struct Board {
    size: usize,
    rows: BoardData,
    // saves the data in a col-based
    cols: BoardData,
    // saves the data in a square based
    squares: BoardData,
}

impl Board {
    pub fn new(size: usize) -> StrResult<Self> {
        Board::from(&vec![vec![0; size]; size])
    }

    pub fn from(data: &BoardData) -> StrResult<Self> {
        match Board::validate_matrix(data) {
            ValidMatrixOptions::NonSquareMatrix => Err(format!("Could not get data that is not square-sized matrix")),
            ValidMatrixOptions::NonSquareSize(size) => Err(format!("Could not get size that is not square number: {size}")),
            ValidMatrixOptions::Ok(size) => {
                let mut board = Board {
                    size,
                    rows: Vec::new(),
                    cols: Vec::new(),
                    squares: Vec::new(),
                };

                board.load_data(&data);

                println!("{:?}", board);

                Ok(board)
            }
        }
    }

    pub fn validate_matrix(data: &BoardData) -> ValidMatrixOptions {
        if !is_square_matrix(data) {
            return ValidMatrixOptions::NonSquareMatrix;
        }

        let size = data.len();

        if !is_square(size) {
            return ValidMatrixOptions::NonSquareSize(size);
        }

        ValidMatrixOptions::Ok(size)
    }

    fn load_data(&mut self, data: &BoardData) {
        let cloned = data.clone();
        let size = cloned.len();
        let size_sqrt = (size as f32).sqrt().floor() as usize;

        self.rows = cloned;

        self.cols = (0..size)
            .map(|i| self.get_col_partial(i, 0, size).unwrap())
            .collect();

        self.squares = (0..size)
            .map(|i| self.get_square_cloned(i / size_sqrt * size_sqrt, i % size_sqrt * size_sqrt).unwrap())
            .collect();
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_square_size(&self) -> usize {
        (self.size as f32).sqrt().floor() as usize
    }

    pub fn at(&self, row: usize, col: usize) -> StrResult<usize> {
        let row_list = self.rows.get(row).ok_or(format!("Could not get row of {row}"))?;
        let item_ref = row_list.get(col).ok_or(format!("Could not get col of {col}"))?;

        Ok(*item_ref)
    }

    pub fn get_row(&self, index: usize) -> StrResult<&Vec<usize>> {
        self.rows.get(index).ok_or(format!("Could not get row {index}"))
    }

    pub fn get_col(&self, index: usize) -> StrResult<&Vec<usize>> {
        self.cols.get(index).ok_or(format!("Could not get col {index}"))
    }

    pub fn get_square(&self, row: usize, col: usize) -> StrResult<&Vec<usize>>{
        let square_size = self.get_square_size();
        let square_position = row / square_size * square_size + col / square_size;
        self.squares.get(square_position).ok_or(format!("Could not get square for r:{row}, c:{col}"))
    }

    pub fn set(&mut self, row: usize, col: usize, value: usize) {
        let square_size = self.get_square_size();
        let square_position = row / square_size * square_size + col / square_size;
        let inner_square_index = (row % square_size) * square_size + col % square_size;

        self.rows[row][col] = value;
        self.cols[col][row] = value;
        self.squares[square_position][inner_square_index] = value;
    }

    fn get_square_cloned(&self, row: usize, col: usize) -> StrResult<Vec<usize>> {
        let square_size = self.get_square_size();
        let square_start_x = col / square_size * square_size;
        let square_start_y = row / square_size * square_size;

        let square_data: Vec<usize> = (square_start_y..square_start_y + square_size)
            .into_iter()
            .flat_map(
                |i| self.get_row_partial(i, square_start_x, square_start_x + square_size).unwrap()
            ).collect();

        Ok(square_data)
    }

    fn get_row_partial(&self, index: usize, start: usize, end: usize) -> StrResult<Vec<usize>> {
        let row_list = self.rows.get(index).ok_or(format!("Could not get row of {index}"))?;
        let row_partial = row_list[start..end].to_vec();
        Ok(row_partial)
    }

    fn get_col_partial(&self, index: usize, start: usize, end: usize) -> StrResult<Vec<usize>> {
        let res_col_list: Vec<StrResult<usize>> = (start..end).map(|row| self.at(row, index)).collect();

        let operationErr = res_col_list.iter().find(|r| r.is_err());

        if let Some(Err(error)) = operationErr {
            return Err(String::from(error));
        }

        let col_list = res_col_list.into_iter().map(Result::unwrap).collect();

        Ok(col_list)
    }
}