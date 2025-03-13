use crate::piece::{Piece, PieceColor, PieceType};

pub type BoardGame = [[Option<Piece>; 8]; 8];

pub fn print_board(board: &BoardGame) {
    for (row_idx, row) in board.iter().enumerate() {
        print!("{}| ", 8 - row_idx);
        for cell in row {
            match cell {
                Some(piece) => print!("{}", piece),
                None => print!("· "),
            }
        }
        println!("");
    }
    println!("  ---------------");
    println!("  a b c d e f g h");
}

#[derive(Clone, Copy)]
pub enum BoardPosition {
    Standard,
    Empty,
}

pub struct BoardFactory;

impl BoardFactory {
    pub fn create(position: BoardPosition) -> BoardGame {
        match position {
            BoardPosition::Empty => [[None; 8]; 8],
            BoardPosition::Standard => Self::create_standard_position(),
        }
    }

    fn create_standard_position() -> BoardGame {
        let mut board_game = [[None; 8]; 8];

        board_game[0][0] = Some(Piece::new(PieceType::Rook, PieceColor::White));
        board_game[0][1] = Some(Piece::new(PieceType::Knight, PieceColor::White));
        board_game[0][2] = Some(Piece::new(PieceType::Bishop, PieceColor::White));
        board_game[0][3] = Some(Piece::new(PieceType::Queen, PieceColor::White));
        board_game[0][4] = Some(Piece::new(PieceType::King, PieceColor::White));
        board_game[0][5] = Some(Piece::new(PieceType::Bishop, PieceColor::White));
        board_game[0][6] = Some(Piece::new(PieceType::Knight, PieceColor::White));
        board_game[0][7] = Some(Piece::new(PieceType::Rook, PieceColor::White));

        for i in 0..8 {
            board_game[1][i] = Some(Piece::new(PieceType::Pawn, PieceColor::White));
        }

        for i in 0..8 {
            board_game[6][i] = Some(Piece::new(PieceType::Pawn, PieceColor::Black));
        }

        board_game[7][0] = Some(Piece::new(PieceType::Rook, PieceColor::Black));
        board_game[7][1] = Some(Piece::new(PieceType::Knight, PieceColor::Black));
        board_game[7][2] = Some(Piece::new(PieceType::Bishop, PieceColor::Black));
        board_game[7][3] = Some(Piece::new(PieceType::Queen, PieceColor::Black));
        board_game[7][4] = Some(Piece::new(PieceType::King, PieceColor::Black));
        board_game[7][5] = Some(Piece::new(PieceType::Bishop, PieceColor::Black));
        board_game[7][6] = Some(Piece::new(PieceType::Knight, PieceColor::Black));
        board_game[7][7] = Some(Piece::new(PieceType::Rook, PieceColor::Black));

        board_game
    }
}
