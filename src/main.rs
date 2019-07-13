use std::io::{stdout, Write};
use termion::clear;
use termion::cursor;
use termion::raw::IntoRawMode;

const BOARD_DISPLAY: &'static str = "   │   │    ┃    │   │    ┃    │   │   \r
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
}
