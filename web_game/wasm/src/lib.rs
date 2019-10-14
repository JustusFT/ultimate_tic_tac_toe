use base_game;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn new_game() -> base_game::game::Game {
    return base_game::game::Game::new();
}

#[wasm_bindgen(catch)]
pub fn new_from_fen(fen: String) -> Result<base_game::game::Game, JsValue> {
    match base_game::fen::new_from_fen(&fen) {
        Ok(x) => Ok(x),
        Err(x) => Err(JsValue::from_str(&x)),
    }
}

#[wasm_bindgen]
pub fn get_fen(game: &base_game::game::Game) -> String {
    return base_game::fen::get_fen(game);
}

#[wasm_bindgen]
pub fn get_game_state(game: &base_game::game::Game) -> JsValue {
    JsValue::from_serde(game).unwrap()
}

#[wasm_bindgen]
pub fn cpu_move(game: &mut base_game::game::Game, depth: i16) {
    let color = if game.turn == base_game::Piece::X {
        1
    } else {
        -1
    };
    let (best_move_a, best_move, _) = base_game::ai::negamax(game, depth, -3000, 3000, color);
    game.make_move(best_move_a.unwrap(), best_move.unwrap());
}
