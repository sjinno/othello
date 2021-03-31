use crate::board;

enum Turn {
    Black,
    White,
}

enum Move {
    Play,
    Pass,
}

enum State {
    Game,
    Over,
}

pub fn start_game() {
    let board = board::Board::new();
    board.draw();
}
