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
    Resign,
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
            Move::Resign => Turn::Neither,
        }
    }
}

trait Handler {
    // fn is_valid_move(board: &Board, turn: Turn) -> bool;
    fn get_move() -> Move;
    fn get_col_input() -> usize;
    fn get_row_input() -> usize;
}

impl Handler for Move {
    fn get_move() -> Move {
        let col = Self::get_col_input();
        let row = Self::get_row_input();
        Move::Play(row, col)
    }

    fn get_col_input() -> usize {
        print!("Which column? ");
        io::stdout().flush().unwrap();

        let mut col = String::new();
        io::stdin()
            .read_line(&mut col)
            .expect("failed to read line");

        match col.trim().parse() {
            Ok(num) if (1..=8).contains(&num) => num,
            _ => Self::get_col_input(),
        }
    }

    fn get_row_input() -> usize {
        print!("Which row? ");
        io::stdout().flush().unwrap();

        let mut row = String::new();
        io::stdin()
            .read_line(&mut row)
            .expect("failed to read line");

        match row.trim().parse() {
            Ok(num) if (1..=8).contains(&num) => num,
            _ => Self::get_row_input(),
        }
    }
}
