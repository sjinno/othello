use crate::board;

#[derive(Clone)]
pub enum Turn {
    Black,
    White,
    Neither,
}

enum Move {
    Play(usize, usize),
    Pass,
}

enum GameMode {
    On,
    Off,
}

pub fn game() {
    let board = board::Board::new();
    board.draw();
    let mut mode = GameMode::On;
    let mut turn = Turn::Black;
    loop {
        match mode {
            GameMode::On => match turn {
                Turn::Black => turn = Turn::White,
                Turn::White => turn = Turn::Black,
                Turn::Neither => mode = GameMode::Off,
            },
            GameMode::Off => break,
        }
    }
}
