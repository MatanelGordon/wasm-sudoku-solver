use crate::board::{Board, BoardData};
use crate::types::{PositionalValue, StrResult};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum AnalyzedCell {
    Value(usize),
    Undetermined(Vec<usize>),
}

impl AnalyzedCell {
    pub fn is_value(&self) -> bool {
        matches!(self, AnalyzedCell::Value(_))
    }

    pub fn is_undetermined(&self) -> bool {
        matches!(self, AnalyzedCell::Undetermined(_))
    }

    pub fn get_value(&self) -> Option<usize> {
        match self {
            AnalyzedCell::Value(v) => Some(*v),
            AnalyzedCell::Undetermined(_) => None,
        }
    }

    pub fn get_undetermined(&self) -> Option<&Vec<usize>> {
        match self {
            AnalyzedCell::Value(_) => None,
            AnalyzedCell::Undetermined(values) => Some(values),
        }
    }
}

pub type AnalyzedBoardData = BoardData<AnalyzedCell>;
pub type AnalyzedBoard = Board<AnalyzedCell>;

struct BoardPosition {
    pub row: usize,
    pub col: usize,
    pub value: usize,
}

fn get_single_repeating_values(cells: &Vec<AnalyzedCell>) -> Vec<(usize, usize)> {
    let flatten_options = cells.iter().flat_map(|a| match a {
        AnalyzedCell::Value(v) => vec![*v],
        AnalyzedCell::Undetermined(opt) => opt.to_vec(),
    });

    flatten_options
        .clone()
        .collect::<HashSet<usize>>()
        .into_iter()
        .filter(|&x| flatten_options.clone().filter(|&x1| x == x1).count() == 1)
        .map(|single_value| {
            let chosen_index = cells
                .iter()
                .position(|cell| {
                    if let AnalyzedCell::Undetermined(options) = cell {
                        return options.iter().find(|&&opt| opt == single_value).is_some();
                    }
                    false
                })
                .unwrap();
            return (chosen_index, single_value);
        })
        .collect()
}

fn infer_square_reduction_all(analyzed_board: &AnalyzedBoard) -> Vec<BoardPosition> {
    let square_size = analyzed_board.get_square_size();

    (0..square_size)
        .flat_map(move |row| {
            (0..square_size)
                .flat_map(move |col| infer_square_reduction(analyzed_board, row,col))
        })
        .collect()
}

fn infer_square_reduction(
    board: &AnalyzedBoard,
    square_row: usize,
    square_col: usize,
) -> Vec<BoardPosition> {
    let square = board.get_square(square_row, square_col);
    let square_size = board.get_square_size();

    if square.is_none() {
        return vec![];
    }

    return get_single_repeating_values(square.unwrap())
        .into_iter()
        .map(|(index, value)| {
            let inner_row = index / square_size;
            let inner_col = index % square_size;
            let row = square_row * square_size + inner_row;
            let col = square_col * square_size + inner_col;
            return BoardPosition { row, col, value };
        })
        .collect();
}

fn infer_row_reduction(rows: &Vec<AnalyzedCell>, index: usize) -> Vec<BoardPosition> {
    get_single_repeating_values(rows)
        .into_iter()
        .map(|(curr_index, value)| BoardPosition {
            row: index,
            col: curr_index,
            value,
        })
        .collect()
}

fn infer_col_reduction(cols: &Vec<AnalyzedCell>, index: usize) -> Vec<BoardPosition> {
    get_single_repeating_values(cols)
        .into_iter()
        .map(|(curr_index, value)| BoardPosition {
            col: index,
            row: curr_index,
            value,
        })
        .collect()
}

fn update_partial_analyzed_board(board: &mut AnalyzedBoard, row: usize, col: usize) -> bool {
    let square_opt = board.get_square(row, col);
    let rows_opt = board.get_row(row);
    let cols_opt = board.get_col(col);
    let mut has_changed = false;

    if cols_opt.is_none() || rows_opt.is_none() || square_opt.is_none() {
        return false;
    }

    let rows = rows_opt.unwrap();
    let cols = cols_opt.unwrap();
    let square = square_opt.unwrap();

    has_changed
}

fn update_analyzed_board(board: &mut AnalyzedBoard) -> bool {
    let mut changed = false;
    let size = board.get_size();

    for row in 0..size {
        for col in 0..size {
            let current = board.at(row, col).unwrap();
            if current.is_undetermined() {
                let mut new_options = HashSet::<usize>::from_iter(1..=size);
                let current_options = current.get_undetermined().unwrap();
                let groups = vec![
                    board.get_row(row).unwrap(),
                    board.get_col(row).unwrap(),
                    board.get_square_1d(row).unwrap(),
                ];

                for &arr in groups.iter() {
                    for cell in arr {
                        if cell.is_value() {
                            let cell_value = &cell.get_value().unwrap();
                            new_options.remove(cell_value);
                        }
                    }
                }

                if new_options.len() < current_options.len() {
                    let value: AnalyzedCell = if new_options.len() == 1 {
                        AnalyzedCell::Value(*(new_options.get(&0).unwrap()))
                    } else {
                        AnalyzedCell::Undetermined(Vec::from_iter(new_options))
                    };

                    board.set(row, col, value);
                    changed = true;
                }
            }
        }
    }

    changed
}

pub fn is_full_board(board: &AnalyzedBoard) -> bool {
    board
        .get_rows_flat()
        .into_iter()
        .find(|cell| cell.is_undetermined())
        .is_none()
}

pub fn to_board(analyzed_board: &AnalyzedBoard) -> StrResult<Board> {
    let data = analyzed_board.get_rows_flat();
    let has_undetermined = data
        .iter()
        .find(|&&val| matches!(val, &AnalyzedCell::Undetermined(_)))
        .is_some();

    if has_undetermined {
        return Err(format!(
            "Could not convert to numerical board: Found Undetermined items"
        ));
    }

    let data: BoardData<usize> = analyzed_board
        .get_rows()
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| match cell {
                    AnalyzedCell::Value(val) => *val,
                    AnalyzedCell::Undetermined(_) => 0,
                })
                .collect()
        })
        .collect();

    Board::from(&data)
}

pub fn analyze_cell(board: &Board, row: usize, col: usize) -> Option<AnalyzedCell> {
    let value_ref = board.at(row, col)?;
    let value = *value_ref;

    if value > board.get_size() {
        return None;
    }

    if value > 0 {
        return Some(AnalyzedCell::Value(value));
    }

    let size = board.get_size();
    let row_list = board.get_row(row)?;
    let col_list = board.get_col(col)?;
    let square_list = board.get_square(row, col)?;

    let mut all_axis_options: Vec<usize> = Vec::new();
    let all_options: HashSet<usize> = (1..=size).collect::<HashSet<_>>();

    all_axis_options.extend(row_list);
    all_axis_options.extend(col_list);
    all_axis_options.extend(square_list);

    let occupied_options: HashSet<usize> = all_axis_options
        .into_iter()
        .filter(|v| *v != 0)
        .collect::<HashSet<_>>();

    let possible_options = all_options
        .difference(&occupied_options)
        .map(|val| *val)
        .collect::<Vec<_>>();

    if possible_options.len() == 0 {
        return None;
    }

    if possible_options.len() == 1 {
        return Some(AnalyzedCell::Value(possible_options[0]));
    }

    Some(AnalyzedCell::Undetermined(possible_options))
}

pub fn analyze_board(board: &Board) -> Option<AnalyzedBoard> {
    let size = board.get_size();
    let mut board_size_data: AnalyzedBoardData = vec![];

    for row in 0..size {
        let mut row_list: Vec<AnalyzedCell> = vec![];
        for col in 0..size {
            let cell = analyze_cell(board, row, col)?;
            row_list.push(cell);
        }
        board_size_data.push(row_list);
    }

    let mut analyzed_board = Board::from(&board_size_data).ok()?;

    infer_square_reduction_all(&analyzed_board)
        .into_iter()
        .for_each(|x| {
            analyzed_board.set(x.row, x.col, AnalyzedCell::Value(x.value));
        });

    return Some(analyzed_board);
}
