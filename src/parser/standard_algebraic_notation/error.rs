use thiserror::Error;

/// Describes an error that occured during the parsing of a string containing a turn in `SAN`.
#[derive(Error, Debug)]
pub enum SanParserError {
    /// Occurs when the given input data is not a chess move in `SAN`.
    #[error("The given input does not contain a valid SAN move: {0}")]
    InvalidData(String),
    /// Occurs when the given `SAN` turn is not an illegal move in the given position.
    #[error("The given move is illegal in the current position: {0}")]
    InvalidMove(String),
}
