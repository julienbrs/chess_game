mod board;
mod piece;
mod chess_move;
use board::{BoardFactory, BoardPosition, print_board};

fn main() {
    let board = BoardFactory::create(BoardPosition::Standard);

    print_board(&board);
}
