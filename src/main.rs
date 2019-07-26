// use rustyline::error::ReadlineError;
// use rustyline::Editor;
// use std::cmp;
// use std::io::{stdin, stdout, Write};
// use termion::clear;
// use termion::cursor;
// use termion::event::Key;
// use termion::input::TermRead;
// use termion::raw::IntoRawMode;

use rustyline::Editor;
use std::io::{stdout, Write};
use termion::clear;
use termion::cursor;
use termion::raw::IntoRawMode;

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

#[derive(Clone, Copy, PartialEq)]
enum Piece {
   X,
   O,
   BLANK,
}

enum Potential {
   X,
   O,
   BOTH,
   NEITHER,
}

#[derive(Clone, Copy)]
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

struct Game {
   local_boards: [LocalBoard; 9],
   current_board: Option<usize>,
   turn: Piece,
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

impl Game {
   fn new() -> Game {
      Game {
         local_boards: [LocalBoard::new(); 9],
         current_board: None,
         turn: Piece::X,
      }
   }

   fn switch_turns(&mut self) {
      self.turn = match self.turn {
         Piece::X => Piece::O,
         Piece::O => Piece::X,
         _ => panic!(),
      }
   }

   fn make_move(&mut self, local_board: usize, cell: usize) {
      // validate the move is legal before proceeding
      assert!(local_board < 9);
      assert!(cell < 9);

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

      self.switch_turns();
   }

   // change a piece of the board in the terminal display
   // pass in which local_board (from 1 to 9) has the cell that needs to be changed
   // then do the same for the cell number
   fn draw_piece(
      &self,
      stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
      piece: Piece,
      local_board: usize,
      cell: usize,
   ) {
      // the boards and cells indices only go up to 8
      assert!(local_board < 9);
      assert!(cell < 9);

      // to target the coordinates of the target cell we do it in 2 steps:
      // 1. go to the top-left of the target local board
      // 2. offset the cursor to go on the right cell
      let (corner_x, corner_y) = board_coordinates(local_board);
      let (offset_x, offset_y) = board_coordinates(cell);

      // then write the piece char at the target
      write!(
         stdout,
         "{move}{piece}",
         move = cursor::Goto(
            X_CORNERS[corner_x] + X_OFFSETS[offset_x],
            Y_CORNERS[corner_y] + Y_OFFSETS[offset_y]
         ),
         piece = piece_to_char(piece)
      )
      .unwrap();
   }

   // re-draw the whole board
   fn draw_board(&self, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
      write!(
         stdout,
         "{clear}{move}{board}",
         clear = clear::All,
         move = cursor::Goto(1, 1),
         board = BOARD_DISPLAY
      )
      .unwrap();

      for i in 0..=8 {
         for j in 0..=8 {
            self.draw_piece(stdout, self.local_boards[i].board[j], i, j)
         }
      }

      // move the cursor to the bottom
      write!(stdout, "\r\n").unwrap();

      stdout.flush().unwrap();
   }

   // request input for next move
   fn request_user_move(&mut self) {
      let mut rl = Editor::<()>::new();
      let current_board_index: usize;

      match self.current_board {
         Some(x) => {
            println!("\rCurrent board: {}", x);
            current_board_index = x;
         }
         None => {
            print!("\rInput board #");
            let readline = rl.readline("> ");
            match readline {
               Ok(line) => {
                  current_board_index = line.parse::<usize>().unwrap();
               }
               Err(err) => {
                  println!("Error: {:?}", err);
                  panic!();
               }
            }
         }
      };

      print!("\rInput cell #");
      let readline = rl.readline("> ");
      match readline {
         Ok(line) => {
            let n = line.parse::<usize>().unwrap();
            self.make_move(current_board_index, n);
         }
         Err(err) => {
            println!("Error: {:?}", err);
            panic!();
         }
      }
   }

   fn winner(&self) -> Option<Piece> {
      return None;
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
            return 10;
         }
         Some(Piece::O) => {
            return -10;
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

   // gets the heuristic value of the row
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
      match self.winner() {
         Some(Piece::X) => {
            return 1000;
         }
         Some(Piece::O) => {
            return -1000;
         }
         _ => {}
      }

      let mut score: i16 = 0;

      WIN_STATES.iter().for_each(|win_triple| {
         score += self.evaluate_row(win_triple);
      });

      return score;
   }
}

fn main() {
   // Enter raw mode.
   let mut stdout = stdout().into_raw_mode().unwrap();

   let mut game = Game::new();

   loop {
      game.draw_board(&mut stdout);

      println!("\r{}", game.evaluate());

      game.request_user_move();
   }
}
