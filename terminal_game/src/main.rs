use base_game::game::Game;
use base_game::monte_carlo::MctsTree;
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

struct TerminalGame {
    stdout: termion::raw::RawTerminal<std::io::Stdout>,
    game: Game,
    search_tree: MctsTree,
}

impl TerminalGame {
    fn new() -> TerminalGame {
        TerminalGame {
            stdout: stdout().into_raw_mode().unwrap(),
            game: Game::new(),
            search_tree: MctsTree::new(),
        }
    }

    // clear the screen and redraw the board
    fn draw_board(&mut self) {
        write!(
            self.stdout,
            "{clear}{move}{board}",
            clear = clear::All,
            move = cursor::Goto(1, 1),
            board = BOARD_DISPLAY
        )
        .unwrap();

        for i in 0..=8 {
            match self.game.local_boards[i].claimer {
                // for claimed boards, draw a big piece over the board
                // for non-claimed boards, draw what pieces are currently on it
                Some(base_game::Piece::X) => self.draw_big_piece(i, BIG_X),
                Some(base_game::Piece::O) => self.draw_big_piece(i, BIG_O),
                _ => {
                    for j in 0..=8 {
                        self.draw_piece(i, j)
                    }
                }
            }
        }

        // move the cursor to the bottom
        write!(self.stdout, "\r\n").unwrap();

        self.stdout.flush().unwrap();
    }

    fn redraw_with_error(&mut self, error: String) {
        self.draw_board();
        println!("\r{}", error);
    }

    // change a piece of the board in the terminal display
    // pass in which local_board (from 1 to 9) has the cell that needs to be changed
    // then do the same for the cell number
    fn draw_piece(&mut self, local_board: usize, cell: usize) {
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
            self.stdout,
            "{move}{piece}",
            move = cursor::Goto(
                u16::try_from(X_CORNERS[corner_x] + X_OFFSETS[offset_x]).ok().unwrap(),
                u16::try_from(Y_CORNERS[corner_y] + Y_OFFSETS[offset_y]).ok().unwrap()
            ),
            piece = piece_to_char(self.game.local_boards[local_board].board[cell])
        )
        .unwrap();
    }

    // for claimed boards, draw a big piece on top the board display
    fn draw_big_piece(&mut self, local_board: usize, overlay: BigPieceStringArray) {
        let (corner_x, corner_y) = board_coordinates(local_board);

        for i in 0..5 {
            write!(
                self.stdout,
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

    // request input for next move
    fn request_user_move(&mut self) {
        let mut rl = Editor::<()>::new();
        let current_board_index: u8;

        match self.game.current_board {
            Some(x) => {
                current_board_index = x;
            }
            None => loop {
                print!("\rInput board #");
                let readline = rl.readline("> ");
                match readline {
                    Ok(line) => {
                        // attempt to convert the string to a number
                        match line.parse::<u8>() {
                            Ok(board_number) => {
                                if board_number > 8 {
                                    self.redraw_with_error(format!(
                                        "Please insert a number from 0-8"
                                    ));
                                    continue;
                                }
                                if self.game.local_boards[usize::from(board_number)].claimer == None
                                {
                                    self.draw_board();
                                    current_board_index = board_number;
                                    break;
                                } else {
                                    self.redraw_with_error(format!(
                                        "Board #{} is already claimed!",
                                        board_number
                                    ));
                                }
                            }
                            Err(_) => {
                                self.redraw_with_error(format!("Please insert a number from 0-8"))
                            }
                        }
                    }
                    _ => {}
                }
            },
        };

        loop {
            println!("\rCurrent board: {}", current_board_index);
            print!("\rInput cell #");
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => match line.parse::<u8>() {
                    Ok(cell_number) => {
                        if cell_number > 8 {
                            self.redraw_with_error(format!("Please insert a number from 0-8"));
                            continue;
                        }
                        if self.game.local_boards[usize::from(current_board_index)].board
                            [usize::from(cell_number)]
                            == base_game::Piece::BLANK
                        {
                            self.game.make_move(current_board_index, cell_number);
                            break;
                        } else {
                            self.redraw_with_error(format!(
                                "Cell #{} is already taken!",
                                cell_number
                            ));
                        }
                    }
                    Err(_) => self.redraw_with_error(format!("Please insert a number from 0-8")),
                },
                _ => {}
            }
        }
    }
}

fn main() {
    let mut game = TerminalGame::new();
    loop {
        game.draw_board();
        println!("\rYour turn.");
        game.request_user_move();
        game.draw_board();
        println!("\rCPU is thinking...");
        let begin = Instant::now();
        let cpu_move = game
            .search_tree
            .evaluate_while(&mut game.game, |games_ran| {
                return begin.elapsed().as_secs() < 10 && games_ran < 10000;
            });
        match cpu_move {
            Some((a, b)) => {
                game.game.make_move(a, b);
            }
            None => {
                println!("CPU can't make a move!");
                panic!();
            }
        }
    }
}
