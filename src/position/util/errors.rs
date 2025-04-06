use thiserror::Error;

#[derive(Error, Debug)]
pub enum PositionError {
    #[error("The given turn is not legal in the current position: {0}")]
    IllegalTurnError(String),
}
