#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;

pub mod ai;
pub mod fen;
pub mod game;
pub mod local_board;
pub mod monte_carlo;

pub const WIN_STATES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum Piece {
    BLANK = 0,
    X = 1,
    O = 2,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
