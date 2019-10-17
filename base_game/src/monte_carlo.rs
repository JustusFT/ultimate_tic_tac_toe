use crate::game;
use crate::Piece;
use rand::prelude::*;
use std::collections::HashMap;
use std::convert::TryInto;

struct ZorbistHasher {
  piece_table: [[u64; 81]; 3],
  current_board_table: [u64; 10],
}

impl ZorbistHasher {
  // there are 81 squares, each square has 3 possible states: X, O, or blank.
  // there are 10 (1+9) total possible states for the current board to be played on:
  // - one for when all boards can be played
  // - nine when a single board can be played
  // we hash using a table for the 81 squares + a table for the possible boards
  fn new() -> ZorbistHasher {
    let mut rng = rand::thread_rng();

    let mut piece_table: [[u64; 81]; 3] = [[0; 81]; 3];
    let mut current_board_table: [u64; 10] = [0; 10];

    for i in 0..3 {
      for j in 0..81 {
        let random_int: u64 = rng.gen();
        piece_table[i][j] = random_int;
      }
    }

    for i in 0..10 {
      let random_int: u64 = rng.gen();
      current_board_table[i] = random_int;
    }

    ZorbistHasher {
      piece_table,
      current_board_table,
    }
  }

  fn hash(&self, game: &game::Game) -> u64 {
    let mut result: u64 = 0;
    for i in 0..9 {
      for j in 0..9 {
        let piece_value = game.local_boards[i].board[j] as usize;
        let piece_position = i * 9 + j;
        result = result | self.piece_table[piece_value][piece_position];
      }
    }
    let current_board_table_index = match game.current_board {
      // indexes 0-8 for Some(x), 9 for None
      Some(x) => usize::from(x),
      None => 9,
    };
    return result | self.current_board_table[current_board_table_index];
  }

  // get the hashed state of the game after making a move
  fn apply_move(&self, initial_hash: u64, board: u8, cell: u8, piece: Piece) -> u64 {
    let piece_value = piece as usize;
    let piece_position = usize::from(board * 9 + cell);
    // XOR out the blank square; XOR in the piece
    return initial_hash
      ^ self.piece_table[Piece::BLANK as usize][piece_position]
      ^ self.piece_table[piece_value][piece_position];
    // // XOR out the old current board; XOR in the new one
    // | self.current_board_table[]
    // | self.current_board_table[]
  }
}

fn legal_moves_for_board(game: &game::Game, board_index: u8) -> Vec<(u8, u8)> {
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
    Some(x) => legal.append(&mut legal_moves_for_board(game, x)),
    None => {
      for i in 0..9 {
        if game.local_boards[usize::from(i)].claimer != None {
          continue;
        }
        legal.append(&mut legal_moves_for_board(game, i))
      }
    }
  }
  return legal;
}

#[derive(Debug, PartialEq)]
struct Node {
  games_played: u32,
  games_won: u32,
  player: Piece,
}

impl Node {
  fn new(player: Piece) -> Node {
    Node {
      games_played: 0,
      games_won: 0,
      player,
    }
  }
}

pub fn evaluate(game: &mut game::Game) -> Option<(u8, u8)> {
  let hasher = ZorbistHasher::new();
  let initial_hash = hasher.hash(&game);

  let mut game_states = HashMap::new();
  // the root represents the opponent, and its immediate children represent the next player to move
  // this way the potential next move's data will represent which move for the current player would lead to more wins
  game_states.insert(
    initial_hash,
    Node::new(match game.turn {
      Piece::X => Piece::O,
      Piece::O => Piece::X,
      _ => panic!(),
    }),
  );

  let mut moves_made = 0;

  'outer: for i in 0..10000 {
    let mut current_game_line = vec![initial_hash];

    // Selection: traverse down the tree until you need to create a new node
    loop {
      let legal = legal_moves(&game);

      if legal.len() == 0 {
        // this means you reached the end of the game line. the game is over and no further expansion is possible.
        // at this point, if we select from ucb1 we will keep selecting the same line if we do try again, so might as well stop evaluating
        // if selections were random it's still possible to reveal new lines, but the longer we continue the less likely we will find newer ones
        // so break if ucb1, continue to try again for random

        // remember to rewind the game back to the initial state
        while moves_made > 0 {
          game.undo_move();
          moves_made -= 1;
        }
        // break 'outer;
        continue 'outer;
      }

      let unvisited_nodes = legal
        .iter()
        .filter(|(a, b)| {
          let new_hash = hasher.apply_move(*current_game_line.last().unwrap(), *a, *b, game.turn);
          let was_not_visited = game_states.get(&new_hash) == None;
          return was_not_visited;
        })
        .collect::<Vec<&(u8, u8)>>();

      if unvisited_nodes.len() == 0 {
        // if all the children nodes were visited at least once, then do ubc1 selection to choose which branch to explore
        // but for now i do it randomly
        let mut rng = thread_rng();
        let selected_index = rng.gen_range(0, legal.len());
        let (a, b) = legal[selected_index];
        let next_hash = hasher.apply_move(*current_game_line.last().unwrap(), a, b, game.turn);
        current_game_line.push(next_hash);
        game.make_move(a, b);
        moves_made += 1;
      } else {
        // Expansion: Expand one of the nodes that wasn't visited yet.
        let mut rng = thread_rng();
        let selected_index = rng.gen_range(0, unvisited_nodes.len());
        let (a, b) = *unvisited_nodes[selected_index];
        let new_hash = hasher.apply_move(*current_game_line.last().unwrap(), a, b, game.turn);
        current_game_line.push(new_hash);
        game_states.insert(new_hash, Node::new(game.turn));
        game.make_move(a, b);
        moves_made += 1;
        break;
      }
    }

    // Simulation: make random moves until the game is over
    loop {
      if game.winner != None {
        // Backpropagation: update the results of the simulated game line
        while let Some(line) = current_game_line.pop() {
          let node = game_states.get_mut(&line).unwrap();
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
      let mut rng = thread_rng();
      let selected_index = rng.gen_range(0, legal.len());
      let (a, b) = legal[selected_index];

      let new_hash = hasher.apply_move(*current_game_line.last().unwrap(), a, b, game.turn);
      game.make_move(a, b);
      game_states.entry(new_hash).or_insert(Node::new(game.turn));
      current_game_line.push(new_hash);
      moves_made += 1;
    }
  }

  // time to pick the best move and return it
  let legal = legal_moves(&game);
  let mut best_score: Option<f32> = None;
  let mut best_move: Option<(u8, u8)> = None;

  for i in 0..legal.len() {
    let (a, b) = legal[i];
    let new_hash = hasher.apply_move(initial_hash, a, b, game.turn);
    println!("\r{:?}", game_states.get(&new_hash));
    match game_states.get(&new_hash) {
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
  }

  return best_move;
}
