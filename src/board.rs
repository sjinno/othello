use std::fmt::{self, Write};

const SIZE: usize = 8; // SIZE = HEIGHT = WIDTH

#[derive(Clone)]
enum Cell {
    Okay,
    Black,
    White,
    Illegal,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Black => f.write_str("● "),
            Cell::White => f.write_str("○ "),
            Cell::Okay => f.write_char('・'),
            Cell::Illegal => f.write_str("x "),
        }
    }
}

pub struct Board(Vec<Vec<Cell>>);

impl Board {
    pub fn new() -> Self {
        let mut board = vec![vec![Cell::Illegal; SIZE]; SIZE];
        board[3][3] = Cell::White;
        board[4][4] = Cell::White;
        board[3][4] = Cell::Black;
        board[4][3] = Cell::Black;
        Self::validate_cells(&mut board);
        Board(board)
    }

    pub fn draw(&self) {
        for row in self.0.iter() {
            for col in row {
                print!("{}", col);
            }
            println!();
        }
    }
}

trait Logic {
    fn validate_cells(board: &mut Vec<Vec<Cell>>);
    fn validate(row: usize, col: usize, board: &mut Vec<Vec<Cell>>);
}

impl Logic for Board {
    fn validate_cells(board: &mut Vec<Vec<Cell>>) {
        for row in 0..SIZE {
            for col in 0..SIZE {
                match board[row][col] {
                    Cell::Black | Cell::White => {
                        match (row, col) {
                            (0, 0) => {
                                Self::validate(0, 1, board); // R
                                Self::validate(1, 0, board); // D
                                Self::validate(1, 1, board); // DR
                            }
                            (0, 7) => {
                                Self::validate(0, 6, board); // L
                                Self::validate(1, 7, board); // D
                                Self::validate(1, 6, board); // DL
                            }
                            (7, 0) => {
                                Self::validate(7, 1, board); // R
                                Self::validate(6, 0, board); // U
                                Self::validate(6, 1, board); // UR
                            }
                            (7, 7) => {
                                Self::validate(7, 6, board); // L
                                Self::validate(6, 7, board); // U
                                Self::validate(6, 6, board); // UL
                            }
                            (0, 1..=6) => {
                                Self::validate(row, col + 1, board); // L
                                Self::validate(row, col - 1, board); // R
                                Self::validate(row + 1, col, board); // D
                                Self::validate(row + 1, col + 1, board); // DL
                                Self::validate(row + 1, col - 1, board); // DR
                            }
                            (7, 1..=6) => {
                                Self::validate(row, col + 1, board); // L
                                Self::validate(row, col - 1, board); // R
                                Self::validate(row - 1, col, board); // U
                                Self::validate(row - 1, col + 1, board); // UL
                                Self::validate(row - 1, col - 1, board); // UR
                            }
                            (1..=6, 0) => {
                                Self::validate(row - 1, col, board); // U
                                Self::validate(row + 1, col, board); // D
                                Self::validate(row, col - 1, board); // R
                                Self::validate(row - 1, col - 1, board); // UR
                                Self::validate(row + 1, col - 1, board); // DR
                            }
                            (1..=6, 7) => {
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
