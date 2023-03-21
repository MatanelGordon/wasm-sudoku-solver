use crate::board::{Board, BoardData};
use crate::infer::BoardPosition;
use crate::types::StrResult;
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

pub fn recalculate_cell(
    board: &AnalyzedBoard,
    row: usize,
    col: usize,
) -> StrResult<Option<AnalyzedCell>> {
    let size = board.get_size();

    if row >= size || col >= size {
        return Err(format!("Index out of bounds: ({row}, {col})"));
    }

    let cell = board.at(row, col).unwrap();

    // we don't mess with values we already defined.
    if cell.is_value() {
        return Ok(None);
    }

    let rows = board.get_row(row).unwrap();
    let cols = board.get_col(col).unwrap();
    let square = board.get_square_of(row, col).unwrap();

    let known: HashSet<usize> = rows
        .iter()
        .chain(cols.iter())
        .chain(square.iter())
        .filter(|x| x.is_value())
        .map(|x| x.get_value().unwrap())
        .collect();

    let options: Vec<usize> = HashSet::<usize>::from_iter(1..=size)
        .difference(&known)
        .map(|x| *x)
        .collect();

    if options.len() == 0 {
        return Err(format!("Invalid cell at ({row},{col})"));
    }

    if options.len() == 1 {
        let value = options.get(0).unwrap();
        return Ok(Some(AnalyzedCell::Value(*value)));
    }

    Ok(Some(AnalyzedCell::Undetermined(options)))
}

pub fn update_axis(
    board: &mut AnalyzedBoard,
    row: usize,
    col: usize,
) -> StrResult<Vec<(usize, usize)>> {
    let size = board.get_size();
    let mut changed_cells: Vec<(usize, usize)> = vec![];

    // updating row and col axis
    for x in 0..size {
        for (_row, _col) in vec![(row, x), (x, col)] {
            let item = recalculate_cell(board, _row, _col)?;

            if item.is_some() {
                board.set(_row, _col, item.unwrap());
                changed_cells.push((_row, _col));
            }
        }
    }

    // updating square
    let square_size = board.get_square_size();
    let square_row = row / square_size;
    let square_col = col / square_size;

    for i in 0..square_size {
        for j in 0..square_size {
            let curr_row = square_row + i;
            let curr_col = square_col + j;
            let item = recalculate_cell(board, curr_row, curr_col)?;
            if item.is_some() {
                board.set(curr_row, curr_col, item.unwrap());
                changed_cells.push((curr_row, curr_col));
            }
        }
    }

    Ok(changed_cells)
}

pub fn update_board(board: &mut AnalyzedBoard) -> StrResult<Vec<(usize, usize)>> {
    let size = board.get_size();
    let mut updated_positions = Vec::<(usize, usize)>::new();

    for row in 0..size {
        for col in 0..size {
            let new_cell = recalculate_cell(board, row, col)?;

            if new_cell.is_some() {
                updated_positions.push((row, col));
            }
        }
    }

    Ok(updated_positions)
}

pub fn analyze_cell(board: &Board, row: usize, col: usize) -> StrResult<AnalyzedCell> {
    let value_ref = board
        .at(row, col)
        .ok_or(format!("could not get cell of ({row},{col})"))?;
    let value = *value_ref;

    if value > board.get_size() {
        return Err(format!(
            "Value of {value} in ({row},{col}) is not valid: Too big"
        ));
    }

    if value > 0 {
        return Ok(AnalyzedCell::Value(value));
    }

    let size = board.get_size();
    let row_list = board
        .get_row(row)
        .ok_or(format!("Could not get row of {row}"))?;
    let col_list = board
        .get_col(col)
        .ok_or(format!("Could not get col of {col}"))?;
    let square_list = board
        .get_square_of(row, col)
        .ok_or(format!("Could not get square_of ({row},{col})"))?;

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
        return Err(format!(
            "cell ({row},{col}) has no options left, hence invalid"
        ));
    }

    if possible_options.len() == 1 {
        return Ok(AnalyzedCell::Value(possible_options[0]));
    }

    Ok(AnalyzedCell::Undetermined(possible_options))
}

pub fn analyze_board(board: &Board) -> StrResult<AnalyzedBoard> {
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

    let mut analyzed_board = Board::from(&board_size_data)?;

    update_board(&mut analyzed_board)?;

    return Ok(analyzed_board);
}
