use std::fmt::{self, Write};

use colored::Colorize;

use crate::events::Turn;

const SIZE: usize = 9; // HEIGHT = WIDTH = 8, but +1 for labels.

#[derive(Clone, PartialEq)]
pub enum Cell {
    Black,
    White,
    Okay,
    Illegal,
    Label(char),
    Indicator(Turn),
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Black => f.write_str("● "),
            Cell::White => f.write_str("○ "),
            Cell::Okay => f.write_char('・'),
            Cell::Illegal => f.write_str("x "),
            Cell::Label(c) => write!(f, "{} ", *c),
            Cell::Indicator(t) => match t {
                Turn::Black => write!(f, "{}", "● ".magenta()),
                Turn::White => write!(f, "{}", "○ ".magenta()),
                Turn::Neither => f.write_str("  "),
            },
        }
    }
}

pub struct Board {
    pub board: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = vec![vec![Cell::Illegal; SIZE]; SIZE];
        Self::set_initial_state_and_label(&mut board);
        Self::validate_cells(&mut board);
        Self { board }
    }

    pub fn draw(&self) {
        println!("\x1B[2J\x1B[1;1H");
        for row in self.board.iter() {
            for col in row {
                print!("{}", col);
            }
            println!();
        }
    }

    fn set_initial_state_and_label(board: &mut Vec<Vec<Cell>>) {
        board[0][0] = Cell::Indicator(Turn::Black);
        board[4][4] = Cell::White;
        board[5][5] = Cell::White;
        board[4][5] = Cell::Black;
        board[5][4] = Cell::Black;
        //# Labeling:
        for (i, c) in ('1'..='8').enumerate() {
            board[0][i + 1] = Cell::Label(c);
            board[i + 1][0] = Cell::Label(c);
        }
        // for (i, c) in ('A'..='H').enumerate() {
        //     board[i + 1][0] = Cell::Label(c);
        // }
        //# Labeling ends.
    }

    pub fn validate_cells(board: &mut Vec<Vec<Cell>>) {
        for row in 1..SIZE {
            for col in 1..SIZE {
                match board[row][col] {
                    Cell::Black | Cell::White => {
                        match (row, col) {
                            (1, 1) => {
                                Self::validate(1, 2, board); // R
                                Self::validate(2, 1, board); // D
                                Self::validate(2, 2, board); // DR
                            }
                            (1, 8) => {
                                Self::validate(1, 7, board); // L
                                Self::validate(2, 8, board); // D
                                Self::validate(2, 7, board); // DL
                            }
                            (8, 1) => {
                                Self::validate(8, 2, board); // R
                                Self::validate(7, 1, board); // U
                                Self::validate(7, 2, board); // UR
                            }
                            (8, 8) => {
                                Self::validate(8, 7, board); // L
                                Self::validate(7, 8, board); // U
                                Self::validate(7, 7, board); // UL
                            }
                            (1, 2..=7) => {
                                Self::validate(row, col + 1, board); // L
                                Self::validate(row, col - 1, board); // R
                                Self::validate(row + 1, col, board); // D
                                Self::validate(row + 1, col + 1, board); // DL
                                Self::validate(row + 1, col - 1, board); // DR
                            }
                            (8, 2..=7) => {
                                Self::validate(row, col + 1, board); // L
                                Self::validate(row, col - 1, board); // R
                                Self::validate(row - 1, col, board); // U
                                Self::validate(row - 1, col + 1, board); // UL
                                Self::validate(row - 1, col - 1, board); // UR
                            }
                            (2..=7, 1) => {
                                Self::validate(row - 1, col, board); // U
                                Self::validate(row + 1, col, board); // D
                                Self::validate(row, col - 1, board); // R
                                Self::validate(row - 1, col - 1, board); // UR
                                Self::validate(row + 1, col - 1, board); // DR
                            }
                            (2..=7, 8) => {
                                Self::validate(row - 1, col, board); // U
                                Self::validate(row + 1, col, board); // D
                                Self::validate(row, col + 1, board); // L
                                Self::validate(row - 1, col + 1, board); // UL
                                Self::validate(row + 1, col + 1, board); // DL
                            }
                            _ => {
                                Self::validate(row - 1, col - 1, board); // UL
                                Self::validate(row - 1, col + 1, board); // UR
                                Self::validate(row + 1, col - 1, board); // DL
                                Self::validate(row + 1, col + 1, board); // DR
                                Self::validate(row - 1, col, board); // U
                                Self::validate(row + 1, col, board); // D
                                Self::validate(row, col - 1, board); // L
                                Self::validate(row, col + 1, board); // R
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn validate(row: usize, col: usize, board: &mut Vec<Vec<Cell>>) {
        match board[row][col] {
            Cell::Illegal => board[row][col] = Cell::Okay,
            _ => {}
        }
    }
}
