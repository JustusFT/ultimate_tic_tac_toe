use crate::{Piece, WIN_STATES};

#[derive(Clone, Copy, Serialize)]
pub struct LocalBoard {
    pub board: [Piece; 9],
    pub claimer: Option<Piece>,
}

impl LocalBoard {
    pub fn new() -> LocalBoard {
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

    pub fn place_piece(&mut self, cell: usize, piece: Piece) {
        // validate the cell is vacant
        if self.board[cell] != Piece::BLANK {
            panic!()
        }

        self.board[cell] = piece;

        self.claimer = self.get_results()
    }
}
