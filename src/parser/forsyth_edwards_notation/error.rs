use thiserror::Error;

#[derive(Error, Debug)]
pub enum FenParserError {
    #[error("The `{missing_part}`-part is missing or missplaced in the input: `{fen_input}`")]
    FenDataMissing {
        missing_part: String,
        fen_input: String,
    },
    #[error("Invalid piece placement data: `{0}`")]
    InvalidPiecePlacementData(String),
    #[error("Invalid input `{input}` for part `{fen_part}`")]
    UnrecognisedContent { input: String, fen_part: String },
}
