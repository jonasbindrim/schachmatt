mod error;
mod export;
mod import;

pub use error::FenParserError;

/// Forsyth-Edwards-Notation
/// Standardized notation to represent chess positions.
pub struct Fen;

/// The default board setup in Forsyth-Edwards-Notation.
pub const DEFAULT_BOARD_SETUP: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
