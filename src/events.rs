use std::io::{self, Write};

use crate::board::{Board, Cell};

#[derive(Clone)]
pub enum Turn {
    Black,
    White,
    Neither,
}

pub enum Move {
    Play(usize, usize),
    Pass,
}

impl Move {
    pub fn handle_move(board: &mut Board, turn: Turn) -> Turn {
        match Self::get_move() {
            Move::Play(r, c) => match turn {
                Turn::Black => {
                    board.board[r][c] = Cell::Black;
                    board.board[0][0] = Cell::Indicator(Turn::White);
                    Turn::White
                }
                Turn::White => {
                    board.board[r][c] = Cell::White;
                    board.board[0][0] = Cell::Indicator(Turn::Black);
                    Turn::Black
                }
                // Automatically set `turn` to `Neither`
                // when there are no empty spaces.
                //
                // Or when each player passes their turn consecutively.
                _ => Turn::Neither,
            },
            Move::Pass => match turn {
                Turn::Black => Turn::White,
                Turn::White => Turn::Black,
                _ => Turn::Neither,
            },
        }
    }
}

trait Handler {
    // fn is_valid_move(board: &Board, turn: Turn) -> bool;
    fn get_move() -> Move;
    // fn set_move(board: &mut Board, turn: Turn);
}

impl Handler for Move {
    fn get_move() -> Move {
        print!("Which column? ");
        io::stdout().flush().unwrap();
        let mut col = String::new();
        io::stdin()
            .read_line(&mut col)
            .expect("failed to read line");
        let col = col.trim().chars().next().unwrap();

        print!("Which row? ");
        io::stdout().flush().unwrap();
        let mut row = String::new();
        io::stdin()
            .read_line(&mut row)
            .expect("failed to read line");
        let row = row.trim().chars().next().unwrap();

        match (row, col) {
            ('1'..='8', '1'..='8') => Move::Play(
                row.to_digit(9).unwrap() as usize,
                col.to_digit(9).unwrap() as usize,
            ),
            _ => Self::get_move(),
        }
    }
}
