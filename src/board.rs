use std::fmt;

use colored::Colorize;

use crate::events::{Move, Turn};

pub const SIZE: usize = 9; // HEIGHT = WIDTH = 8, but +1 for labels.

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
            Cell::Black => write!(f, "{}", "● ".black().on_green()),
            Cell::White => write!(f, "{}", "● ".on_green()),
            Cell::Okay => write!(f, "{}", "・".on_green()),
            Cell::Illegal => write!(f, "{}", "・".on_green()),
            Cell::Label(c) => write!(f, "{} ", *c),
            Cell::Indicator(t) => match t {
                Turn::Black => write!(f, "{}", "● ".green()),
                Turn::White => write!(f, "{}", "○ ".green()),
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

    pub fn draw(&self, turn: Turn, mv: Option<Move>) {
        print!("\x1B[2J\x1B[1;1H");
        match mv {
            Some(Move::Pass) => match turn {
                Turn::Black => println!("White passed."),
                Turn::White => println!("Black passed."),
                _ => {}
            },
            Some(Move::Resign) => match turn {
                Turn::Black => println!("Black has resigned.\nWhite wins."),
                Turn::White => println!("White has resigned.\nBlack wins."),
                _ => {}
            },
            Some(Move::Dominate) => match turn {
                Turn::Black => println!("Black has dominated the board.\nBlack wins."),
                Turn::White => println!("White has dominated the board.\nWhite wins."),
                _ => {}
            },
            Some(Move::Win(score)) => match turn {
                Turn::Black => println!("Black wins by {} points.", score),
                Turn::White => println!("White wins by {} points.", score),
                Turn::Neither => println!("TIE!!"),
            },
            _ => {}
        }
        for row in self.board.iter() {
            for col in row {
                print!("{}", col);
            }
            println!();
        }
    }

    fn set_initial_state_and_label(board: &mut Vec<Vec<Cell>>) {
        board[0][0] = Cell::Indicator(Turn::Black); // Indicator located at top left corner.
        board[4][4] = Cell::White; // Initial position of White.
        board[5][5] = Cell::White; // Initial position of White.
        board[4][5] = Cell::Black; // Initial position of Black
        board[5][4] = Cell::Black; // Initial position of Black

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

    // This function validates whether or not empty cells are playable.
    pub fn validate_cells(board: &mut Vec<Vec<Cell>>) {
        for row in 1..SIZE {
            for col in 1..SIZE {
                match board[row][col] {
                    Cell::Black | Cell::White => {
                        match (row, col) {
                            //# Corner edge cases:
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
                            //# Corner edge cases end.
                            (1, 2..=7) => {
                                Self::validate(row, col - 1, board); // L
                                Self::validate(row, col + 1, board); // R
                                Self::validate(row + 1, col, board); // D
                                Self::validate(row + 1, col - 1, board); // DL
                                Self::validate(row + 1, col + 1, board); // DR
                            }
                            (8, 2..=7) => {
                                Self::validate(row, col - 1, board); // L
                                Self::validate(row, col + 1, board); // R
                                Self::validate(row - 1, col, board); // U
                                Self::validate(row - 1, col - 1, board); // UL
                                Self::validate(row - 1, col + 1, board); // UR
                            }
                            (2..=7, 1) => {
                                Self::validate(row - 1, col, board); // U
                                Self::validate(row + 1, col, board); // D
                                Self::validate(row, col + 1, board); // R
                                Self::validate(row - 1, col + 1, board); // UR
                                Self::validate(row + 1, col + 1, board); // DR
                            }
                            (2..=7, 8) => {
                                Self::validate(row - 1, col, board); // U
                                Self::validate(row + 1, col, board); // D
                                Self::validate(row, col - 1, board); // L
                                Self::validate(row - 1, col - 1, board); // UL
                                Self::validate(row + 1, col - 1, board); // DL
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
        if board[row][col] == Cell::Illegal {
            board[row][col] = Cell::Okay
        }
    }
}
