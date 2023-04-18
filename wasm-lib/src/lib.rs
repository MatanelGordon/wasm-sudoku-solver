extern crate core;

mod utils;
use sudoku_core::board::{Board, BoardData};
use sudoku_core::solve::simple_solve;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-lib!");
}

#[wasm_bindgen]
pub fn solve(arr: &[usize]) -> Result<Vec<usize>, String> {
    let flat_data = Vec::from(arr);
    let square_size = (flat_data.len() as f32).sqrt().floor() as usize;
    let data = flat_data
        .chunks(square_size)
        .map(|slice| slice.to_vec())
        .collect::<BoardData>();

    let board = Board::from(&data)?;
    let solved = simple_solve(&board)?;

    let solved_flat = solved
        .get_rows_flat()
        .iter()
        .map(|&x| *x)
        .collect::<Vec<_>>();

    return Ok(solved_flat);
}

#[wasm_bindgen]
pub fn is_valid(arr: &[usize]) -> bool{
    let flat_data = Vec::from(arr);
    let square_size = (flat_data.len() as f32).sqrt().floor() as usize;
    let data = flat_data
        .chunks(square_size)
        .map(|slice| slice.to_vec())
        .collect::<BoardData>();

    let board = Board::from(&data);

    return board.is_ok() && board.unwrap().is_valid_numerical();
}