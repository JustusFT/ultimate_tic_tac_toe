use crate::game;
use crate::Piece;
use rand::Rng;
use rand_pcg::Lcg64Xsh32;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

fn vacant_squares_for_board(game: &game::Game, board_index: u8) -> Vec<(u8, u8)> {
  let mut legal = Vec::new();
  for i in 0..9 {
    let cell = game.local_boards[usize::from(board_index)].board[usize::from(i)];
    if cell == Piece::BLANK {
      legal.push((board_index, i));
    }
  }
  return legal;
}

fn legal_moves(game: &game::Game) -> Vec<(u8, u8)> {
  let mut legal = Vec::new();
  // no legal moves if game is over
  if game.winner != None {
    return legal;
  }
  match game.current_board {
    Some(x) => legal.append(&mut vacant_squares_for_board(game, x)),
    None => {
      for i in 0..9 {
        if game.local_boards[usize::from(i)].claimer != None {
          continue;
        }
        legal.append(&mut vacant_squares_for_board(game, i))
      }
    }
  }
  return legal;
}

#[derive(Debug, PartialEq)]
struct MctsNode {
  games_played: u32,
  games_won: u32,
  player: Piece,
}

fn opponent_for(piece: Piece) -> Piece {
  match piece {
    Piece::X => Piece::O,
    Piece::O => Piece::X,
    _ => Piece::BLANK,
  }
}

fn ucb1(node: &MctsNode, parent_node: &MctsNode) -> f32 {
  let exploration_factor = 1.414;
  let win_rate = node.games_won as f32 / node.games_played as f32;

  return win_rate
    + exploration_factor
      * ((parent_node.games_played as f32).ln() / (node.games_played as f32)).sqrt();
}

impl MctsNode {
  fn new(player: Piece) -> MctsNode {
    MctsNode {
      games_played: 0,
      games_won: 0,
      player,
    }
  }
}

#[wasm_bindgen]
pub struct MctsTree {
  games: HashMap<u64, MctsNode>,
  rng: Lcg64Xsh32,
}

impl MctsTree {
  pub fn new() -> MctsTree {
    MctsTree {
      games: HashMap::new(),
      rng: Lcg64Xsh32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7),
    }
  }

  pub fn evaluate_while<F>(&mut self, game: &mut game::Game, condition: F) -> Option<(u8, u8)>
  where
    F: Fn(i32) -> bool,
  {
    let initial_hash = game.hash;

    // the root represents the opponent, and its immediate children represent the next player to move
    // this way the potential next move's data will represent which move for the current player would lead to more wins
    self
      .games
      .entry(initial_hash)
      .or_insert(MctsNode::new(opponent_for(game.turn)));

    let mut moves_made = 0;
    let mut games_ran = 0;

    'outer: while condition(games_ran) {
      games_ran += 1;
      let mut current_game_line = vec![initial_hash];

      // Selection: traverse down the tree until you need to create a new node
      loop {
        if game.winner != None {
          // this means you reached the end of the game line. the game is over and no further expansion is possible.
          // can't do simulation either, so skip to backpropagation
          // TODO reduce duplicate code
          // Backpropagation: update the results of the simulated game line
          while let Some(line) = current_game_line.pop() {
            let node = self.games.get_mut(&line).unwrap();
            // winning is worth 2 so draws can be worth 1
            node.games_played += 2;
            if game.winner == Some(node.player) {
              node.games_won += 2;
            } else if game.winner == Some(Piece::BLANK) {
              node.games_won += 1;
            }
          }

          // remember to rewind the game back to the initial state
          while moves_made > 0 {
            game.undo_move();
            moves_made -= 1;
          }
          // break 'outer;
          continue 'outer;
        }

        let legal = legal_moves(&game);

        let unvisited_nodes = legal
          .iter()
          .filter(|(a, b)| {
            game.make_move(*a, *b);
            let new_hash = game.hash;
            game.undo_move();
            let was_not_visited = self.games.get(&new_hash) == None;
            return was_not_visited;
          })
          .collect::<Vec<&(u8, u8)>>();

        if unvisited_nodes.len() == 0 {
          // if all the children nodes were visited at least once, then do ubc1 selection to choose which branch to explore
          let mut best_move: Option<(u8, u8)> = None;
          let mut best_score: Option<f32> = None;
          let parent_node = self.games.get(&game.hash).unwrap();

          for j in 0..legal.len() {
            let (a, b) = legal[j];
            game.make_move(a, b);
            let score = ucb1(self.games.get(&game.hash).unwrap(), parent_node);
            if score > best_score.unwrap_or(-1.0) {
              best_score = Some(score);
              best_move = Some((a, b));
            }
            game.undo_move();
          }

          match best_move {
            Some((a, b)) => game.make_move(a, b),
            None => panic!("Failed selection phase"),
          }

          current_game_line.push(game.hash);
          moves_made += 1;
        } else {
          // Expansion: Expand one of the nodes that wasn't visited yet.
          let selected_index = self.rng.gen_range(0, unvisited_nodes.len());
          let (a, b) = *unvisited_nodes[selected_index];
          game.make_move(a, b);
          current_game_line.push(game.hash);
          self
            .games
            .insert(game.hash, MctsNode::new(opponent_for(game.turn)));
          moves_made += 1;
          break;
        }
      }

      // Simulation: make random moves until the game is over
      loop {
        if game.winner != None {
          // Backpropagation: update the results of the simulated game line
          while let Some(line) = current_game_line.pop() {
            let node = self.games.get_mut(&line).unwrap();
            // winning is worth 2 so draws can be worth 1
            node.games_played += 2;
            if game.winner == Some(node.player) {
              node.games_won += 2;
            } else if game.winner == Some(Piece::BLANK) {
              node.games_won += 1;
            }
          }

          // remember to rewind the game back to the initial state
          while moves_made > 0 {
            game.undo_move();
            moves_made -= 1;
          }
          break;
        }
        let legal = legal_moves(&game);
        let selected_index = self.rng.gen_range(0, legal.len());
        let (a, b) = legal[selected_index];
        game.make_move(a, b);
        current_game_line.push(game.hash);
        self
          .games
          .entry(game.hash)
          .or_insert(MctsNode::new(opponent_for(game.turn)));
        moves_made += 1;
      }
    }

    // time to pick the best move and return it
    let legal = legal_moves(&game);
    let mut best_score: Option<f32> = None;
    let mut best_move: Option<(u8, u8)> = None;

    for i in 0..legal.len() {
      let (a, b) = legal[i];
      game.make_move(a, b);
      match self.games.get(&game.hash) {
        Some(node) => {
          let score = node.games_won as f32 / node.games_played as f32;
          if score > best_score.unwrap_or(-1.0) {
            best_score = Some(score);
            best_move = Some((a, b));
          }
        }
        None => {
          // this could be reached if all immediate legal moves weren't evaluated yet
        }
      }
      game.undo_move();
    }

    return best_move;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::fen;
  use std::fs::File;
  use std::io::{prelude::*, BufReader};

  // when refactoring ai, make sure that it does the same moves as it did before
  // the ai will output the same game given same seed and simulation count
  #[test]
  fn ai_did_not_change_its_moves() {
    let mut game = game::Game::new();
    let mut ai = MctsTree::new();
    let file = File::open("test_data/ai_simulation").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
      let cpu_move = ai.evaluate_while(&mut game, |x| x < 100).unwrap();
      game.make_move(cpu_move.0, cpu_move.1);
      let fen = fen::get_fen(&game);
      assert_eq!(line.unwrap(), fen);
    }
  }
}
