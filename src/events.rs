use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::board::{Board, Cell};
use crate::{check, flip, option};

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
    Resign,
    Undo,
    Dominate,
    Win(u8),
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
                        return (res.1, Some(Move::Win(res.2)));
                    }

                    if Self::check_automatic_win(board, Cell::White) {
                        return (Turn::Black, Some(Move::Dominate));
                    }

                    if Self::check_playablity(board, turn) {
                        board.board[0][0] = Cell::Indicator(Turn::White);
                        (Turn::White, None)
                    } else {
                        (Turn::Black, None)
                    }
                }
                Turn::White => {
                    board.board[r][c] = Cell::White;
                    Board::validate_cells(&mut board.board);

                    let res = Self::check_end_game(board);
                    if res.0 {
                        return (res.1, Some(Move::Win(res.2)));
                    }

                    if Self::check_automatic_win(board, Cell::Black) {
                        return (Turn::White, Some(Move::Dominate));
                    }

                    if Self::check_playablity(board, turn) {
                        board.board[0][0] = Cell::Indicator(Turn::Black);
                        (Turn::Black, None)
                    } else {
                        (Turn::White, None)
                    }
                }
                // Automatically set `turn` to `Neither`
                // when there are no empty spaces.
                //
                // Or when each player passes their turn consecutively.
                _ => (Turn::Neither, None),
            },
            Move::Pass => match turn {
                Turn::Black => {
                    if Self::check_playablity(board, turn) {
                        board.board[0][0] = Cell::Indicator(Turn::White);
                        (Turn::White, Some(Move::Pass))
                    } else {
                        println!("Can't pass.");
                        sleep(Duration::new(1, 0));
                        (Turn::Black, Some(Move::Pass))
                    }
                }
                Turn::White => {
                    if Self::check_playablity(board, turn) {
                        board.board[0][0] = Cell::Indicator(Turn::Black);
                        (Turn::Black, Some(Move::Pass))
                    } else {
                        println!("Can't pass.");
                        sleep(Duration::new(1, 0));
                        (Turn::White, Some(Move::Pass))
                    }
                }
                _ => (Turn::Neither, None),
            },
            Move::Resign => match turn {
                Turn::Black => (Turn::Black, Some(Move::Resign)),
                Turn::White => (Turn::White, Some(Move::Resign)),
                _ => (Turn::Neither, None),
            },
            Move::Undo => Self::handle_move(board, turn),
            _ => (Turn::Neither, None),
        }
    }
}

trait InputHandler {
    fn get_move(board: &mut Board, turn: Turn) -> Move;
    fn get_col_input() -> usize;
    fn get_row_input() -> usize;
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
        match Self::get_col_input() {
            0 => Move::Pass,
            9 => Move::Resign,
            col => match Self::get_row_input() {
                42 => Move::Undo,
                row => {
                    if Self::is_valid_move(board, turn, row, col) {
                        Move::Play(row, col)
                    } else {
                        Self::get_move(board, turn)
                    }
                }
            },
        }
    }

    fn get_col_input() -> usize {
        print!("Which column? (Enter `p` to pass or `r` to resign.) ");
        io::stdout().flush().unwrap();

        let mut col = String::new();
        io::stdin()
            .read_line(&mut col)
            .expect("failed to read line");

        let col = col.trim();
        match col {
            c if c.eq_ignore_ascii_case("p") => option!(pass),
            c if c.eq_ignore_ascii_case("r") => option!(resign),
            c => match c.parse() {
                Ok(num) if (1..=8).contains(&num) => num,
                _ => Self::get_col_input(),
            },
        }
    }

    fn get_row_input() -> usize {
        print!("Which row? (Enter `u` to undo the column input.) ");
        io::stdout().flush().unwrap();

        let mut row = String::new();
        io::stdin()
            .read_line(&mut row)
            .expect("failed to read line");

        let row = row.trim();
        match row {
            r if r.eq_ignore_ascii_case("u") => option!(undo),
            r => match r.parse() {
                Ok(num) if (1..=8).contains(&num) => num,
                _ => Self::get_row_input(),
            },
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
    fn check_end_game(board: &Board) -> (bool, Turn, u8);
    fn check_automatic_win(board: &Board, opponent_color: Cell) -> bool;
}

impl PlayabilityChecker for Move {
    fn check_playablity(board: &Board, turn: Turn) -> bool {
        match turn {
            Turn::Black => Direction::iter().any(|d| check!(board, Cell::White, Cell::Black, d)),
            Turn::White => Direction::iter().any(|d| check!(board, Cell::Black, Cell::White, d)),
            _ => false,
        }
    }

    fn check_end_game(board: &Board) -> (bool, Turn, u8) {
        if !Self::check_playablity(board, Turn::Black)
            && !Self::check_playablity(board, Turn::White)
        {
            check!(board)
        }
        (false, Turn::Neither, 0)
    }

    fn check_automatic_win(board: &Board, opponent_color: Cell) -> bool {
        check!(board, opponent_color)
    }
}
