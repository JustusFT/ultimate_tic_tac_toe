use crate::game::{Game,GameBuilder};
use crate::local_board::LocalBoard;
use crate::Piece;
use std::char;

fn local_board_from_fen(fen: &str, claimer: char) -> Result<LocalBoard, String> {
    let chars: Vec<char> = fen.chars().collect();
    if chars.len() != 9 {
        return Err("LocalBoard must be exactly 9 characters".to_string());
    }

    let mut pieces: [Piece; 9] = [Piece::BLANK; 9];
    for i in 0..chars.len() {
        pieces[i] = match chars[i] {
            'x' => Piece::X,
            'o' => Piece::O,
            '.' => Piece::BLANK,
            _ => return Err(chars[i].to_string()),
        }
    }

    Ok(LocalBoard {
        board: pieces,
        claimer: match claimer {
            'x' => Some(Piece::X),
            'o' => Some(Piece::O),
            '-' => Some(Piece::BLANK),
            '.' => None,
            _ => return Err("Claimer is an invalid character".to_string()),
        },
    })
}

// FEN format looks like
// 123456789/123456789/123456789/123456789/123456789/123456789/123456789/123456789/123456789 123456789 x 5
pub fn new_from_fen(fen: &str) -> Result<Game, String> {
    let segments: Vec<&str> = fen.split(' ').collect();
    if segments.len() != 4 {
        return Err("Invalid FEN: segment count is not equal to 4".to_string());
    }
    let board_fens: Vec<&str> = segments[0].split('/').collect();
    if board_fens.len() != 9 {
        return Err("Invalid FEN: string does not contain exactly 9 boards".to_string());
    }

    let claimers: Vec<char> = segments[1].chars().collect();
    if claimers.len() != 9 {
        return Err("Invalid FEN: claimer string does not contain exactly 9 pieces".to_string());
    }

    let mut boards: [LocalBoard; 9] = [LocalBoard::new(); 9];

    for i in 0..board_fens.len() {
        boards[i] = local_board_from_fen(board_fens[i], claimers[i])?;
    }

    let turn = match segments[2] {
        "x" => Piece::X,
        "o" => Piece::O,
        _ => return Err("Turn is an invalid character".to_string()),
    };

    let current_board = match segments[3] {
        "-" => None,
        "0" => Some(0),
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        _ => return Err("Current board is an invalid character".to_string()),
    };

    return Ok(GameBuilder::new()
        .with_initial_local_boards(boards)
        .with_initial_current_board(current_board)
        .with_initial_turn(turn)
        .build());
}

pub fn get_fen(game: &Game) -> String {
    let mut fen: String = "".to_string();
    // 1: generate the board string
    // for each board:
    //   for each board piece:
    //     push piece char to the fen string
    //   add the foward slash
    //
    // pop to remove the extra forward slash
    // push a space to move on to the next step
    game.local_boards.iter().for_each(|x| {
        x.board.iter().for_each(|y| {
            fen.push(match y {
                Piece::X => 'x',
                Piece::O => 'o',
                Piece::BLANK => '.',
            });
        });
        fen.push('/');
    });
    fen.pop();
    fen.push(' ');
    // 2: generate the claimed string
    // for each board:
    //   push claimer char to the fen string
    //
    // push a space to move on to the next step
    game.local_boards.iter().for_each(|x| {
        fen.push(match x.claimer {
            Some(Piece::X) => 'x',
            Some(Piece::O) => 'o',
            Some(Piece::BLANK) => '-',
            None => '.',
        });
    });
    fen.push(' ');
    // 3: push the current player
    fen.push(match game.turn {
        Piece::X => 'x',
        Piece::O => 'o',
        _ => panic!(),
    });
    fen.push(' ');
    // 4: push the current board
    fen.push(match game.current_board {
        Some(x) => char::from_digit(x.into(), 10).unwrap(),
        None => '-',
    });

    return fen;
}
