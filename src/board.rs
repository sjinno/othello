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

// Might not work well if you have light themed terminal.
// If that's the case, feel free to modify colors.
// Or you can maybe simply apply dark theme. :)
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Feel free to change colors.
            // Color reference can be found at:
            // https://docs.rs/colored/2.0.0/colored/
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
    pub cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            cells: vec![vec![Cell::Illegal; SIZE]; SIZE],
        };
        board.set_initial_state_and_label();
        board.validate_cells();
        board
    }

    pub fn draw(&self, turn: Turn, mv: Option<Move>) {
        print!("\x1B[2J\x1B[1;1H");
        for row in self.cells.iter() {
            for col in row {
                print!("{}", col);
            }
            println!();
        }
        match mv {
            Some(Move::Pass) => match turn {
                Turn::Black => println!("{}", "White passed.".yellow()),
                Turn::White => println!("{}", "Black passed.".yellow()),
                _ => {}
            },
            Some(Move::Skip) => match turn {
                Turn::Black => {
                    println!(
                        "{}",
                        "Couln't find available moves for White.\nSkipping it's turn...".yellow()
                    )
                }
                Turn::White => {
                    println!(
                        "{}",
                        "Couln't find available moves for Black.\nSkipping it's turn...".yellow()
                    )
                }
                _ => {}
            },
            Some(Move::Resign) => match turn {
                Turn::Black => println!("{}", "Black has resigned.\nWhite wins.".purple()),
                Turn::White => println!("{}", "White has resigned.\nBlack wins.".purple()),
                _ => {}
            },
            Some(Move::Win(b, w)) => match turn {
                Turn::Black => {
                    println!("{}", format!("Black wins by {} points.", b - w).purple())
                }
                Turn::White => {
                    println!("{}", format!("White wins by {} points.", w - b).purple())
                }
                Turn::Neither => println!("TIE!!"),
            },
            _ => {}
        }
    }

    fn set_initial_state_and_label(&mut self) {
        self.cells[0][0] = Cell::Indicator(Turn::Black); // Indicator located at top left corner.
        self.cells[4][4] = Cell::White; // Initial position of White.
        self.cells[5][5] = Cell::White; // Initial position of White.
        self.cells[4][5] = Cell::Black; // Initial position of Black
        self.cells[5][4] = Cell::Black; // Initial position of Black

        //# Labeling:
        for (i, c) in ('1'..='8').enumerate() {
            self.cells[0][i + 1] = Cell::Label(c);
        }
        for (i, c) in ('A'..='H').enumerate() {
            self.cells[i + 1][0] = Cell::Label(c);
        }
        //# Labeling ends.
    }

    // This function validates whether or not empty cells are okay to be played.
    pub fn validate_cells(&mut self) {
        for row in 1..SIZE {
            for col in 1..SIZE {
                match self.cells[row][col] {
                    Cell::Black | Cell::White => {
                        match (row, col) {
                            //# Corner edge cases:
                            (1, 1) => {
                                self.validate(1, 2); // R
                                self.validate(2, 1); // D
                                self.validate(2, 2); // DR
                            }
                            (1, 8) => {
                                self.validate(1, 7); // L
                                self.validate(2, 8); // D
                                self.validate(2, 7); // DL
                            }
                            (8, 1) => {
                                self.validate(8, 2); // R
                                self.validate(7, 1); // U
                                self.validate(7, 2); // UR
                            }
                            (8, 8) => {
                                self.validate(8, 7); // L
                                self.validate(7, 8); // U
                                self.validate(7, 7); // UL
                            }
                            //# Corner edge cases end.
                            (1, 2..=7) => {
                                self.validate(row, col - 1); // L
                                self.validate(row, col + 1); // R
                                self.validate(row + 1, col); // D
                                self.validate(row + 1, col - 1); // DL
                                self.validate(row + 1, col + 1); // DR
                            }
                            (8, 2..=7) => {
                                self.validate(row, col - 1); // L
                                self.validate(row, col + 1); // R
                                self.validate(row - 1, col); // U
                                self.validate(row - 1, col - 1); // UL
                                self.validate(row - 1, col + 1); // UR
                            }
                            (2..=7, 1) => {
                                self.validate(row - 1, col); // U
                                self.validate(row + 1, col); // D
                                self.validate(row, col + 1); // R
                                self.validate(row - 1, col + 1); // UR
                                self.validate(row + 1, col + 1); // DR
                            }
                            (2..=7, 8) => {
                                self.validate(row - 1, col); // U
                                self.validate(row + 1, col); // D
                                self.validate(row, col - 1); // L
                                self.validate(row - 1, col - 1); // UL
                                self.validate(row + 1, col - 1); // DL
                            }
                            _ => {
                                self.validate(row - 1, col - 1); // UL
                                self.validate(row - 1, col + 1); // UR
                                self.validate(row + 1, col - 1); // DL
                                self.validate(row + 1, col + 1); // DR
                                self.validate(row - 1, col); // U
                                self.validate(row + 1, col); // D
                                self.validate(row, col - 1); // L
                                self.validate(row, col + 1); // R
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn validate(&mut self, row: usize, col: usize) {
        if self.cells[row][col] == Cell::Illegal {
            self.cells[row][col] = Cell::Okay;
        }
    }

    pub fn count_scores(&self) -> (bool, Turn, Option<u8>, Option<u8>) {
        let mut black_count = 0;
        let mut white_count = 0;
        for row in 1..SIZE {
            for col in 1..SIZE {
                if self.cells[row][col] == Cell::Black {
                    black_count += 1;
                } else if self.cells[row][col] == Cell::White {
                    white_count += 1;
                }
            }
        }

        if black_count > white_count {
            (true, Turn::Black, Some(black_count), Some(white_count))
        } else if black_count < white_count {
            (true, Turn::White, Some(black_count), Some(white_count))
        } else {
            (true, Turn::Neither, Some(black_count), Some(white_count))
        }
    }
}
