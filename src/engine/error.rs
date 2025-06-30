use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum SquareError {
    #[error("input must be 2 characters like 'e2'")]
    InvalidLength,

    #[error("invalid column letter (must be 'a' to 'h')")]
    InvalidColumn,

    #[error("invalid row number (must be '1' to '8')")]
    InvalidRow,

    #[error("position is out of bounds (should be 0..=7)")]
    OutOfBounds,
}
