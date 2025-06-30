use crate::engine::error::SquareError;
use crate::{
    engine::board::BoardGame,
    engine::piece::{PieceColor, PieceType},
};
use std::fmt;
use std::io::IntoInnerError;

#[derive(Debug)]
pub enum MoveError {
    OutOfBounds,
    NoPieceAtSource,
    SamePosition,
    CaptureOwnPiece,
    InvalidPawnCapture,
    InvalidPawnMove,
    InvalidRookMove,
    InvalidKnightMove,
    InvalidBishopMove,
    InvalidKingMove,
    InvalidQueenMove,
    PieceBlocking,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Square(u8); // range 0–63

impl TryFrom<(u8, u8)> for Square {
    type Error = SquareError;
    fn try_from((row, col): (u8, u8)) -> Result<Self, Self::Error> {
        if row < 8 && col < 8 {
            Ok(Square(row * 8 + col))
        } else {
            Err(SquareError::OutOfBounds)
        }
    }
}

impl Square {
    pub fn row(self) -> usize {
        (self.0 / 8) as usize
    }

    pub fn col(self) -> usize {
        (self.0 % 8) as usize
    }

    pub fn to_tuple(self) -> (usize, usize) {
        (self.row(), self.col())
    }

    pub fn offset(self, d_row: i32, d_col: i32) -> Option<Square> {
        let row = self.row() as i32 + d_row;
        let col = self.col() as i32 + d_col;
        if row >= 0 && row < 8 && col >= 0 && col < 8 {
            Some(Square((row * 8 + col) as u8))
        } else {
            None
        }
    }
}

fn parse_position(input: &str) -> Result<Square, SquareError> {
    let bytes = input.as_bytes();

    if bytes.len() != 2 {
        return Err(SquareError::InvalidLength);
    }
    let col = match bytes[0] {
        b'a'..b'h' => bytes[0] - b'a',
        _ => return Err(SquareError::InvalidColumn),
    };

    let row = match bytes[1] {
        b'1'..b'8' => bytes[1] - b'1',
        _ => return Err(SquareError::InvalidRow),
    };

    Square::try_from((row, col))
}

pub fn parse_move(input: &str) -> Result<ChessMove, SquareError> {
    if input.len() != 4 {
        return Err(SquareError::InvalidLength);
    }

    let from = parse_position(&input[0..2])?;
    let to = parse_position(&input[2..4])?;

    Ok(ChessMove { from, to })
}

#[derive(Clone)]
pub struct ChessMove {
    pub from: Square,
    pub to: Square,
}

impl fmt::Display for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert column number (0-7) back to chess notation (a-h)
        let from_col = (self.from.col() as u8 + b'a') as char;
        let to_col = (self.to.col() as u8 + b'a') as char;

        // Convert row number (0-7) back to chess notation (1-8)
        let from_row = 8 - self.from.row();
        let to_row = 8 - self.to.row();

        write!(f, "{}{}{}{}", from_col, from_row, to_col, to_row)
    }
}

pub fn is_valid_move(board: &BoardGame, move_: &ChessMove) -> Result<(), MoveError> {
    // Verify piece exists
    let piece = match board[move_.from.row()][move_.from.col()] {
        Some(piece) => piece,
        None => return Err(MoveError::NoPieceAtSource),
    };

    // Verify movement
    if move_.to == move_.from {
        return Err(MoveError::SamePosition);
    }

    // Check for capture
    let is_capture = match board[move_.to.row()][move_.to.col()] {
        Some(destination_piece) => {
            if piece.color == destination_piece.color {
                return Err(MoveError::CaptureOwnPiece);
            }
            true
        }
        None => false,
    };

    match piece.piece_type {
        PieceType::Pawn => {
            let from_row = move_.from.row();
            let from_col = move_.from.col();
            let to_row = move_.to.row();
            let to_col = move_.to.col();

            let row_diff = to_row as i32 - from_row as i32;
            let col_diff = to_col as i32 - from_col as i32;

            let direction = match piece.color {
                PieceColor::White => -1,
                PieceColor::Black => 1,
            };

            let start_row = match piece.color {
                PieceColor::White => 6,
                PieceColor::Black => 1,
            };

            if is_capture {
                // Diagonal capture: must move one square diagonally forward
                if row_diff != direction || col_diff.abs() != 1 {
                    return Err(MoveError::InvalidPawnCapture);
                }
            } else {
                // Forward move: 1 or 2 squares straight ahead
                if col_diff != 0 {
                    return Err(MoveError::InvalidPawnMove);
                }

                if row_diff == direction {
                    // Single step
                    if board[(from_row as i32 + direction) as usize][from_col].is_some() {
                        return Err(MoveError::PieceBlocking);
                    }
                } else if row_diff == 2 * direction && from_row == start_row {
                    let intermediate_row = (from_row as i32 + direction) as usize;
                    let target_row = (from_row as i32 + 2 * direction) as usize;

                    if board[intermediate_row][from_col].is_some()
                        || board[target_row][from_col].is_some()
                    {
                        return Err(MoveError::PieceBlocking);
                    }
                } else {
                    return Err(MoveError::InvalidPawnMove);
                }
            }
        }
        PieceType::Rook => {
            let dx = (move_.to.col() as i32 - move_.from.col() as i32).signum();
            let dy = (move_.to.row() as i32 - move_.from.row() as i32).signum();

            if dx != 0 && dy != 0 {
                return Err(MoveError::InvalidRookMove);
            }

            let mut current = move_.from;

            while let Some(next) = current.offset(dy, dx) {
                if next == move_.to {
                    break;
                }
                let (row, col) = next.to_tuple();
                if board[row][col].is_some() {
                    return Err(MoveError::PieceBlocking);
                }
            }
        }
        PieceType::Knight => {
            let row_diff = (move_.to.row() as i32 - move_.from.row() as i32).abs();
            let col_diff = (move_.to.col() as i32 - move_.from.col() as i32).abs();

            if !((row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2)) {
                return Err(MoveError::InvalidKnightMove);
            }
        }
        PieceType::Bishop => {
            let row_diff = (move_.to.row() as i32 - move_.from.row() as i32).abs();
            let col_diff = (move_.to.col() as i32 - move_.from.col() as i32).abs();

            if row_diff != col_diff {
                return Err(MoveError::InvalidBishopMove);
            }

            let dx = (move_.to.col() as i32 - move_.from.col() as i32).signum();
            let dy = (move_.to.row() as i32 - move_.from.row() as i32).signum();
            let mut current = move_.from;

            while let Some(next) = current.offset(dy, dx) {
                if next == move_.to {
                    break;
                }

                let (row, col) = next.to_tuple();
                if board[row][col].is_some() {
                    return Err(MoveError::PieceBlocking);
                }
            }
        }
        PieceType::King => {
            let row_diff = (move_.to.row() as i32 - move_.from.row() as i32).abs();
            let col_diff = (move_.to.col() as i32 - move_.from.col() as i32).abs();

            if row_diff > 1 || col_diff > 1 {
                return Err(MoveError::InvalidKingMove);
            }
        }
        PieceType::Queen => {
            let dx = (move_.to.col() as i32 - move_.from.col() as i32).signum();
            let dy = (move_.to.row() as i32 - move_.from.row() as i32).signum();
            let row_diff = (move_.to.row() as i32 - move_.from.row() as i32).abs();
            let col_diff = (move_.to.col() as i32 - move_.from.col() as i32).abs();

            let is_straight = (dx == 0 && dy != 0) || (dx != 0 && dy == 0);
            let is_diagonal = row_diff == col_diff;

            if !is_straight && !is_diagonal {
                return Err(MoveError::InvalidQueenMove);
            }

            let mut current = move_.from;

            while let Some(next) = current.offset(dy, dx) {
                if next == move_.to {
                    break;
                }

                let (r, c) = next.to_tuple();
                if board[r][c].is_some() {
                    return Err(MoveError::PieceBlocking);
                }

                current = next;
            }
        }
    }

    Ok(())
}
