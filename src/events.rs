use std::io;
use std::thread::sleep;
use std::time::Duration;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::board::{Board, Cell};
use crate::{check, flip};

#[derive(Clone, Copy, PartialEq)]
pub enum Turn {
    Black,
    White,
    Neither,
}

#[derive(Copy, Clone)]
pub enum Move {
    Play(usize, usize),
    Pass,
    Skip,
    Resign,
    Win(u8, u8),
}

#[derive(EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Move {
    pub fn handle_move(board: &mut Board, turn: Turn) -> (Turn, Option<Move>) {
        match Self::get_move(board, turn) {
            Move::Play(r, c) => match turn {
                Turn::Black => {
                    board.board[r][c] = Cell::Black;
                    Board::validate_cells(&mut board.board);

                    let res = Self::check_end_game(board);
                    if res.0 {
                        return (res.1, Some(Move::Win(res.2, res.3)));
                    }

                    // Checks if the other player has available cells
                    // that they can play.
                    //
                    // If not, you will be automatically given a turn again.
                    if Self::check_playablity(board, turn) {
                        board.board[0][0] = Cell::Indicator(Turn::White);
                        (Turn::White, None)
                    } else {
                        (Turn::Black, Some(Move::Skip))
                    }
                }
                Turn::White => {
                    board.board[r][c] = Cell::White;
                    Board::validate_cells(&mut board.board);

                    let res = Self::check_end_game(board);
                    if res.0 {
                        return (res.1, Some(Move::Win(res.2, res.3)));
                    }

                    if Self::check_playablity(board, turn) {
                        board.board[0][0] = Cell::Indicator(Turn::Black);
                        (Turn::Black, None)
                    } else {
                        (Turn::White, Some(Move::Skip))
                    }
                }
                _ => (Turn::Neither, None),
            },
            Move::Pass => match turn {
                Turn::Black => {
                    if Self::check_playablity(board, turn) {
                        board.board[0][0] = Cell::Indicator(Turn::White);
                        (Turn::White, Some(Move::Pass))
                    } else {
                        println!("Cannot pass.");
                        sleep(Duration::from_secs_f32(1.5));
                        (Turn::Black, None)
                    }
                }
                Turn::White => {
                    if Self::check_playablity(board, turn) {
                        board.board[0][0] = Cell::Indicator(Turn::Black);
                        (Turn::Black, Some(Move::Pass))
                    } else {
                        println!("Cannot pass.");
                        sleep(Duration::from_secs_f32(1.5));
                        (Turn::White, None)
                    }
                }
                _ => (Turn::Neither, None),
            },
            Move::Resign => match turn {
                Turn::Black => (Turn::Black, Some(Move::Resign)),
                Turn::White => (Turn::White, Some(Move::Resign)),
                _ => (Turn::Neither, None),
            },
            _ => (Turn::Neither, None),
        }
    }
}

trait InputHandler {
    fn get_move(board: &mut Board, turn: Turn) -> Move;
    fn get_input() -> Move;
    fn is_valid_move(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool;
    fn flip_discs(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool;
    fn try_flipping_up(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool;
    fn try_flipping_down(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool;
    fn try_flipping_left(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool;
    fn try_flipping_right(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool;
    fn try_flipping_up_left(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool;
    fn try_flipping_up_right(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool;
    fn try_flipping_down_left(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool;
    fn try_flipping_down_right(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool;
}

impl InputHandler for Move {
    fn get_move(board: &mut Board, turn: Turn) -> Move {
        match Self::get_input() {
            Move::Play(row, col) => {
                // This is where flipping discs happens if and only if
                // there are flippable dics, and returns `Move::Play(_, _)`.
                //
                // Otherwise, prompt the player to input `row` and `col` again.
                if Self::is_valid_move(board, turn, row, col) {
                    Move::Play(row, col)
                } else {
                    println!("Invalid move. Please try again.");
                    Self::get_move(board, turn)
                }
            }
            mv => mv,
        }
    }

    fn get_input() -> Move {
        println!("Enter your move. (Example: 3d)");
        println!("Enter `p` to pass or `r` to resign.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input = input
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect::<Vec<_>>();

        match input.len() {
            1 => match input[0] {
                'p' => Move::Pass,
                'r' => Move::Resign,
                _ => Self::get_input(),
            },
            2 => match input[0] {
                c if ('1'..='8').contains(&c) => match input[1].to_ascii_lowercase() {
                    r if ('a'..='h').contains(&r) => {
                        let row = r.to_digit(18).unwrap() as usize - 9;
                        let col = c.to_digit(10).unwrap() as usize;
                        Move::Play(row, col)
                    }
                    _ => Self::get_input(),
                },
                _ => Self::get_input(),
            },
            _ => Self::get_input(),
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
            | Self::try_flipping_down(board, turn, row, col)
            | Self::try_flipping_left(board, turn, row, col)
            | Self::try_flipping_right(board, turn, row, col)
            | Self::try_flipping_up_left(board, turn, row, col)
            | Self::try_flipping_up_right(board, turn, row, col)
            | Self::try_flipping_down_left(board, turn, row, col)
            | Self::try_flipping_down_right(board, turn, row, col)
    }

    fn try_flipping_up(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool {
        match (row, col) {
            (1..=2, _) => false,
            _ => match turn {
                Turn::Black => flip!(board, Cell::Black, Cell::White, Direction::Up, row, col),
                Turn::White => flip!(board, Cell::White, Cell::Black, Direction::Up, row, col),
                _ => true,
            },
        }
    }

    fn try_flipping_down(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool {
        match (row, col) {
            (7..=8, _) => false,
            _ => match turn {
                Turn::Black => flip!(board, Cell::Black, Cell::White, Direction::Down, row, col),
                Turn::White => flip!(board, Cell::White, Cell::Black, Direction::Down, row, col),
                _ => true,
            },
        }
    }

    fn try_flipping_left(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool {
        match (row, col) {
            (_, 1..=2) => false,
            _ => match turn {
                Turn::Black => flip!(board, Cell::Black, Cell::White, Direction::Left, row, col),
                Turn::White => flip!(board, Cell::White, Cell::Black, Direction::Left, row, col),
                _ => true,
            },
        }
    }

    fn try_flipping_right(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool {
        match (row, col) {
            (_, 7..=8) => false,
            _ => match turn {
                Turn::Black => flip!(board, Cell::Black, Cell::White, Direction::Right, row, col),
                Turn::White => flip!(board, Cell::White, Cell::Black, Direction::Right, row, col),
                _ => true,
            },
        }
    }

    fn try_flipping_up_left(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool {
        match (row, col) {
            (1..=2, _) | (_, 1..=2) => false,
            _ => match turn {
                Turn::Black => flip!(board, Cell::Black, Cell::White, Direction::UpLeft, row, col),
                Turn::White => flip!(board, Cell::White, Cell::Black, Direction::UpLeft, row, col),
                _ => true,
            },
        }
    }

    fn try_flipping_up_right(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool {
        match (row, col) {
            (1..=2, _) | (_, 7..=8) => false,
            _ => match turn {
                Turn::Black => flip!(
                    board,
                    Cell::Black,
                    Cell::White,
                    Direction::UpRight,
                    row,
                    col
                ),
                Turn::White => flip!(
                    board,
                    Cell::White,
                    Cell::Black,
                    Direction::UpRight,
                    row,
                    col
                ),
                _ => true,
            },
        }
    }

    fn try_flipping_down_left(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool {
        match (row, col) {
            (7..=8, _) | (_, 1..=2) => false,
            _ => match turn {
                Turn::Black => flip!(
                    board,
                    Cell::Black,
                    Cell::White,
                    Direction::DownLeft,
                    row,
                    col
                ),
                Turn::White => flip!(
                    board,
                    Cell::White,
                    Cell::Black,
                    Direction::DownLeft,
                    row,
                    col
                ),
                _ => true,
            },
        }
    }

    fn try_flipping_down_right(board: &mut Board, turn: Turn, row: usize, col: usize) -> bool {
        match (row, col) {
            (7..=8, _) | (_, 7..=8) => false,
            _ => match turn {
                Turn::Black => flip!(
                    board,
                    Cell::Black,
                    Cell::White,
                    Direction::DownRight,
                    row,
                    col
                ),
                Turn::White => flip!(
                    board,
                    Cell::White,
                    Cell::Black,
                    Direction::DownRight,
                    row,
                    col
                ),
                _ => true,
            },
        }
    }
}

trait PlayabilityChecker {
    fn check_playablity(board: &Board, turn: Turn) -> bool;
    fn check_end_game(board: &Board) -> (bool, Turn, u8, u8);
}

impl PlayabilityChecker for Move {
    fn check_playablity(board: &Board, turn: Turn) -> bool {
        match turn {
            Turn::Black => Direction::iter().any(|d| check!(board, Cell::White, Cell::Black, d)),
            Turn::White => Direction::iter().any(|d| check!(board, Cell::Black, Cell::White, d)),
            _ => false,
        }
    }

    fn check_end_game(board: &Board) -> (bool, Turn, u8, u8) {
        if !Self::check_playablity(board, Turn::Black)
            && !Self::check_playablity(board, Turn::White)
        {
            check!(board)
        }
        (false, Turn::Neither, 0, 0)
    }
}
