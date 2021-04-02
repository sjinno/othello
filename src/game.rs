use crate::board::Board;
use crate::events::{Move, Turn};

pub enum Game {
    On,
    Off,
}

impl Game {
    pub fn start() {
        let mut board = Board::new();
        board.draw();
        let mut mode = Game::On;
        let mut turn = Turn::Black;
        loop {
            match mode {
                Game::On => {
                    match turn {
                        Turn::Black => {
                            turn = Move::handle_move(&mut board, turn);
                        }
                        Turn::White => {
                            turn = Move::handle_move(&mut board, turn);
                        }
                        Turn::Neither => mode = Game::Off,
                    }
                    board.draw();
                }
                Game::Off => break,
            }
        }
    }
}
