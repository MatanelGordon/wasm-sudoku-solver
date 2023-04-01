use crate::analyze::{analyze_board, is_full_board, to_board, AnalyzedBoard, AnalyzedCell};
use crate::board::Board;
use crate::infer::{infer_all, infer_positions, InferredPosition};
use crate::types::{PositionalValue, StrResult};
use crate::update::update_positions;
use rand::rngs::ThreadRng;
use rand::seq::{IteratorRandom};
use std::cmp::min;
use std::collections::{HashMap};

pub fn simple_solve(board: &Board) -> StrResult<Board> {
    let analyzed_board = analyze_board(board)?;
    let mut rng_thread = rand::thread_rng();

    return solve_analyzed(&analyzed_board, &mut rng_thread);
}

// will return a cell where:
// least amount of options
// including least frequent options
fn guess_cell<'a>(
    board: &'a AnalyzedBoard,
    thread_rng: &mut ThreadRng,
) -> Option<PositionalValue<&'a Vec<usize>>> {
    let flattened_board = board.get_flat();

    let mut flatten_known = Vec::<usize>::new();
    let mut known_repeat = HashMap::<usize, usize>::new();
    let mut flatten_unknown = Vec::<PositionalValue<&'a Vec<usize>>>::new();
    let mut min_length = board.get_size();

    for cell in flattened_board.iter() {
        match cell.value {
            AnalyzedCell::Value(v) => {
                flatten_known.push(*v);

                let curr = known_repeat.get(v).unwrap_or(&0);

                known_repeat.insert(*v, *curr + 1);
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
        .filter(|x| x.value.len() == min_length)
        .choose(thread_rng)?;

    return Some(chosen.clone());
}

fn solve_analyzed(board: &AnalyzedBoard, rng_thread: &mut ThreadRng) -> StrResult<Board> {
    let mut analyzed_board = board.clone();
    let mut positions: Vec<InferredPosition> = infer_all(&analyzed_board)?;
    let mut changed_positions: Vec<(usize, usize)> = Vec::new();

    // updating the board until there is nothing to update
    while positions.len() > 0 {
        for pos in positions.iter() {
            analyzed_board.set(pos.row, pos.col, AnalyzedCell::Value(pos.value))?;
        }

        let inferred_positions: Vec<(usize, usize)> =
            positions.iter().map(|x| (x.row, x.col)).collect();

        let updated_positions = update_positions(&mut analyzed_board, &inferred_positions)?;

        changed_positions.clear();
        changed_positions.extend(&inferred_positions);
        changed_positions.extend(&updated_positions);

        positions = infer_positions(&analyzed_board, &changed_positions)?;

        println!("{}", &analyzed_board);
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
            let res = solve_analyzed(&cloned, rng_thread);

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
