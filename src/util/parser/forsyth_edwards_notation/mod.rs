//! Forsyth-Edwards-Notation.
//! Methods to convert `Position` into and load `Position` from FEN representations.

mod export;
mod import;

pub use export::export_to_fen as export;
pub(crate) use import::DEFAULT_BOARD_SETUP;
pub use import::import_from_fen as import;
