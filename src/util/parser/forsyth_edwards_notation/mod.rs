//! Forsyth-Edwards-Notation.  
//! Methods to convert `Position` into FEN notation and import `Position` from FEN representations.

mod error;
mod export;
mod import;

pub use export::export_to_fen as export;
pub(crate) use import::DEFAULT_BOARD_SETUP;
pub use import::import_from_fen as import;
