use base_game::fen;
use base_game::game;
use base_game::monte_carlo;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open("test_data/ai_simulation")
    .unwrap();

  // make new game
  // simulate cpu vs cpu game with constant seed, 1000 sims per player
  // until game is over
  let mut game = game::Game::new();
  let mut ai = monte_carlo::MctsTree::new();
  while game.winner == None {
    let cpu_move = ai.evaluate_while(&mut game, |x| x < 100).unwrap();
    game.make_move(cpu_move.0, cpu_move.1);
    let fen = fen::get_fen(&game);
    if let Err(e) = writeln!(file, "{}", fen) {
      eprintln!("Couldn't write to file: {}", e);
    }
  }
}
