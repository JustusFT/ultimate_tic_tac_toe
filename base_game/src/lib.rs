#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;

pub const WIN_STATES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Serialize)]
pub enum Piece {
    BLANK,
    X,
    O,
}

#[derive(Clone, Copy, Serialize)]
pub struct LocalBoard {
    pub board: [Piece; 9],
    pub claimer: Option<Piece>,
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

#[derive(Serialize)]
pub struct MoveInfo {
    board_index: u8,
    piece_index: u8,
    current_board: Option<u8>,
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct Game {
    #[wasm_bindgen(skip)]
    pub local_boards: [LocalBoard; 9],
    pub current_board: Option<u8>,
    pub turn: Piece,
    pub winner: Option<Piece>,
    history: Vec<MoveInfo>,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            local_boards: [LocalBoard::new(); 9],
            current_board: None,
            turn: Piece::X,
            winner: None,
            history: Vec::new(),
        }
    }

    // reverse the game state using the latest move in the history
    pub fn undo_move(&mut self) {
        let last_move = self.history.pop();

        match last_move {
            Some(x) => {
                self.local_boards[usize::from(x.board_index)].board[usize::from(x.piece_index)] =
                    Piece::BLANK;
                // if you're undoing, the result of the game state will
                // - revert the current board
                // - always unclaim the last board played on
                // - always switch to the other player's turn
                // - always have no winner
                self.current_board = x.current_board;
                self.local_boards[usize::from(x.board_index)].claimer = None;
                self.winner = None;
                self.switch_turns();
            }
            None => {
                // there's nothing to undo
            }
        }
    }

    pub fn make_move(&mut self, local_board: u8, cell: u8) {
        // validate the move is legal before proceeding
        assert!(local_board < 9);
        assert!(cell < 9);
        assert!(self.winner == None);

        match self.current_board {
            Some(n) => assert!(local_board == n),
            None => {}
        }

        match self.local_boards[usize::from(local_board)].claimer {
            Some(_) => panic!(),
            None => {}
        }

        // update the target cell
        self.local_boards[usize::from(local_board)].place_piece(usize::from(cell), self.turn);

        // update the winner status
        self.update_win_state();

        self.switch_turns();

        self.history.push(MoveInfo {
            board_index: local_board,
            piece_index: cell,
            current_board: self.current_board,
        });

        // update the current_board
        // - if the next local_board is claimed, set it to None, which means the player can play anywhere
        // we make sure to update after the move history and not before
        self.current_board = match self.local_boards[usize::from(cell)].claimer {
            Some(_) => None,
            None => Some(cell),
        };
    }

    fn switch_turns(&mut self) {
        self.turn = match self.turn {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
            _ => panic!(),
        }
    }

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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
