use crate::local_board::LocalBoard;
use crate::{Piece, WIN_STATES};
use rand::prelude::*;
use std::fmt;
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize)]
pub struct MoveInfo {
    board_index: u8,
    piece_index: u8,
    current_board: Option<u8>,
}

#[wasm_bindgen]
struct ZorbistHasher {
    // there are 81 squares, each square has 3 possible states: X, O, or blank.
    piece_table: [[u64; 81]; 3],
    // there are 10 (1+9) total possible states for the current board to be played on:
    // - one for when all boards can be played
    // - nine when a single board can be played
    current_board_table: [u64; 10], // we can hash the game state using both tables above
}

impl fmt::Debug for ZorbistHasher {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(formatter, "[Hash table data...]");
    }
}

impl ZorbistHasher {
    fn new() -> ZorbistHasher {
        let mut rng = rand::thread_rng();

        let mut piece_table: [[u64; 81]; 3] = [[0; 81]; 3];
        let mut current_board_table: [u64; 10] = [0; 10];

        for i in 0..3 {
            for j in 0..81 {
                let random_int: u64 = rng.gen();
                piece_table[i][j] = random_int;
            }
        }

        for i in 0..10 {
            let random_int: u64 = rng.gen();
            current_board_table[i] = random_int;
        }

        ZorbistHasher {
            piece_table,
            current_board_table,
        }
    }

    fn hash(&self, game: &Game) -> u64 {
        let mut result: u64 = 0;
        for i in 0..9 {
            for j in 0..9 {
                let piece_value = game.local_boards[i].board[j] as usize;
                let piece_position = i * 9 + j;
                result = result ^ self.piece_table[piece_value][piece_position];
            }
        }
        let current_board_table_index = match game.current_board {
            // indexes 0-8 for Some(x), 9 for None
            Some(x) => usize::from(x),
            None => 9,
        };
        return result ^ self.current_board_table[current_board_table_index];
    }
}

pub struct GameBuilder {
    pub local_boards: Option<[LocalBoard; 9]>,
    pub current_board: Option<Option<u8>>,
    pub turn: Option<Piece>,
}

impl GameBuilder {
    pub fn new() -> GameBuilder {
        GameBuilder {
            local_boards: None,
            current_board: None,
            turn: None,
        }
    }
    pub fn with_initial_local_boards(mut self, boards: [LocalBoard; 9]) -> GameBuilder {
        self.local_boards = Some(boards);
        return self;
    }
    pub fn with_initial_current_board(mut self, current_board: Option<u8>) -> GameBuilder {
        self.current_board = Some(current_board);
        return self;
    }

    pub fn with_initial_turn(mut self, turn: Piece) -> GameBuilder {
        self.turn = Some(turn);
        return self;
    }

    pub fn build(&self) -> Game {
        let mut game = Game {
            local_boards: self.local_boards.unwrap_or([LocalBoard::new(); 9]),
            current_board: self.current_board.unwrap_or(None),
            turn: self.turn.unwrap_or(Piece::X),
            winner: None,
            history: Vec::new(),
            hash: 0,
            hasher: ZorbistHasher::new(),
        };
        game.hash = game.hasher.hash(&game);
        return game;
    }
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
    pub hash: u64,
    #[serde(skip)]
    hasher: ZorbistHasher,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        return GameBuilder::new().build();
    }

    fn get_current_board_hash_index(current_board: Option<u8>) -> usize {
        // use indexes 0-8 for the nine boards, index 9 for None
        match current_board {
            Some(x) => x.into(),
            None => 9,
        }
    }

    // this method should run before updating the game state with make_move() or undo_move()
    fn update_hash(
        &mut self,
        board: u8,
        cell: u8,
        old_piece: Piece,
        new_piece: Piece,
        next_current_board: Option<u8>,
    ) {
        let piece_position = usize::from(board * 9 + cell);
        self.hash = self.hash
            // XOR out the old piece; XOR in the piece placed
            ^ self.hasher.piece_table[old_piece as usize][piece_position]
            ^ self.hasher.piece_table[new_piece as usize][piece_position]
            // XOR out the old current board; XOR in the new one
            ^ self.hasher.current_board_table[Game::get_current_board_hash_index(self.current_board)]
            ^ self.hasher.current_board_table[Game::get_current_board_hash_index(next_current_board)]
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
            Some(_) => panic!("Already claimed!"),
            None => {}
        }

        // update the history
        self.history.push(MoveInfo {
            board_index: local_board,
            piece_index: cell,
            current_board: self.current_board,
        });

        // apply the next piece and update claimed board info
        self.local_boards[usize::from(local_board)].place_piece(usize::from(cell), self.turn);
        self.update_win_state();

        // figure out where the next player has to make their move
        // if the next local_board is claimed, set it to None, which means the next player can play anywhere
        let next_current_board = match self.local_boards[usize::from(cell)].claimer {
            Some(_) => None,
            None => Some(cell),
        };

        // apply the move to the hash as well
        self.update_hash(
            local_board,
            cell,
            Piece::BLANK,
            self.turn,
            next_current_board,
        );

        self.current_board = next_current_board;
        self.switch_turns();
    }

    // reverse the game state using the last move in the history
    pub fn undo_move(&mut self) {
        let last_move = self.history.pop();
        self.switch_turns();

        match last_move {
            Some(x) => {
                // update the hash first before mutating the game state
                self.update_hash(
                    x.board_index,
                    x.piece_index,
                    self.turn,
                    Piece::BLANK,
                    x.current_board,
                );
                // undo game state
                self.local_boards[usize::from(x.board_index)].board[usize::from(x.piece_index)] =
                    Piece::BLANK;
                self.current_board = x.current_board;
                // if you're undoing, the result of the game state will
                // - always unclaim the last board played on
                // - always have no winner
                self.local_boards[usize::from(x.board_index)].claimer = None;
                self.winner = None;
            }
            None => {
                // there's nothing to undo
            }
        }
    }

    fn switch_turns(&mut self) {
        self.turn = match self.turn {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
            _ => panic!(),
        }
    }

    // check if the current state has a winner
    // - if there is a winner then self.winner is updated to reflect who won
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
    use super::*;

    #[test]
    fn hash_is_consistent() {
        let mut game = Game::new();
        let first_state_hash_1 = game.hash;
        game.make_move(0, 0);
        let second_state_hash_1 = game.hash;
        game.undo_move();
        let first_state_hash_2 = game.hash;
        game.make_move(0, 0);
        let second_state_hash_2 = game.hash;
        assert_eq!(first_state_hash_1, first_state_hash_2);
        assert_eq!(second_state_hash_1, second_state_hash_2);
    }
}
