use std::cmp;
use crate::game;

enum Potential {
  X,
  O,
  BOTH,
  NEITHER,
}

// remove a piece and switch turns
// it cant reverse claimed state, winner state, or current local board, do that manually
fn remove_move(game: &mut game::Game, local_board: usize, cell: usize) {
  // validate the move is legal before proceeding
  assert!(local_board < 9);
  assert!(cell < 9);

  // update the target cell
  game.local_boards[local_board].board[cell] = game::Piece::BLANK;

  game.switch_turns();
}

fn local_row_potential(game: &game::Game, local_board_index: usize, win_state: &[usize; 3]) -> Potential {
  let has_x = win_state
      .iter()
      .any(|x| game.local_boards[local_board_index].board[*x] == game::Piece::X);

  let has_o = win_state
      .iter()
      .any(|x| game.local_boards[local_board_index].board[*x] == game::Piece::O);

  if has_x && has_o {
      return Potential::BOTH;
  } else if has_x {
      return Potential::X;
  } else if has_o {
      return Potential::O;
  } else {
      return Potential::NEITHER;
  }
}

fn evaluate_local_row(game: &game::Game, local_board_index: usize, win_state: &[usize; 3]) -> i16 {
  // should be a bit different than global row, rows that sending anywhere upon solve are less valuable
  let local_board = game.local_boards[local_board_index];
  let potential = local_row_potential(game, local_board_index, win_state);
  match potential {
      Potential::NEITHER => {
        // No score since neither can win this row. It doesn't matter
        return 0;
      }
      Potential::BOTH => {
        // only thing you should care about is if this row has some cells that give the other player the 'send anywhere' ability
        // TODO:
        return 0;
      }
      Potential::X => {
        let mut score = 0;
        win_state.iter().for_each(|x| {
            if local_board.board[*x] == game::Piece::X {
              score += 1;
            }
        });
        return score;
      }
      Potential::O => {
        let mut score = 0;
        win_state.iter().for_each(|x| {
            if local_board.board[*x] == game::Piece::O {
              score -= 1;
            }
        });
        return score;
      }
  }
}

fn evaluate_local_board(game: &game::Game, local_board_index: usize) -> i16 {
  let local_board = game.local_boards[local_board_index];
  match local_board.claimer {
      Some(game::Piece::X) => {
        return 20;
      }
      Some(game::Piece::O) => {
        return -20;
      }
      _ => {
        let mut score = 0;
        game::WIN_STATES.iter().for_each(|win_state| {
            score += evaluate_local_row(game, local_board_index, win_state);
        });
        return score;
      }
  }
}

// returns whether X, O, both, or neither can claim this row
// maybe add a new enum for this function, don't use Option<Piece>
fn row_potential(game: &game::Game, win_state: &[usize; 3]) -> Potential {
  let has_x = win_state
      .iter()
      .any(|x| game.local_boards[*x].claimer == Some(game::Piece::X));

  let has_o = win_state
      .iter()
      .any(|x| game.local_boards[*x].claimer == Some(game::Piece::O));

  if has_x && has_o {
      return Potential::NEITHER;
  } else if has_x {
      return Potential::X;
  } else if has_o {
      return Potential::O;
  } else {
      return Potential::BOTH;
  }
}

// gets the heuristic value of the row10
fn evaluate_row(game: &game::Game, win_state: &[usize; 3]) -> i16 {
  let potential = row_potential(game, win_state);
  match potential {
      Potential::NEITHER => {
        // No score since neither can win this row. It doesn't matter
        return 0;
      }
      _ => {
        let mut score = 0;
        win_state.iter().for_each(|x| {
            score += evaluate_local_board(game, *x);
        });
        return score;
      }
  }
}

// gets the heuristic value of the board
fn evaluate(game: &game::Game) -> i16 {
  match game.winner {
      Some(game::Piece::X) => {
        return 1000;
      }
      Some(game::Piece::O) => {
        return -1000;
      }
      Some(game::Piece::BLANK) => {
        return 0;
      }
      _ => {}
  }

  let mut score: i16 = 0;

  game::WIN_STATES.iter().for_each(|win_triple| {
      score += evaluate_row(game, win_triple);
  });

  return score;
}

pub fn negamax(
  game: &mut game::Game,
  depth: i16,
  mut alpha: i16,
  beta: i16,
  color: i16,
) -> (Option<usize>, Option<usize>, i16) {
  if depth == 0 || game.winner != None {
      let score = (color * evaluate(game)) - depth;
      return (None, None, score);
  }
  let mut best_move_a = None;
  let mut best_move = None;
  let mut best_score = -2000;

  let original_board = game.current_board;
  let original_winner = game.winner;

  // loop through legal moves
  match game.current_board {
      Some(current_board) => {
        best_move_a = Some(current_board);
        let original_claimer = game.local_boards[current_board].claimer;
        for i in 0..9 {
            let x = game.local_boards[current_board].board[i];
            if x == game::Piece::BLANK {
              // legal move!
              game.make_move(current_board, i);
              let (_, _, next_score) = negamax(game, depth - 1, -beta, -alpha, -color);
              if -next_score > best_score {
                  best_score = -next_score;
                  best_move = Some(i);
              }
              remove_move(game, current_board, i);
              game.local_boards[current_board].claimer = original_claimer;
              game.current_board = original_board;
              game.winner = original_winner;
              alpha = cmp::max(alpha, -next_score);
              if alpha >= beta {
                  break;
              }
            }
        }
      }
      // if can go anywhere then loop through each unclaimed board
      None => {
        for current_board in 0..9 {
            if game.local_boards[current_board].claimer != None {
              continue;
            }
            // TODO: DRY it up with above
            let original_claimer = game.local_boards[current_board].claimer;
            for i in 0..9 {
              let x = game.local_boards[current_board].board[i];
              if x == game::Piece::BLANK {
                  // legal move!
                  game.make_move(current_board, i);
                  let (_, _, next_score) = negamax(game, depth - 1, -beta, -alpha, -color);
                  if -next_score > best_score {
                    best_move_a = Some(current_board);
                    best_score = -next_score;
                    best_move = Some(i);
                  }
                  remove_move(game, current_board, i);
                  game.local_boards[current_board].claimer = original_claimer;
                  game.current_board = original_board;
                  game.winner = original_winner;
                  alpha = cmp::max(alpha, -next_score);
                  if alpha >= beta {
                    break;
                  }
              }
            }
        }
      }
  }
  return (best_move_a, best_move, best_score);
}