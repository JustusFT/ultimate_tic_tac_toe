// use rustyline::Editor;
use std::cmp;
// use std::io::{stdout, Write};
// use termion::clear;
// use termion::cursor;
// use termion::raw::IntoRawMode;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;

const WIN_STATES: [[usize; 3]; 8] = [
   [0, 1, 2],
   [3, 4, 5],
   [6, 7, 8],
   [0, 3, 6],
   [1, 4, 7],
   [2, 5, 8],
   [0, 4, 8],
   [2, 4, 6],
];

const BOARD_DISPLAY: &'static str = "   \
   │   │    ┃    │   │    ┃    │   │   \r
───┼───┼─── ┃ ───┼───┼─── ┃ ───┼───┼───\r
   │   │    ┃    │   │    ┃    │   │   \r
───┼───┼─── ┃ ───┼───┼─── ┃ ───┼───┼───\r
   │   │    ┃    │   │    ┃    │   │   \r
━━━━━━━━━━━━╋━━━━━━━━━━━━━╋━━━━━━━━━━━━\r
   │   │    ┃    │   │    ┃    │   │   \r
───┼───┼─── ┃ ───┼───┼─── ┃ ───┼───┼───\r
   │   │    ┃    │   │    ┃    │   │   \r
───┼───┼─── ┃ ───┼───┼─── ┃ ───┼───┼───\r
   │   │    ┃    │   │    ┃    │   │   \r
━━━━━━━━━━━━╋━━━━━━━━━━━━━╋━━━━━━━━━━━━\r
   │   │    ┃    │   │    ┃    │   │   \r
───┼───┼─── ┃ ───┼───┼─── ┃ ───┼───┼───\r
   │   │    ┃    │   │    ┃    │   │   \r
───┼───┼─── ┃ ───┼───┼─── ┃ ───┼───┼───\r
   │   │    ┃    │   │    ┃    │   │   \r
";

// these mark the coordinates where the top-left cell of a local board is located from the BOARD_DISPLAY
const X_CORNERS: [u16; 3] = [2, 16, 30];
const Y_CORNERS: [u16; 3] = [1, 7, 13];
// these mark the distance to the other cells of the local board, starting from the top left cell of the local board
const X_OFFSETS: [u16; 3] = [0, 4, 8];
const Y_OFFSETS: [u16; 3] = [0, 2, 4];

#[derive(Clone, Copy, PartialEq, Serialize)]
pub enum Piece {
   BLANK,
   X,
   O,
}

enum Potential {
   X,
   O,
   BOTH,
   NEITHER,
}

#[derive(PartialEq)]
enum GameWinState {
   X,
   O,
   DRAW,
   ONGOING,
}

#[derive(Clone, Copy, Serialize)]
struct LocalBoard {
   board: [Piece; 9],
   claimer: Option<Piece>,
}

impl LocalBoard {
   fn new() -> LocalBoard {
      LocalBoard {
         board: [Piece::BLANK; 9],
         claimer: None,
      }
   }

   // we use this to be able to directly read the board data on the javascript side
   fn get_board_pointer(&self) -> *const Piece {
      self.board.as_ptr()
   }

   // find if there was a win, or a draw
   fn get_results(&self) -> Option<Piece> {
      let win = WIN_STATES.iter().find(|win_state| {
         let [a, b, c] = win_state;
         return self.board[*a] != Piece::BLANK
            && self.board[*a] == self.board[*b]
            && self.board[*b] == self.board[*c];
      });

      match win {
         Some(x) => Some(self.board[x[0]]),
         None => {
            let draw = self.board.iter().all(|x| *x != Piece::BLANK);
            if draw {
               return Some(Piece::BLANK);
            }
            return None;
         }
      }
   }

   fn place_piece(&mut self, cell: usize, piece: Piece) {
      // validate the cell is vacant
      if self.board[cell] != Piece::BLANK {
         panic!()
      }

      self.board[cell] = piece;

      self.claimer = self.get_results()
   }
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct Game {
   local_boards: [LocalBoard; 9],
   current_board: Option<usize>,
   turn: Piece,
   winner: Option<Piece>,
}

// converts board number into 2D coords (x, y)
// 0 is (0, 0), 8 is (2, 2)
fn board_coordinates(cell: usize) -> (usize, usize) {
   assert!(cell < 9);
   (cell % 3, cell / 3)
}

fn piece_to_char(piece: Piece) -> char {
   match piece {
      Piece::X => 'X',
      Piece::O => 'O',
      Piece::BLANK => ' ',
   }
}

#[wasm_bindgen]
impl Game {
   pub fn new() -> Game {
      Game {
         local_boards: [LocalBoard::new(); 9],
         current_board: None,
         turn: Piece::X,
         winner: None,
      }
   }

   pub fn get_state(&self) -> JsValue {
      JsValue::from_serde(&self).unwrap()
   }

   pub fn get_board_pointer(&self, index: usize) -> *const Piece {
      self.local_boards[index].get_board_pointer()
   }

   fn switch_turns(&mut self) {
      self.turn = match self.turn {
         Piece::X => Piece::O,
         Piece::O => Piece::X,
         _ => panic!(),
      }
   }

   pub fn make_move(&mut self, local_board: usize, cell: usize) {
      // validate the move is legal before proceeding
      assert!(local_board < 9);
      assert!(cell < 9);
      assert!(self.winner == None);

      match self.current_board {
         Some(n) => assert!(local_board == n),
         None => {}
      }

      match self.local_boards[local_board].claimer {
         Some(_) => panic!(),
         None => {}
      }

      // update the target cell
      self.local_boards[local_board].place_piece(cell, self.turn);

      // update the current_board
      // - if the next local_board is claimed, set it to None, which means the player can play anywhere
      self.current_board = match self.local_boards[cell].claimer {
         Some(_) => None,
         None => Some(cell),
      };

      // update the winner status
      self.update_win_state();

      self.switch_turns();
   }

   pub fn cpu_move(&mut self, depth: i16) {
      let (best_move_a, best_move, _) = self.negamax(depth, -3000, 3000, -1);
      self.make_move(best_move_a.unwrap(), best_move.unwrap());
   }

   // remove a piece and switch turns
   // it cant reverse claimed state, winner state, or current local board, do that manually
   fn remove_move(&mut self, local_board: usize, cell: usize) {
      // validate the move is legal before proceeding
      assert!(local_board < 9);
      assert!(cell < 9);

      // update the target cell
      self.local_boards[local_board].board[cell] = Piece::BLANK;

      self.switch_turns();
   }

   // // change a piece of the board in the terminal display
   // // pass in which local_board (from 1 to 9) has the cell that needs to be changed
   // // then do the same for the cell number
   // fn draw_piece(
   //    &self,
   //    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
   //    piece: Piece,
   //    local_board: usize,
   //    cell: usize,
   // ) {
   //    // the boards and cells indices only go up to 8
   //    assert!(local_board < 9);
   //    assert!(cell < 9);

   //    // to target the coordinates of the target cell we do it in 2 steps:
   //    // 1. go to the top-left of the target local board
   //    // 2. offset the cursor to go on the right cell
   //    let (corner_x, corner_y) = board_coordinates(local_board);
   //    let (offset_x, offset_y) = board_coordinates(cell);

   //    // then write the piece char at the target
   //    write!(
   //       stdout,
   //       "{move}{piece}",
   //       move = cursor::Goto(
   //          X_CORNERS[corner_x] + X_OFFSETS[offset_x],
   //          Y_CORNERS[corner_y] + Y_OFFSETS[offset_y]
   //       ),
   //       piece = piece_to_char(piece)
   //    )
   //    .unwrap();
   // }

   // // re-draw the whole board
   // fn draw_board(&self, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
   //    write!(
   //       stdout,
   //       "{clear}{move}{board}",
   //       clear = clear::All,
   //       move = cursor::Goto(1, 1),
   //       board = BOARD_DISPLAY
   //    )
   //    .unwrap();

   //    for i in 0..=8 {
   //       for j in 0..=8 {
   //          self.draw_piece(stdout, self.local_boards[i].board[j], i, j)
   //       }
   //    }

   //    // move the cursor to the bottom
   //    write!(stdout, "\r\n").unwrap();

   //    stdout.flush().unwrap();
   // }

   // // request input for next move
   // fn request_user_move(&mut self) {
   //    let mut rl = Editor::<()>::new();
   //    let mut current_board_index: usize;

   //    match self.current_board {
   //       Some(x) => {
   //          println!("\rCurrent board: {}", x);
   //          current_board_index = x;
   //       }
   //       None => loop {
   //          print!("\rInput board #");
   //          let readline = rl.readline("> ");
   //          match readline {
   //             Ok(line) => {
   //                current_board_index = line.parse::<usize>().unwrap();
   //                if self.local_boards[current_board_index].claimer == None {
   //                   break;
   //                }
   //             }
   //             _ => {}
   //          }
   //       },
   //    };

   //    loop {
   //       print!("\rInput cell #");
   //       let readline = rl.readline("> ");
   //       match readline {
   //          Ok(line) => {
   //             let n = line.parse::<usize>().unwrap();
   //             if self.local_boards[current_board_index].board[n] == Piece::BLANK {
   //                self.make_move(current_board_index, n);
   //                break;
   //             }
   //          }
   //          _ => {}
   //       }
   //    }
   // }

   fn update_win_state(&mut self) {
      // check for 3 in a rows
      for i in 0..WIN_STATES.len() {
         let [a, b, c] = WIN_STATES[i];
         if self.local_boards[a].claimer != Some(Piece::BLANK)
            && self.local_boards[a].claimer == self.local_boards[b].claimer
            && self.local_boards[b].claimer == self.local_boards[c].claimer
         {
            match self.local_boards[a].claimer {
               Some(Piece::X) => self.winner = Some(Piece::X),
               Some(Piece::O) => self.winner = Some(Piece::O),
               _ => {}
            }
         }
      }
      // check for draws
      if self.local_boards.iter().all(|x| x.claimer != None) {
         self.winner = Some(Piece::BLANK)
      }
   }

   fn get_win_state(&self) -> GameWinState {
      // check for 3 in a rows
      for i in 0..WIN_STATES.len() {
         let [a, b, c] = WIN_STATES[i];
         if self.local_boards[a].claimer != Some(Piece::BLANK)
            && self.local_boards[a].claimer == self.local_boards[b].claimer
            && self.local_boards[b].claimer == self.local_boards[c].claimer
         {
            match self.local_boards[a].claimer {
               Some(Piece::X) => {
                  return GameWinState::X;
               }
               Some(Piece::O) => {
                  return GameWinState::O;
               }
               _ => {}
            }
         }
      }
      // check for draws
      if self.local_boards.iter().all(|x| x.claimer != None) {
         return GameWinState::DRAW;
      }
      // otherwise, it's an ongoing game
      return GameWinState::ONGOING;
   }

   fn local_row_potential(&self, local_board_index: usize, win_state: &[usize; 3]) -> Potential {
      let has_x = win_state
         .iter()
         .any(|x| self.local_boards[local_board_index].board[*x] == Piece::X);

      let has_o = win_state
         .iter()
         .any(|x| self.local_boards[local_board_index].board[*x] == Piece::O);

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

   fn evaluate_local_row(&self, local_board_index: usize, win_state: &[usize; 3]) -> i16 {
      // should be a bit different than global row, rows that sending anywhere upon solve are less valuable
      let local_board = self.local_boards[local_board_index];
      let potential = self.local_row_potential(local_board_index, win_state);
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
               if local_board.board[*x] == Piece::X {
                  score += 1;
               }
            });
            return score;
         }
         Potential::O => {
            let mut score = 0;
            win_state.iter().for_each(|x| {
               if local_board.board[*x] == Piece::O {
                  score -= 1;
               }
            });
            return score;
         }
      }
   }

   fn evaluate_local_board(&self, local_board_index: usize) -> i16 {
      let local_board = self.local_boards[local_board_index];
      match local_board.claimer {
         Some(Piece::X) => {
            return 20;
         }
         Some(Piece::O) => {
            return -20;
         }
         _ => {
            let mut score = 0;
            WIN_STATES.iter().for_each(|win_state| {
               score += self.evaluate_local_row(local_board_index, win_state);
            });
            return score;
         }
      }
   }

   // returns whether X, O, both, or neither can claim this row
   // maybe add a new enum for this function, don't use Option<Piece>
   fn row_potential(&self, win_state: &[usize; 3]) -> Potential {
      let has_x = win_state
         .iter()
         .any(|x| self.local_boards[*x].claimer == Some(Piece::X));

      let has_o = win_state
         .iter()
         .any(|x| self.local_boards[*x].claimer == Some(Piece::O));

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
   fn evaluate_row(&self, win_state: &[usize; 3]) -> i16 {
      let potential = self.row_potential(win_state);
      match potential {
         Potential::NEITHER => {
            // No score since neither can win this row. It doesn't matter
            return 0;
         }
         _ => {
            let mut score = 0;
            win_state.iter().for_each(|x| {
               score += self.evaluate_local_board(*x);
            });
            return score;
         }
      }
   }

   // gets the heuristic value of the board
   fn evaluate(&self) -> i16 {
      // match self.get_win_state() {
      //    GameWinState::X => {
      //       return 1000;
      //    }
      //    GameWinState::O => {
      //       return -1000;
      //    }
      //    GameWinState::DRAW => {
      //       return 0;
      //    }
      //    _ => {}
      // }

      match self.winner {
         Some(Piece::X) => {
            return 1000;
         }
         Some(Piece::O) => {
            return -1000;
         }
         Some(Piece::BLANK) => {
            return 0;
         }
         _ => {}
      }

      let mut score: i16 = 0;

      WIN_STATES.iter().for_each(|win_triple| {
         score += self.evaluate_row(win_triple);
      });

      return score;
   }

   fn is_game_won(&self) -> bool {
      self.winner != None
   }

   fn negamax(
      &mut self,
      depth: i16,
      mut alpha: i16,
      beta: i16,
      color: i16,
   ) -> (Option<usize>, Option<usize>, i16) {
      // if depth == 0 || self.get_win_state() != GameWinState::ONGOING {
      if depth == 0 || self.is_game_won() {
         let score = (color * self.evaluate()) - depth;
         return (None, None, score);
      }
      let mut best_move_a = None;
      let mut best_move = None;
      let mut best_score = -2000;

      let original_board = self.current_board;
      let original_winner = self.winner;

      // loop through legal moves
      match self.current_board {
         Some(current_board) => {
            best_move_a = Some(current_board);
            let original_claimer = self.local_boards[current_board].claimer;
            for i in 0..9 {
               let x = self.local_boards[current_board].board[i];
               if x == Piece::BLANK {
                  // legal move!
                  self.make_move(current_board, i);
                  let (_, _, next_score) = self.negamax(depth - 1, -beta, -alpha, -color);
                  if -next_score > best_score {
                     best_score = -next_score;
                     best_move = Some(i);
                  }
                  self.remove_move(current_board, i);
                  self.local_boards[current_board].claimer = original_claimer;
                  self.current_board = original_board;
                  self.winner = original_winner;
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
               if self.local_boards[current_board].claimer != None {
                  continue;
               }
               // TODO: DRY it up with above
               let original_claimer = self.local_boards[current_board].claimer;
               for i in 0..9 {
                  let x = self.local_boards[current_board].board[i];
                  if x == Piece::BLANK {
                     // legal move!
                     self.make_move(current_board, i);
                     let (_, _, next_score) = self.negamax(depth - 1, -beta, -alpha, -color);
                     if -next_score > best_score {
                        best_move_a = Some(current_board);
                        best_score = -next_score;
                        best_move = Some(i);
                     }
                     self.remove_move(current_board, i);
                     self.local_boards[current_board].claimer = original_claimer;
                     self.current_board = original_board;
                     self.winner = original_winner;
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
}

// fn main() {
//    // Enter raw mode.
//    let mut stdout = stdout().into_raw_mode().unwrap();

//    let mut game = Game::new();

//    loop {
//       game.draw_board(&mut stdout);

//       println!("\r{}", game.evaluate());

//       game.request_user_move();
//       let (best_move_a, best_move, _) = game.negamax(8, -3000, 3000, -1);

//       game.make_move(best_move_a.unwrap(), best_move.unwrap());
//    }
// }
