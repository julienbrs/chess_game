use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Utiliser des symboles d'échecs Unicode
        let symbol = match self.piece_type {
            PieceType::King => {
                if self.color == PieceColor::White {
                    "♔ "
                } else {
                    "♚ "
                }
            }
            PieceType::Queen => {
                if self.color == PieceColor::White {
                    "♕ "
                } else {
                    "♛ "
                }
            }
            PieceType::Rook => {
                if self.color == PieceColor::White {
                    "♖ "
                } else {
                    "♜ "
                }
            }
            PieceType::Bishop => {
                if self.color == PieceColor::White {
                    "♗ "
                } else {
                    "♝ "
                }
            }
            PieceType::Knight => {
                if self.color == PieceColor::White {
                    "♘ "
                } else {
                    "♞ "
                }
            }
            PieceType::Pawn => {
                if self.color == PieceColor::White {
                    "♙ "
                } else {
                    "♟ "
                }
            }
        };
        write!(f, "{}", symbol)
    }
}

impl Piece {
    pub fn new(piece_type: PieceType, color: PieceColor) -> Piece {
        Piece { piece_type, color }
    }

    pub fn move_piece(&mut self) {
        return;
    }
}
