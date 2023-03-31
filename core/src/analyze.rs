use crate::board::{Board, BoardData};
use crate::types::StrResult;
use crate::update::{update_board};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

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

impl Display for AnalyzedCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalyzedCell::Value(v) => write!(f, "{v}"),
            AnalyzedCell::Undetermined(_) => write!(f, " "),
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

    let x = update_board(&mut analyzed_board)?;
    println!("updated from analysis: \n {:?}", &x);
    return Ok(analyzed_board);
}
