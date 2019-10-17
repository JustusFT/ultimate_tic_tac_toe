use crate::local_board::LocalBoard;
use crate::{Piece, WIN_STATES};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize)]
pub struct MoveInfo {
    board_index: u8,
    piece_index: u8,
    current_board: Option<u8>,
}

#[wasm_bindgen]
#[derive(Debug, Serialize)]
pub struct Game {
    #[wasm_bindgen(skip)]
    pub local_boards: [LocalBoard; 9],
    pub current_board: Option<u8>,
    pub turn: Piece,
    pub winner: Option<Piece>,
    #[wasm_bindgen(skip)]
    pub history: Vec<MoveInfo>,
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
