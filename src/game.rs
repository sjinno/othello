use std::io;

use crate::board::{self, Board};

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
    let mut board = Board::new();
    board.draw();
    let mut mode = GameMode::On;
    let mut turn = Turn::Black;
    loop {
        match mode {
            GameMode::On => {
                match turn {
                    Turn::Black => {
                        make_move(&mut board);
                        turn = Turn::White;
                    }
                    Turn::White => turn = Turn::Black,
                    Turn::Neither => mode = GameMode::Off,
                }

                board.draw();
            }
            GameMode::Off => break,
        }
    }
}

fn make_move(board: &mut Board) -> Move {
    let mut mv = String::new();
    io::stdin().read_line(&mut mv).expect("failed to read line");

    Move::Pass
}

// trait Logic {
//     fn make_move(&mut self) -> Move;
//     fn is_valid_move(&self) -> bool;
// }
