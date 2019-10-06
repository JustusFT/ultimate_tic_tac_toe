// TODO reimplement terminal app

// use rustyline::Editor;
// use std::io::{stdout, Write};
// use termion::clear;
// use termion::cursor;
// use termion::raw::IntoRawMode;

// const BOARD_DISPLAY: &'static str = "   \
//    │   │    ┃    │   │    ┃    │   │   \r
// ───┼───┼─── ┃ ───┼───┼─── ┃ ───┼───┼───\r
//    │   │    ┃    │   │    ┃    │   │   \r
// ───┼───┼─── ┃ ───┼───┼─── ┃ ───┼───┼───\r
//    │   │    ┃    │   │    ┃    │   │   \r
// ━━━━━━━━━━━━╋━━━━━━━━━━━━━╋━━━━━━━━━━━━\r
//    │   │    ┃    │   │    ┃    │   │   \r
// ───┼───┼─── ┃ ───┼───┼─── ┃ ───┼───┼───\r
//    │   │    ┃    │   │    ┃    │   │   \r
// ───┼───┼─── ┃ ───┼───┼─── ┃ ───┼───┼───\r
//    │   │    ┃    │   │    ┃    │   │   \r
// ━━━━━━━━━━━━╋━━━━━━━━━━━━━╋━━━━━━━━━━━━\r
//    │   │    ┃    │   │    ┃    │   │   \r
// ───┼───┼─── ┃ ───┼───┼─── ┃ ───┼───┼───\r
//    │   │    ┃    │   │    ┃    │   │   \r
// ───┼───┼─── ┃ ───┼───┼─── ┃ ───┼───┼───\r
//    │   │    ┃    │   │    ┃    │   │   \r
// ";

// // these mark the coordinates where the top-left cell of a local board is located from the BOARD_DISPLAY
// const X_CORNERS: [u16; 3] = [2, 16, 30];
// const Y_CORNERS: [u16; 3] = [1, 7, 13];
// // these mark the distance to the other cells of the local board, starting from the top left cell of the local board
// const X_OFFSETS: [u16; 3] = [0, 4, 8];
// const Y_OFFSETS: [u16; 3] = [0, 2, 4];

// struct TerminalGame {
//   game: &Game
// }

// impl TerminalGame {
//   fn new() -> TerminalGame {
//     TerminalGame {
//       game: Game::new()
//     }
//   }

//   // converts board number into 2D coords (x, y)
//   // 0 is (0, 0), 8 is (2, 2)
//   fn board_coordinates(cell: usize) -> (usize, usize) {
//     assert!(cell < 9);
//     (cell % 3, cell / 3)
//   }

//   fn piece_to_char(piece: Piece) -> char {
//     match piece {
//       Piece::X => 'X',
//       Piece::O => 'O',
//       Piece::BLANK => ' ',
//     }
//   }

//   // change a piece of the board in the terminal display
//   // pass in which local_board (from 1 to 9) has the cell that needs to be changed
//   // then do the same for the cell number
//   fn draw_piece(
//     &self,
//     stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
//     piece: Piece,
//     local_board: usize,
//     cell: usize,
//   ) {
//     // the boards and cells indices only go up to 8
//     assert!(local_board < 9);
//     assert!(cell < 9);

//     // to target the coordinates of the target cell we do it in 2 steps:
//     // 1. go to the top-left of the target local board
//     // 2. offset the cursor to go on the right cell
//     let (corner_x, corner_y) = TerminalGame::board_coordinates(local_board);
//     let (offset_x, offset_y) = TerminalGame::board_coordinates(cell);

//     // then write the piece char at the target
//     write!(
//       stdout,
//       "{move}{piece}",
//       move = cursor::Goto(
//         X_CORNERS[corner_x] + X_OFFSETS[offset_x],
//         Y_CORNERS[corner_y] + Y_OFFSETS[offset_y]
//       ),
//       piece = TerminalGame::piece_to_char(piece)
//     )
//     .unwrap();
//   }

//   // re-draw the whole board
//   fn draw_board(&self, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
//     write!(
//       stdout,
//       "{clear}{move}{board}",
//       clear = clear::All,
//       move = cursor::Goto(1, 1),
//       board = BOARD_DISPLAY
//     )
//     .unwrap();

//     for i in 0..=8 {
//       for j in 0..=8 {
//         self.draw_piece(stdout, self.game.local_boards[i].board[j], i, j)
//       }
//     }

//     // move the cursor to the bottom
//     write!(stdout, "\r\n").unwrap();

//     stdout.flush().unwrap();
//   }

//   // request input for next move
//   fn request_user_move(&mut self) {
//     let mut rl = Editor::<()>::new();
//     let mut current_board_index: usize;

//     match self.game.current_board {
//       Some(x) => {
//         println!("\rCurrent board: {}", x);
//         current_board_index = x;
//       }
//       None => loop {
//         print!("\rInput board #");
//         let readline = rl.readline("> ");
//         match readline {
//           Ok(line) => {
//             current_board_index = line.parse::<usize>().unwrap();
//             if self.game.local_boards[current_board_index].claimer == None {
//               break;
//             }
//           }
//           _ => {}
//         }
//       },
//     };

//     loop {
//       print!("\rInput cell #");
//       let readline = rl.readline("> ");
//       match readline {
//         Ok(line) => {
//           let n = line.parse::<usize>().unwrap();
//           if self.game.local_boards[current_board_index].board[n] == Piece::BLANK {
//             self.game.make_move(current_board_index, n);
//             break;
//           }
//         }
//         _ => {}
//       }
//     }
//   }

//   fn play(&self) {
//     // Enter raw mode.
//     let mut stdout = stdout().into_raw_mode().unwrap();

//     loop {
//       self.draw_board(&mut stdout);

//       println!("\r{}", self.game.evaluate());

//       self.request_user_move();
//       let (best_move_a, best_move, _) = self.game.negamax(8, -3000, 3000, -1);

//       self.game.make_move(best_move_a.unwrap(), best_move.unwrap());
//     }
//   }
// }
