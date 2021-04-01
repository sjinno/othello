use std::io::{self, Write};

use crate::board::{Board, Cell};

#[derive(Clone, Copy, PartialEq)]
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
        match Self::get_move(board, turn) {
            Move::Play(r, c) => match turn {
                Turn::Black => {
                    board.board[r][c] = Cell::Black;
                    board.board[0][0] = Cell::Indicator(Turn::White);
                    Board::validate_cells(&mut board.board);
                    Turn::White
                }
                Turn::White => {
                    board.board[r][c] = Cell::White;
                    board.board[0][0] = Cell::Indicator(Turn::Black);
                    Board::validate_cells(&mut board.board);
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
    fn get_move(board: &mut Board, turn: Turn) -> Move;
    fn get_col_input() -> usize;
    fn get_row_input() -> usize;
    fn is_valid_move(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool;
    fn flip_discs(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool;
    fn try_flipping_up(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool;
    // fn try_flipping_down(board: &Board, turn: Turn, row: usize, col: usize) -> bool;
    // fn try_flipping_left(board: &Board, turn: Turn, row: usize, col: usize) -> bool;
    // fn try_flipping_right(board: &Board, turn: Turn, row: usize, col: usize) -> bool;
    // fn try_flipping_up_left(board: &Board, turn: Turn, row: usize, col: usize) -> bool;
    // fn try_flipping_up_right(board: &Board, turn: Turn, row: usize, col: usize) -> bool;
    // fn try_flipping_down_left(board: &Board, turn: Turn, row: usize, col: usize) -> bool;
    // fn try_flipping_down_right(board: &Board, turn: Turn, row: usize, col: usize) -> bool;
}

impl Handler for Move {
    fn get_move(board: &mut Board, turn: Turn) -> Move {
        let col = Self::get_col_input();
        let row = Self::get_row_input();
        if Self::is_valid_move(board, turn, row, col) {
            Move::Play(row, col)
        } else {
            Self::get_move(board, turn)
        }
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

    fn is_valid_move(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool {
        match board.board[row][col] {
            Cell::Okay => Self::flip_discs(board, turn, row, col),
            _ => false,
        }
    }

    fn flip_discs(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool {
        Self::try_flipping_up(board, turn, row, col)
    }

    fn try_flipping_up(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool {
        println!("row = {}, col = {}", row, col);
        match (row, col) {
            (1..=2, _) => false,
            _ => match turn {
                Turn::Black => {
                    if board.board[row - 1][col] == Cell::White {
                        println!("I'm here :)");
                        let mut count = 2;
                        while row - count != 0 {
                            if board.board[row - count][col] == Cell::White {
                                count += 1;
                                continue;
                            } else if board.board[row - count][col] == Cell::Black {
                                (1..count).for_each(|r| {
                                    board.board[row - r][col] = Cell::Black;
                                });
                                return true;
                            } else {
                                return false;
                            }
                        }
                        false
                    } else {
                        false
                    }
                }
                Turn::White => {
                    if board.board[row - 1][col] == Cell::Black {
                        println!("I'm here :)");
                        let mut count = 2;
                        while row - count != 0 {
                            if board.board[row - count][col] == Cell::Black {
                                count += 1;
                                continue;
                            } else if board.board[row - count][col] == Cell::White {
                                (1..count).for_each(|r| {
                                    board.board[row - r][col] = Cell::White;
                                });
                                return true;
                            } else {
                                return false;
                            }
                        }
                        false
                    } else {
                        false
                    }
                }
                _ => true,
            },
        }
    }
}
