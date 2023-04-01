use crate::analyze::{analyze_board, is_full_board, to_board, AnalyzedBoard, AnalyzedCell};
use crate::board::Board;
use crate::infer::{infer_all, infer_positions, InferredPosition};
use crate::types::{PositionalValue, StrResult};
use crate::update::{update_board, update_positions};
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use std::cmp::min;
use std::collections::HashMap;

pub fn simple_solve(board: &Board) -> StrResult<Board> {
    let mut analyzed_board = analyze_board(board)?;
    let mut rng_thread = rand::thread_rng();

    update_board(&mut analyzed_board)?;

    return solve_analyzed(&analyzed_board, &mut rng_thread, 0);
}

/**
@returns return a cell where:
- Least amount of options.
- Sort options by least frequent options.
*/
fn guess_cell<'a>(
    board: &'a AnalyzedBoard,
    thread_rng: &mut ThreadRng,
) -> Option<PositionalValue<Vec<usize>>> {
    let flattened_board = board.get_flat();

    let mut flatten_known = Vec::<usize>::new();
    let mut known_frequency = HashMap::<usize, usize>::new();
    let mut flatten_unknown = Vec::<PositionalValue<&'a Vec<usize>>>::new();
    let mut min_length = board.get_size();

    for cell in flattened_board.iter() {
        match cell.value {
            AnalyzedCell::Value(v) => {
                flatten_known.push(*v);

                let curr = *known_frequency.get(v).unwrap_or(&0);

                known_frequency.insert(*v, curr + 1);
            }
            AnalyzedCell::Undetermined(options) => {
                flatten_unknown.push(PositionalValue {
                    row: cell.row,
                    col: cell.col,
                    value: options,
                });

                min_length = min(min_length, options.len());
            }
        }
    }

    let chosen = flatten_unknown
        .iter()
        .filter(|&x| x.value.len() == min_length)
        .choose(thread_rng)?;

    let mut options = chosen.value.to_vec();

    let get_frequency = |k: &usize| known_frequency.get(k).unwrap_or(&0);

    options.sort_by(|a, b| get_frequency(a).cmp(get_frequency(b)));

    Some(PositionalValue {
        row: chosen.row,
        col: chosen.col,
        value: options,
    })
}

fn solve_analyzed(
    board: &AnalyzedBoard,
    rng_thread: &mut ThreadRng,
    rec: usize,
) -> StrResult<Board> {
    println!("[^] Entering limit {rec}");

    let mut analyzed_board = board.clone();
    let mut positions: Vec<InferredPosition> = infer_all(&analyzed_board)?;
    let mut changed_positions: Vec<(usize, usize)> = Vec::new();

    println!("{}", &analyzed_board);

    // updating the board until there is nothing to update
    while positions.len() > 0 {
        changed_positions.clear();

        for pos in positions.iter() {
            analyzed_board.set(pos.row, pos.col, AnalyzedCell::Value(pos.value))?;
            changed_positions.push((pos.row, pos.col));
        }

        let updated_positions = update_positions(&mut analyzed_board, &changed_positions)?;

        changed_positions.extend(&updated_positions);

        positions = infer_positions(&analyzed_board, &changed_positions)?;
    }

    if is_full_board(&analyzed_board) {
        return to_board(&analyzed_board);
    }

    //Guessing a cell (Backtracking)
    let guess_cell_value = guess_cell(&analyzed_board, rng_thread);

    if let Some(PositionalValue { row, col, value }) = guess_cell_value {
        for opt in value.iter() {
            println!("---trying to solve {row},{col} -> {opt}");
            let mut cloned = board.clone();
            cloned.set(row, col, AnalyzedCell::Value(*opt))?;

            update_positions(&mut cloned, &vec![(row, col)])?;

            let res = solve_analyzed(&cloned, rng_thread, rec + 1);

            match res {
                Ok(res) => {
                    return Ok(res);
                }
                Err(e) => {
                    println!("[-] error in solving when guessing ({row},{col}) ->  {opt} : {e}");
                    continue;
                }
            }
        }

        return Err(format!("Invalid solve for {row},{col}"));
    }

    if is_full_board(&analyzed_board) {
        return to_board(&analyzed_board);
    }

    Err(format!("Solve Failed"))
}
