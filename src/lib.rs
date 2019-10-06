#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;

mod game;
mod game_ai;

#[wasm_bindgen]
pub fn new_game() -> game::Game {
  return game::Game::new();
}

#[wasm_bindgen]
pub fn get_game_state(game: &game::Game) -> JsValue {
  JsValue::from_serde(game).unwrap()
}

#[wasm_bindgen]
pub fn cpu_move(game: &mut game::Game, depth: i16) {
  let color = if game.turn == game::Piece::X { 1 } else { -1 };
  let (best_move_a, best_move, _) = game_ai::negamax(game, depth, -3000, 3000, color);
  game.make_move(best_move_a.unwrap(), best_move.unwrap());
}
