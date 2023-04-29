#[derive(Copy, Clone, PartialEq, Debug)]
enum Piece {
    O, X, Nil,
}

#[derive(Debug)]
pub enum Turn {
    Player1,
    Player2,
}

impl Turn {
    fn next(&self) -> Self {
        match self {
            Turn::Player1 => Turn::Player2,
            Turn::Player2 => Turn::Player1,
        }
    }
}

#[derive(Debug)]
struct BoardState([Piece; 9]);

impl BoardState {
    fn is_valid(&self, m: u8) -> bool {
        m < 9 && self.0[m as usize] == Piece::Nil
    }
}

#[derive(Debug)]
pub struct GameState {
    board: BoardState,
    turn: Turn,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: BoardState([Piece::Nil; 9]),
            turn: Turn::Player1,
        }
    }

    pub fn make_move(&mut self, m: u8) -> Result<(), String> {
        if !self.board.is_valid(m) {
            return Err("Invalid move".to_string());
        }

        let piece = match self.turn {
            Turn::Player1 => Piece::O,
            Turn::Player2 => Piece::X,
        };

        self.board.0[m as usize] = piece;
        self.turn = self.turn.next();

        Ok(())
    }
}
