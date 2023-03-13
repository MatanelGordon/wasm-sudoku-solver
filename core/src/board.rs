use crate::types::StrResult;
use crate::validators::{is_square, is_square_matrix};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;

pub type BoardData<T = usize> = Vec<Vec<T>>;

pub enum ValidMatrixOptions {
    NonSquareMatrix,
    NonSquareSize(usize),
    Ok(usize),
}

#[derive(Debug, Clone)]
pub struct Board<T = usize> {
    size: usize,
    rows: BoardData<T>,
    // saves the data in a col-based
    cols: BoardData<T>,
    // saves the data in a square based
    squares: BoardData<T>,
}

impl Board<usize> {
    pub fn new(size: usize) -> StrResult<Self> {
        Board::from(&vec![vec![0; size]; size])
    }
}

impl<T> Board<T>
where
    T: Clone,
{
    pub fn from(data: &BoardData<T>) -> StrResult<Self> {
        match Board::<T>::validate_matrix(data) {
            ValidMatrixOptions::NonSquareMatrix => Err(format!(
                "Could not get data that is not square-sized matrix"
            )),
            ValidMatrixOptions::NonSquareSize(size) => Err(format!(
                "Could not get size that is not square number: {size}"
            )),
            ValidMatrixOptions::Ok(size) => {
                let mut board = Board {
                    size,
                    rows: Vec::new(),
                    cols: Vec::new(),
                    squares: Vec::new(),
                };

                board.load_data(&data);

                Ok(board)
            }
        }
    }

    pub fn validate_matrix(data: &BoardData<T>) -> ValidMatrixOptions {
        if !is_square_matrix(data) {
            return ValidMatrixOptions::NonSquareMatrix;
        }

        let size = data.len();

        if !is_square(size) {
            return ValidMatrixOptions::NonSquareSize(size);
        }

        ValidMatrixOptions::Ok(size)
    }

    fn load_data(&mut self, data: &BoardData<T>) {
        let cloned = data.clone();
        let size = cloned.len();
        let size_sqrt = (size as f32).sqrt().floor() as usize;

        self.rows = cloned;

        self.cols = (0..size)
            .map(|i| self.get_col_partial(i, 0, size).unwrap())
            .collect();

        self.squares = (0..size)
            .map(|i| {
                self.get_square_cloned(i / size_sqrt * size_sqrt, i % size_sqrt * size_sqrt)
                    .unwrap()
            })
            .collect();
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_square_size(&self) -> usize {
        (self.size as f32).sqrt().floor() as usize
    }

    pub fn at(&self, row: usize, col: usize) -> Option<&T> {
        let row_list = self.rows.get(row)?;
        let item_ref = row_list.get(col)?;

        Some(item_ref)
    }

    pub fn get_row(&self, index: usize) -> Option<&Vec<T>> {
        self.rows.get(index)
    }

    pub fn get_col(&self, index: usize) -> Option<&Vec<T>> {
        self.cols.get(index)
    }

    pub fn get_square(&self, row: usize, col: usize) -> Option<&Vec<T>> {
        let square_size = self.get_square_size();
        let square_position = row / square_size * square_size + col / square_size;
        self.squares.get(square_position)
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let square_size = self.get_square_size();
        let square_position = row / square_size * square_size + col / square_size;
        let inner_square_index = (row % square_size) * square_size + col % square_size;

        self.rows[row][col] = value.clone();
        self.cols[col][row] = value.clone();
        self.squares[square_position][inner_square_index] = value.clone();
    }

    fn get_square_cloned(&self, row: usize, col: usize) -> Option<Vec<T>> {
        let square_size = self.get_square_size();
        let square_start_x = col / square_size * square_size;
        let square_start_y = row / square_size * square_size;

        let square_data: Vec<T> = (square_start_y..square_start_y + square_size)
            .into_iter()
            .flat_map(|i| {
                self.get_row_partial(i, square_start_x, square_start_x + square_size)
                    .unwrap()
            })
            .collect();

        Some(square_data)
    }

    fn get_row_partial(&self, index: usize, start: usize, end: usize) -> Option<Vec<T>> {
        let row_list = self.rows.get(index)?;
        let row_partial = row_list[start..end].to_vec();
        Some(row_partial)
    }

    fn get_col_partial(&self, index: usize, start: usize, end: usize) -> Option<Vec<T>> {
        let size = self.get_size();

        if index >= size || start > end || end > size {
            return None;
        }

        let col_list: Vec<T> = (start..end)
            .map(|row| self.at(row, index).unwrap().clone())
            .collect();

        Some(col_list)
    }
}
