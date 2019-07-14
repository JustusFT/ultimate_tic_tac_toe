use std::io::{stdin, stdout, Write};
use termion::clear;
use termion::cursor;
use termion::raw::IntoRawMode;

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

// converts board number into 2D coords (x, y)
// 1 is (0, 0), 9 is (2, 2)
fn board_coordinates(cell: usize) -> (usize, usize) {
   assert!(0 < cell && cell < 10);
   ((cell - 1) % 3, (cell - 1) / 3)
}

// change a piece of the board in the terminal display
// pass in which local_board (from 1 to 9) has the cell that needs to be changed
// then do the same for the cell number
fn set_piece(
   stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
   piece: char,
   local_board: usize,
   cell: usize,
) {
   // the boards and cells are labeled 1 to 9, so we make sure that we only pass 1 to 9
   assert!(0 < local_board && local_board < 10);
   assert!(0 < cell && cell < 10);

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
      piece = piece
   )
   .unwrap();
}

fn main() {
   // Enter raw mode.
   let mut stdout = stdout().into_raw_mode().unwrap();

   write!(
      stdout,
      "{clear}{move}{board}",
      clear = clear::All,
      move = cursor::Goto(1, 1),
      board = BOARD_DISPLAY
   )
   .unwrap();

   stdout.flush().unwrap();

   set_piece(&mut stdout, 'O', 9, 1);
}
