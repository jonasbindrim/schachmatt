use thiserror::Error;

#[derive(Error, Debug)]
pub enum PgnParserError {
    #[error("Input contains invalid metadata: {0}")]
    InvalidMetadataContent(String),
    #[error("Invalid turn data content: {0}")]
    InvalidTurnData(String),
    #[error("Invalid game result indicator: {0}")]
    InvalidGameResult(String),
    #[error("The turn data contains an illegal turn in the current position: {0}")]
    IllegalTurn(String),
}
