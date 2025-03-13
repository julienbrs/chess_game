use crate::{
    board::BoardGame,
    piece::{PieceColor, PieceType},
};

#[derive(PartialEq)]
pub struct Position {
    row: usize,
    column: usize,
}

pub struct ChessMove {
    from: Position,
    to: Position,
}

pub fn is_valid_move(move_: ChessMove, piece_type: &PieceType, board: &BoardGame) -> bool {
    fn within_bounds(position: &Position) -> bool {
        position.row >= 0 && position.row <= 7 && position.column >= 0 && position.column <= 7
    }

    // Check if in boundaries
    if !within_bounds(&move_.from) || !within_bounds(&move_.to) {
        return false;
    }

    // Verify there is a piece to move
    let piece = match board[move_.from.row][move_.from.column] {
        Some(piece) => piece,
        None => return false,
    };

    // Verify if the piece is moving
    if move_.to == move_.from {
        return false;
    }

    // Verify if there there is an ally piece at destination
    let capture: bool = match board[move_.to.row][move_.to.column] {
        Some(destination_piece) => {
            if piece.color == destination_piece.color {
                return false;
            }
            true
        }
        None => false,
    };

    match piece.piece_type {
        PieceType::Pawn => {
            if capture {
                // TODO: capture "En passant"
                if (piece.color == PieceColor::White)
                    && !((move_.to.column == move_.from.column - 1
                        || move_.to.column == move_.from.column + 1)
                        && move_.to.row == move_.from.row - 1)
                {
                    return false;
                }
                if (piece.color == PieceColor::Black)
                    && !((move_.to.column == move_.from.column - 1
                        || move_.to.column == move_.from.column + 1)
                        && move_.to.row == move_.from.row + 1)
                {
                    return false;
                }
            }

            if piece.color == PieceColor::White {
                let valid_single_move =
                    move_.to.row == move_.from.row - 1 && move_.to.column == move_.from.column;

                let empty_blocking_cell = match board[move_.from.row - 1][move_.from.column] {
                    Some(_) => false,
                    None => true,
                };
                let valid_double_move = move_.to.row == move_.from.row - 2
                    && move_.to.column == move_.from.column
                    && empty_blocking_cell;

                return valid_single_move || (valid_double_move && move_.from.row == 6);
            }

            if piece.color == PieceColor::Black {
                let valid_single_move =
                    move_.to.row == move_.from.row + 1 && move_.to.column == move_.from.column;

                let empty_blocking_cell = match board[move_.from.row + 1][move_.from.column] {
                    Some(_) => false,
                    None => true,
                };
                let valid_double_move = move_.to.row == move_.from.row + 2
                    && move_.to.column == move_.from.column
                    && empty_blocking_cell;

                return valid_single_move || (valid_double_move && move_.from.row == 1);
            }
        }
        PieceType::Rook => {
            let dx = (move_.to.column as i32 - move_.from.column as i32).signum();
            let dy = (move_.to.row as i32 - move_.from.row as i32).signum();

            // Check if move is horizontal or vertical
            if dx != 0 && dy != 0 {
                return false;
            }

            let mut current = Position {
                column: move_.from.column,
                row: move_.from.row,
            };

            while current != move_.to {
                current.column = (current.column as i32 + dx) as usize;
                current.row = (current.row as i32 + dy) as usize;

                match board[current.row][current.column] {
                    Some(_) => return false,
                    None => (),
                };
            }
            return true;
        }
        PieceType::Knight => {
            let row_diff = (move_.to.row as i32 - move_.from.row as i32).abs();
            let col_diff = (move_.to.column as i32 - move_.from.column as i32).abs();

            if !((row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2)) {
                return false;
            }
            return true;
        }
        PieceType::Bishop => {
            let row_diff = (move_.to.row as i32 - move_.from.row as i32).abs();
            let col_diff = (move_.to.column as i32 - move_.from.column as i32).abs();

            // Moving in diagonal
            if !(row_diff == col_diff) {
                return false;
            }

            // Check for blocking pieces
            let dx = (move_.to.column as i32 - move_.from.column as i32).signum();
            let dy = (move_.to.row as i32 - move_.from.row as i32).signum();
            let mut current = move_.from;

            while current != move_.to {
                current.column = (current.column as i32 + dx) as usize;
                current.row = (current.row as i32 + dy) as usize;

                match board[current.row][current.column] {
                    Some(_) => return false,
                    None => (),
                };
            }

            return true;
        }
        PieceType::King => {
            // TODO: castle move
            let row_diff = (move_.to.row as i32 - move_.from.row as i32).abs();
            let col_diff = (move_.to.column as i32 - move_.from.column as i32).abs();

            if !(row_diff <= 1 && col_diff <= 1) {
                return false;
            }
            return true;
        }
        PieceType::Queen => {
            let dx = (move_.to.column as i32 - move_.from.column as i32).signum();
            let dy = (move_.to.row as i32 - move_.from.row as i32).signum();

            let is_straight_move = (dx == 0 && dy != 0) || (dx != 0 && dy == 0);

            let row_diff = (move_.to.row as i32 - move_.from.row as i32).abs();
            let col_diff = (move_.to.column as i32 - move_.from.column as i32).abs();

            let is_diagonal_move = row_diff == col_diff;

            // Check valid movement
            if !(is_diagonal_move || is_straight_move) {
                return false;
            }

            let mut current = move_.from;

            while current != move_.to {
                current.column = (current.column as i32 + dx) as usize;
                current.row = (current.row as i32 + dy) as usize;

                match board[current.row][current.column] {
                    Some(_) => return false,
                    None => (),
                };
            }

            return true;
        }
    }
    true
}
