use crate::{
    board::BoardGame,
    piece::{PieceColor, PieceType},
};
use std::fmt;

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

#[derive(PartialEq, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

fn parse_position(input: &str) -> Result<Position, &'static str> {
    let column = match input.chars().nth(0) {
        Some(c) => match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return Err("Invalid column"),
        },
        None => return Err("Input too short"),
    };

    let row = match input.chars().nth(1) {
        Some(c) => match c.to_digit(10) {
            Some(d) if d >= 1 && d <= 8 => 8 - d as usize, // Convert to 0-7 index, inverted
            _ => return Err("Invalid row"),
        },
        None => return Err("Input too short"),
    };

    Ok(Position { row, column })
}

pub fn parse_move(input: &str) -> Result<ChessMove, &'static str> {
    if input.len() != 4 {
        return Err("Invalid move format. Please use format like 'e2e4'");
    }

    let from = parse_position(&input[0..2])?;
    let to = parse_position(&input[2..4])?;

    Ok(ChessMove { from, to })
}

#[derive(Clone)]
pub struct ChessMove {
    pub from: Position,
    pub to: Position,
}

impl fmt::Display for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert column number (0-7) back to chess notation (a-h)
        let from_col = (self.from.column as u8 + b'a') as char;
        let to_col = (self.to.column as u8 + b'a') as char;

        // Convert row number (0-7) back to chess notation (1-8)
        let from_row = 8 - self.from.row;
        let to_row = 8 - self.to.row;

        write!(f, "{}{}{}{}", from_col, from_row, to_col, to_row)
    }
}

pub fn is_valid_move(board: &BoardGame, move_: &ChessMove) -> Result<(), MoveError> {
    fn within_bounds(position: &Position) -> bool {
        position.row <= 7 && position.column <= 7
    }

    // Check boundaries
    if !within_bounds(&move_.from) || !within_bounds(&move_.to) {
        return Err(MoveError::OutOfBounds);
    }

    // Verify piece exists
    let piece = match board[move_.from.row][move_.from.column] {
        Some(piece) => piece,
        None => return Err(MoveError::NoPieceAtSource),
    };

    // Verify movement
    if move_.to == move_.from {
        return Err(MoveError::SamePosition);
    }

    // Check for capture
    let is_capture = match board[move_.to.row][move_.to.column] {
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
            if is_capture {
                // Capture moves
                match piece.color {
                    PieceColor::White => {
                        if !(move_.to.row == move_.from.row - 1
                            && (move_.to.column == move_.from.column - 1
                                || move_.to.column == move_.from.column + 1))
                        {
                            return Err(MoveError::InvalidPawnCapture);
                        }
                    }
                    PieceColor::Black => {
                        if !(move_.to.row == move_.from.row + 1
                            && (move_.to.column == move_.from.column - 1
                                || move_.to.column == move_.from.column + 1))
                        {
                            return Err(MoveError::InvalidPawnCapture);
                        }
                    }
                }
            } else {
                // Normal moves
                match piece.color {
                    PieceColor::White => {
                        let valid_single_move = move_.to.row == move_.from.row - 1
                            && move_.to.column == move_.from.column;

                        let empty_blocking_cell =
                            board[move_.from.row - 1][move_.from.column].is_none();
                        let valid_double_move = move_.to.row == move_.from.row - 2
                            && move_.to.column == move_.from.column
                            && move_.from.row == 6
                            && empty_blocking_cell;

                        if !valid_single_move && !valid_double_move {
                            return Err(MoveError::InvalidPawnMove);
                        }
                    }
                    PieceColor::Black => {
                        let valid_single_move = move_.to.row == move_.from.row + 1
                            && move_.to.column == move_.from.column;

                        let empty_blocking_cell =
                            board[move_.from.row + 1][move_.from.column].is_none();
                        let valid_double_move = move_.to.row == move_.from.row + 2
                            && move_.to.column == move_.from.column
                            && move_.from.row == 1
                            && empty_blocking_cell;

                        if !valid_single_move && !valid_double_move {
                            return Err(MoveError::InvalidPawnMove);
                        }
                    }
                }
            }
        }
        PieceType::Rook => {
            let dx = (move_.to.column as i32 - move_.from.column as i32).signum();
            let dy = (move_.to.row as i32 - move_.from.row as i32).signum();

            if dx != 0 && dy != 0 {
                return Err(MoveError::InvalidRookMove);
            }

            let mut current = Position {
                column: move_.from.column,
                row: move_.from.row,
            };

            while current != move_.to {
                current.column = (current.column as i32 + dx) as usize;
                current.row = (current.row as i32 + dy) as usize;

                if let Some(_) = board[current.row][current.column] {
                    return Err(MoveError::PieceBlocking);
                }
            }
        }
        PieceType::Knight => {
            let row_diff = (move_.to.row as i32 - move_.from.row as i32).abs();
            let col_diff = (move_.to.column as i32 - move_.from.column as i32).abs();

            if !((row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2)) {
                return Err(MoveError::InvalidKnightMove);
            }
        }
        PieceType::Bishop => {
            let row_diff = (move_.to.row as i32 - move_.from.row as i32).abs();
            let col_diff = (move_.to.column as i32 - move_.from.column as i32).abs();

            if row_diff != col_diff {
                return Err(MoveError::InvalidBishopMove);
            }

            let dx = (move_.to.column as i32 - move_.from.column as i32).signum();
            let dy = (move_.to.row as i32 - move_.from.row as i32).signum();
            let mut current = Position {
                column: move_.from.column,
                row: move_.from.row,
            };

            while current != move_.to {
                current.column = (current.column as i32 + dx) as usize;
                current.row = (current.row as i32 + dy) as usize;

                if let Some(_) = board[current.row][current.column] {
                    return Err(MoveError::PieceBlocking);
                }
            }
        }
        PieceType::King => {
            let row_diff = (move_.to.row as i32 - move_.from.row as i32).abs();
            let col_diff = (move_.to.column as i32 - move_.from.column as i32).abs();

            if row_diff > 1 || col_diff > 1 {
                return Err(MoveError::InvalidKingMove);
            }
        }
        PieceType::Queen => {
            let dx = (move_.to.column as i32 - move_.from.column as i32).signum();
            let dy = (move_.to.row as i32 - move_.from.row as i32).signum();
            let row_diff = (move_.to.row as i32 - move_.from.row as i32).abs();
            let col_diff = (move_.to.column as i32 - move_.from.column as i32).abs();

            let is_straight = (dx == 0 && dy != 0) || (dx != 0 && dy == 0);
            let is_diagonal = row_diff == col_diff;

            if !is_straight && !is_diagonal {
                return Err(MoveError::InvalidQueenMove);
            }

            let mut current = Position {
                column: move_.from.column,
                row: move_.from.row,
            };

            while current != move_.to {
                current.column = (current.column as i32 + dx) as usize;
                current.row = (current.row as i32 + dy) as usize;

                if let Some(_) = board[current.row][current.column] {
                    return Err(MoveError::PieceBlocking);
                }
            }
        }
    }

    Ok(())
}
