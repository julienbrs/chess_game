mod board;
mod chess_move;
mod piece;
use crate::chess_move::{ChessMove, Position};
use board::{BoardFactory, BoardPosition, make_move, print_board};
use chess_move::parse_move;
use std::io::{self, Write};

fn main() {
    let mut board = BoardFactory::create(BoardPosition::Standard);

    loop {
        print_board(&board);
        print!("Enter move (e.g. e2e4) or 'quit' to exit: ");
        // Ensure the prompt is displayed before reading input
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_lowercase();

        if input == "quit" {
            println!("Thanks for playing!");
            break;
        }

        match parse_move(&input) {
            Ok(chess_move) => match make_move(&mut board, &chess_move) {
                Ok(_) => println!("Move completed: {}", chess_move),
                Err(e) => println!("{}", e),
            },
            Err(e) => println!("Error parsing move: {}", e),
        }
    }
}
