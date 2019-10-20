use base_game::game::Game;
use base_game::monte_carlo::MctsTree;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen(start)]
pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
extern "C" {
    type Date;

    #[wasm_bindgen(static_method_of = Date)]
    pub fn now() -> f64;
}

#[wasm_bindgen]
pub fn new_game() -> Game {
    return Game::new();
}

#[wasm_bindgen]
pub fn new_mcts() -> MctsTree {
    return MctsTree::new();
}

#[wasm_bindgen(catch)]
pub fn new_from_fen(fen: String) -> Result<Game, JsValue> {
    match base_game::fen::new_from_fen(&fen) {
        Ok(x) => Ok(x),
        Err(x) => Err(JsValue::from_str(&x)),
    }
}

#[wasm_bindgen]
pub fn get_fen(game: &Game) -> String {
    return base_game::fen::get_fen(game);
}

#[wasm_bindgen]
pub fn get_game_state(game: &Game) -> JsValue {
    JsValue::from_serde(game).unwrap()
}

#[wasm_bindgen]
pub fn cpu_move(game: &mut Game, tree: &mut MctsTree) {
    let begin = Date::now();
    let cpu_move = tree.evaluate_while(game, |games_ran| {
        return Date::now() < begin + 10000.0_f64 && games_ran < 10000;
    });
    let (a, b) = cpu_move.unwrap();
    game.make_move(a, b);
}

// #[wasm_bindgen]
// pub fn cpu_move(game: &mut Game, depth: i16) {
//     let color = if game.turn == base_game::Piece::X {
//         1
//     } else {
//         -1
//     };
//     let (best_move_a, best_move, _) = base_game::ai::negamax(game, depth, -3000, 3000, color);
//     game.make_move(best_move_a.unwrap(), best_move.unwrap());
// }
