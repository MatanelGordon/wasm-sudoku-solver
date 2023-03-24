use crate::analyze::{AnalyzedBoard, AnalyzedCell};
use crate::types::StrResult;
use std::collections::HashSet;

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

pub fn update_square(
    board: &mut AnalyzedBoard,
    row: usize,
    col: usize,
) -> StrResult<Vec<(usize, usize)>> {
    // updating square
    let square_size = board.get_square_size();
    let square_row = row / square_size;
    let square_col = col / square_size;
    let mut changed: Vec<(usize, usize)> = Vec::new();

    for i in 0..square_size {
        for j in 0..square_size {
            let curr_row = square_row + i;
            let curr_col = square_col + j;
            update_position(board, curr_row, curr_col, &mut changed)?;
        }
    }

    Ok(changed)
}

fn update_position<'a>(
    board: &'a mut AnalyzedBoard,
    row: usize,
    col: usize,
    positions_vec: &'a mut Vec<(usize, usize)>,
) -> StrResult<bool> {
    let new_cell = recalculate_cell(board, row, col)?;
    let mut has_changed = false;

    if new_cell.is_some() {
        let value = new_cell.unwrap();

        if value.is_value() {
            println!("Found value: {row},{col} = {:?}", &value);
            positions_vec.push((row, col));
            has_changed = true;
        }

        board.set(row, col, value);
    }

    Ok(has_changed)
}

pub fn update_axis(
    board: &mut AnalyzedBoard,
    axis: usize,
    is_row: bool,
) -> StrResult<Vec<(usize, usize)>> {
    let size = board.get_size();
    let mut changed_cells: Vec<(usize, usize)> = vec![];

    // updating either row / col
    for x in 0..size {
        if is_row {
            update_position(board, axis, x, &mut changed_cells)?;
        } else {
            update_position(board, x, axis, &mut changed_cells)?;
        }
    }

    Ok(changed_cells)
}

pub fn update_board(board: &mut AnalyzedBoard) -> StrResult<Vec<(usize, usize)>> {
    let size = board.get_size();
    let mut updated_positions: Vec<(usize, usize)> = Vec::new();

    for row in 0..size {
        for col in 0..size {
            update_position(board, row, col, &mut updated_positions)?;
        }
    }

    Ok(updated_positions)
}

pub fn update_positions<'a>(
    board: &'a mut AnalyzedBoard,
    positions: &'a Vec<(usize, usize)>,
) -> StrResult<Vec<(usize, usize)>> {
    let square_size = board.get_square_size();
    let mut checked_rows: Vec<usize> = Vec::new();
    let mut checked_cols: Vec<usize> = Vec::new();
    let mut checked_squares: Vec<usize> = Vec::new();
    let mut changed_positions: Vec<(usize, usize)> = Vec::new();

    for &(row, col) in positions.iter() {
        if !checked_rows.contains(&row) {
            checked_rows.push(row);
            let positions = update_axis(board, row, true)?;
            changed_positions.extend(positions);
        }

        if !checked_cols.contains(&col) {
            checked_cols.push(col);
            let positions = update_axis(board, col, false)?;
            changed_positions.extend(positions);
        }

        let square_index = row * square_size + col;
        if checked_squares.contains(&square_index) {
            checked_squares.push(square_index);
            let positions = update_square(board, row, col)?;
            changed_positions.extend(positions);
        }
    }

    changed_positions.sort_by(|&(r1, c1), &(r2, c2)| {
        ((r1 as isize - r2 as isize) * square_size as isize + (c1 as isize - c2 as isize)).cmp(&0)
    });
    changed_positions.dedup_by(|(r1, c1), (r2, c2)| r1 == r2 && c1 == c2);

    Ok(changed_positions)
}
