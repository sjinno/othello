use crate::board::Board;
use crate::events;

#[derive(Clone)]
pub enum Turn {
    Black,
    White,
    Neither,
}

enum GameMode {
    On,
    Off,
}

pub fn game() {
    let mut board = Board::new();
    board.draw();
    let mut mode = GameMode::On;
    let mut turn = Turn::Black;
    loop {
        match mode {
            GameMode::On => {
                match turn {
                    Turn::Black => {
                        turn = events::handle_move(&mut board, turn);
                    }
                    Turn::White => {
                        turn = events::handle_move(&mut board, turn);
                    }
                    Turn::Neither => mode = GameMode::Off,
                }
                board.draw();
            }
            GameMode::Off => break,
        }
    }
}
