use base_game;
use rustyline::Editor;
use std::convert::TryFrom;
use std::io::{stdout, Write};
use std::time::Instant;
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

type BigPieceStringArray = [&'static str; 5];

const BIG_X: BigPieceStringArray = [
    " █▄│   │▄█ ",
    "──▀█▄─▄█▀──",
    "   │███│   ",
    "──▄█▀─▀█▄──",
    " █▀│   │▀█ ",
];

const BIG_O: BigPieceStringArray = [
    "  ▄█▀▀▀█▄  ",
    "─█▀┼───┼▀█─",
    " █ │   │ █ ",
    "─█▄┼───┼▄█─",
    "  ▀█▄▄▄█▀  ",
];

// these mark the coordinates where the top-left cell of a local board is located from the BOARD_DISPLAY
const X_CORNERS: [u16; 3] = [2, 16, 30];
const Y_CORNERS: [u16; 3] = [1, 7, 13];
// these mark the distance to the other cells of the local board, starting from the top left cell of the local board
const X_OFFSETS: [u16; 3] = [0, 4, 8];
const Y_OFFSETS: [u16; 3] = [0, 2, 4];

// converts board number into 2D coords (x, y)
// 0 is (0, 0), 8 is (2, 2)
fn board_coordinates(cell: usize) -> (usize, usize) {
    assert!(cell < 9);
    (cell % 3, cell / 3)
}

fn piece_to_char(piece: base_game::Piece) -> char {
    match piece {
        base_game::Piece::X => 'X',
        base_game::Piece::O => 'O',
        base_game::Piece::BLANK => ' ',
    }
}

// change a piece of the board in the terminal display
// pass in which local_board (from 1 to 9) has the cell that needs to be changed
// then do the same for the cell number
fn draw_piece(
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    piece: base_game::Piece,
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
        u16::try_from(X_CORNERS[corner_x] + X_OFFSETS[offset_x]).ok().unwrap(),
        u16::try_from(Y_CORNERS[corner_y] + Y_OFFSETS[offset_y]).ok().unwrap()
      ),
      piece = piece_to_char(piece)
    )
    .unwrap();
}

// for claimed boards, draw a big piece on top the board display
fn draw_big_piece(
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    local_board: usize,
    overlay: BigPieceStringArray,
) {
    let (corner_x, corner_y) = board_coordinates(local_board);

    for i in 0..5 {
        write!(
            stdout,
            "{move}{line_text}",
            move = cursor::Goto(
                X_CORNERS[corner_x] - 1,
                Y_CORNERS[corner_y] + i
            ),
            line_text = overlay[usize::from(i)]
        )
        .unwrap();
    }
}

// re-draw the whole board
fn draw_board(
    game: &base_game::game::Game,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
) {
    write!(
      stdout,
      "{clear}{move}{board}",
      clear = clear::All,
      move = cursor::Goto(1, 1),
      board = BOARD_DISPLAY
    )
    .unwrap();

    for i in 0..=8 {
        match game.local_boards[i].claimer {
            // for claimed boards, draw a big piece over the board
            // for non-claimed boards, draw what pieces are currently on it
            Some(base_game::Piece::X) => draw_big_piece(stdout, i, BIG_X),
            Some(base_game::Piece::O) => draw_big_piece(stdout, i, BIG_O),
            _ => {
                for j in 0..=8 {
                    draw_piece(stdout, game.local_boards[i].board[j], i, j)
                }
            }
        }
    }

    // move the cursor to the bottom
    write!(stdout, "\r\n").unwrap();

    stdout.flush().unwrap();
}

// request input for next move
fn request_user_move(game: &mut base_game::game::Game) {
    let mut rl = Editor::<()>::new();
    let mut current_board_index: u8;

    match game.current_board {
        Some(x) => {
            println!("\rCurrent board: {}", x);
            current_board_index = x;
        }
        None => loop {
            print!("\rInput board #");
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    current_board_index = line.parse::<u8>().unwrap();
                    if game.local_boards[usize::from(current_board_index)].claimer == None {
                        break;
                    }
                }
                _ => {}
            }
        },
    };

    loop {
        print!("\rInput cell #");
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let n = line.parse::<u8>().unwrap();
                if game.local_boards[usize::from(current_board_index)].board[usize::from(n)]
                    == base_game::Piece::BLANK
                {
                    game.make_move(current_board_index, n);
                    break;
                }
            }
            _ => {}
        }
    }
}

fn main() {
    // Enter raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut game = base_game::game::Game::new();
    let mut search_tree = base_game::monte_carlo::MctsTree::new();
    // let mut game = base_game::fen::new_from_fen("x.x..xx.o/.xo.o...x/.xxo.oox./..oxx.o../...x..o.x/.o.....x./o.x...xxx/o.o.o..../oo.o..o.x ......x.o o 6").unwrap();

    loop {
        draw_board(&game, &mut stdout);
        println!("\rYour turn.");
        request_user_move(&mut game);
        draw_board(&game, &mut stdout);
        println!("\rCPU is thinking...");
        let begin = Instant::now();
        let cpu_move = search_tree.evaluate_while(&mut game, |games_ran| {
            return begin.elapsed().as_secs() < 10 && games_ran < 10000;
        });
        match cpu_move {
            Some((a, b)) => {
                println!("{}, {} is the move", a, b);
                game.make_move(a, b);
            }
            None => {
                println!("No move was evaluated!");
                break;
            }
        }
        // let (best_move_a, best_move, _) = base_game::ai::negamax(&mut game, 5, -3000, 3000, -1);
        // game.make_move(best_move_a.unwrap(), best_move.unwrap());
    }
}
