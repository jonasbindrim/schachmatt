use thiserror::Error;

#[derive(Error, Debug)]
pub enum SanParserError {
    #[error("The given input does not contain a valid SAN move: {0}")]
    InvalidData(String),
    #[error("The given move is illegal in the current position: {0}")]
    InvalidMove(String),
}
