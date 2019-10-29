use base_game::game;
use base_game::monte_carlo;

fn main() {
  // make new game
  // simulate cpu vs cpu game with constant seed, 1000 sims per player
  // until game is over
  let mut game = game::Game::new();
  let mut ai = monte_carlo::MctsTree::new();
  while game.winner == None {
    let cpu_move = ai.evaluate_while(&mut game, |x| x < 1000).unwrap();
    game.make_move(cpu_move.0, cpu_move.1);
  }
}
