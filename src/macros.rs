// This `flip` macro just flips discs when a disc is placed
// at a valid cell.
#[macro_export]
macro_rules! flip {
    ( $board:expr, $you:expr, $opponent:expr, $dir:expr, $row:expr, $col:expr ) => {{
        use Direction::*;
        match $dir {
            Up => {
                if $board.board[$row - 1][$col] == $opponent {
                    let mut count = 2;
                    while $row - count > 0 {
                        if $board.board[$row - count][$col] == $opponent {
                            count += 1;
                            continue;
                        } else if $board.board[$row - count][$col] == $you {
                            (1..count).for_each(|r| {
                                $board.board[$row - r][$col] = $you;
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
            Down => {
                if $board.board[$row + 1][$col] == $opponent {
                    let mut count = 2;
                    while $row + count != 9 {
                        if $board.board[$row + count][$col] == $opponent {
                            count += 1;
                            continue;
                        } else if $board.board[$row + count][$col] == $you {
                            (1..count).for_each(|r| {
                                $board.board[$row + r][$col] = $you;
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
            Left => {
                if $board.board[$row][$col - 1] == $opponent {
                    let mut count = 2;
                    while $col - count != 0 {
                        if $board.board[$row][$col - count] == $opponent {
                            count += 1;
                            continue;
                        } else if $board.board[$row][$col - count] == $you {
                            (1..count).for_each(|c| {
                                $board.board[$row][$col - c] = $you;
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
            Right => {
                if $board.board[$row][$col + 1] == $opponent {
                    let mut count = 2;
                    while $col + count != 9 {
                        if $board.board[$row][$col + count] == $opponent {
                            count += 1;
                            continue;
                        } else if $board.board[$row][$col + count] == $you {
                            (1..count).for_each(|c| {
                                $board.board[$row][$col + c] = $you;
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
            UpLeft => {
                if $board.board[$row - 1][$col - 1] == $opponent {
                    let mut count = 2;
                    while $row - count != 0 && $col - count != 0 {
                        if $board.board[$row - count][$col - count] == $opponent {
                            count += 1;
                            continue;
                        } else if $board.board[$row - count][$col - count] == $you {
                            (1..count).for_each(|c| {
                                $board.board[$row - c][$col - c] = $you;
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
            UpRight => {
                if $board.board[$row - 1][$col + 1] == $opponent {
                    let mut count = 2;
                    while $row - count != 0 && $col + count != 9 {
                        if $board.board[$row - count][$col + count] == $opponent {
                            count += 1;
                            continue;
                        } else if $board.board[$row - count][$col + count] == $you {
                            (1..count).for_each(|c| {
                                $board.board[$row - c][$col + c] = $you;
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
            DownLeft => {
                if $board.board[$row + 1][$col - 1] == $opponent {
                    let mut count = 2;
                    while $row + count != 9 && $col - count != 0 {
                        if $board.board[$row + count][$col - count] == $opponent {
                            count += 1;
                            continue;
                        } else if $board.board[$row + count][$col - count] == $you {
                            (1..count).for_each(|c| {
                                $board.board[$row + c][$col - c] = $you;
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
            DownRight => {
                if $board.board[$row + 1][$col + 1] == $opponent {
                    let mut count = 2;
                    while $row + count != 9 && $col + count != 9 {
                        if $board.board[$row + count][$col + count] == $opponent {
                            count += 1;
                            continue;
                        } else if $board.board[$row + count][$col + count] == $you {
                            (1..count).for_each(|c| {
                                $board.board[$row + c][$col + c] = $you;
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
        }
    }};
}

// This macro handles several cases where:
//   1. If cells are available for whichever player to play.
//   2. Calculate scores when a game ends (automatically).
#[macro_export]
macro_rules! check {
    ( $board:expr, $you:expr, $opponent:expr, $dir:expr ) => {{
        use crate::board::SIZE;
        use Direction::*;
        match $dir {
            Up => {
                for row in 3..SIZE {
                    for col in 1..SIZE {
                        if $board.board[row][col] == Cell::Okay {
                            if $board.board[row - 1][col] == $opponent {
                                let mut count = 2;
                                while row - count != 0 {
                                    if $board.board[row - count][col] == $opponent {
                                        count += 1;
                                        continue;
                                    } else if $board.board[row - count][col] == $you {
                                        return true;
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                false
            }
            Down => {
                for row in 1..=6 {
                    for col in 1..SIZE {
                        if $board.board[row][col] == Cell::Okay {
                            if $board.board[row + 1][col] == $opponent {
                                let mut count = 2;
                                while row + count != 9 {
                                    if $board.board[row + count][col] == $opponent {
                                        count += 1;
                                        continue;
                                    } else if $board.board[row + count][col] == $you {
                                        return true;
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                false
            }
            Left => {
                for row in 1..SIZE {
                    for col in 3..SIZE {
                        if $board.board[row][col] == Cell::Okay {
                            if $board.board[row][col - 1] == $opponent {
                                let mut count = 2;
                                while col - count != 0 {
                                    if $board.board[row][col - count] == $opponent {
                                        count += 1;
                                        continue;
                                    } else if $board.board[row][col - count] == $you {
                                        return true;
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                false
            }
            Right => {
                for row in 1..SIZE {
                    for col in 1..=6 {
                        if $board.board[row][col] == Cell::Okay {
                            if $board.board[row][col + 1] == $opponent {
                                let mut count = 2;
                                while col + count != 9 {
                                    if $board.board[row][col + count] == $opponent {
                                        count += 1;
                                        continue;
                                    } else if $board.board[row][col + count] == $you {
                                        return true;
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                false
            }
            UpLeft => {
                for row in 3..SIZE {
                    for col in 3..SIZE {
                        if $board.board[row][col] == Cell::Okay {
                            if $board.board[row - 1][col - 1] == $opponent {
                                let mut count = 2;
                                while row - count != 0 && col - count != 0 {
                                    if $board.board[row - count][col - count] == $opponent {
                                        count += 1;
                                        continue;
                                    } else if $board.board[row - count][col - count] == $you {
                                        return true;
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                false
            }
            UpRight => {
                for row in 3..SIZE {
                    for col in 1..=6 {
                        if $board.board[row][col] == Cell::Okay {
                            if $board.board[row - 1][col + 1] == $opponent {
                                let mut count = 2;
                                while row - count != 0 && col + count != 9 {
                                    if $board.board[row - count][col + count] == $opponent {
                                        count += 1;
                                        continue;
                                    } else if $board.board[row - count][col + count] == $you {
                                        return true;
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                false
            }
            DownLeft => {
                for row in 1..=6 {
                    for col in 3..SIZE {
                        if $board.board[row][col] == Cell::Okay {
                            if $board.board[row + 1][col - 1] == $opponent {
                                let mut count = 2;
                                while row + count != 9 && col - count != 0 {
                                    if $board.board[row + count][col - count] == $opponent {
                                        count += 1;
                                        continue;
                                    } else if $board.board[row + count][col - count] == $you {
                                        return true;
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                false
            }
            DownRight => {
                for row in 1..=6 {
                    for col in 1..=6 {
                        if $board.board[row][col] == Cell::Okay {
                            if $board.board[row + 1][col + 1] == $opponent {
                                let mut count = 2;
                                while row + count != 9 && col + count != 9 {
                                    if $board.board[row + count][col + count] == $opponent {
                                        count += 1;
                                        continue;
                                    } else if $board.board[row + count][col + count] == $you {
                                        return true;
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                false
            }
        }
    }};

    ( $board:expr, $opponent_color:expr ) => {{
        use crate::board::SIZE;
        for row in 1..SIZE {
            for col in 1..SIZE {
                if $board.board[row][col] == $opponent_color {
                    return false;
                }
            }
        }
        true
    }};

    ( $board:expr ) => {{
        use crate::board::SIZE;

        let mut black_count = 0;
        let mut white_count = 0;
        for row in 1..SIZE {
            for col in 1..SIZE {
                if $board.board[row][col] == Cell::Black {
                    black_count += 1;
                } else if $board.board[row][col] == Cell::White {
                    white_count += 1;
                }
            }
        }

        if black_count > white_count {
            return (true, Turn::Black, black_count, white_count);
        } else if black_count < white_count {
            return (true, Turn::White, black_count, white_count);
        } else {
            return (true, Turn::Neither, black_count, white_count);
        }
    }};
}
