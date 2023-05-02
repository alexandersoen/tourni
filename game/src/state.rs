use std::slice::Iter;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Piece {
    O,
    X,
    Nil,
}

impl Piece {
    fn to_player(&self) -> Option<Player> {
        match self {
            Piece::O => Some(Player::Player1),
            Piece::X => Some(Player::Player2),
            Piece::Nil => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn iter_all() -> Iter<'static, Self> {
        static PLAYERS: [Player; 2] = [Player::Player1, Player::Player2];
        PLAYERS.iter()
    }

    fn next(&self) -> Self {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }

    fn to_piece(&self) -> Option<Piece> {
        match self {
            Player::Player1 => Some(Piece::O),
            Player::Player2 => Some(Piece::X),
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
    pub turn: Player,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: BoardState([Piece::Nil; 9]),
            turn: Player::Player1,
        }
    }

    pub fn make_move(&mut self, m: u8) -> Result<(), String> {
        if !self.board.is_valid(m) {
            return Err("Invalid move".to_string());
        }

        let piece = self
            .turn
            .to_piece()
            .ok_or("Unidentifiable piece".to_string())?;

        self.board.0[m as usize] = piece;
        self.turn = self.turn.next();

        Ok(())
    }

    pub fn winner(&self) -> Option<Player> {
        if self.board.0[0] == self.board.0[1] && self.board.0[0] == self.board.0[2] {
            return self.board.0[0].to_player();
        }

        if self.board.0[0] == self.board.0[3] && self.board.0[0] == self.board.0[6] {
            return self.board.0[0].to_player();
        }

        if self.board.0[2] == self.board.0[5] && self.board.0[2] == self.board.0[8] {
            return self.board.0[2].to_player();
        }

        if self.board.0[6] == self.board.0[7] && self.board.0[6] == self.board.0[8] {
            return self.board.0[6].to_player();
        }

        if self.board.0[0] == self.board.0[4] && self.board.0[0] == self.board.0[8] {
            return self.board.0[0].to_player();
        }

        if self.board.0[2] == self.board.0[4] && self.board.0[2] == self.board.0[6] {
            return self.board.0[2].to_player();
        }

        None
    }
}
