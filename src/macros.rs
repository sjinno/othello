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
                    println!("I'm here :)");
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
                    while $row - count != 0 || $col - count != 0 {
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
                    println!("I'm here :)");
                    let mut count = 2;
                    while $row - count != 0 || $col + count != 9 {
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
                    while $row + count != 9 || $col - count != 0 {
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
                    while $row + count != 9 || $col + count != 9 {
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
