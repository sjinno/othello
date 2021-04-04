use crate::board::Board;
use crate::events::{Move, Turn};

pub enum Game {
    On,
    Off,
}

impl Game {
    pub fn start() {
        let mut board = Board::new();
        let mut mode = Game::On;
        let mut turn = Turn::Black;
        let mut mv: Option<Move> = None;
        board.draw(turn, mv);
        loop {
            match mode {
                Game::On => match turn {
                    Turn::Black => {
                        let res = Move::handle_move(&mut board, turn);
                        turn = res.0;
                        mv = res.1;
                        board.draw(turn, mv);
                    }
                    Turn::White => {
                        let res = Move::handle_move(&mut board, turn);
                        turn = res.0;
                        mv = res.1;
                        board.draw(turn, mv);
                    }
                    Turn::Neither => mode = Game::Off,
                },
                Game::Off => {
                    break;
                }
            }
            // When either of `Resign, Dominate, Win` happens, game ends.
            if let Some(Move::Resign) | Some(Move::Dominate) | Some(Move::Win(_, _)) = mv {
                turn = Turn::Neither
            }
        }
    }
}
