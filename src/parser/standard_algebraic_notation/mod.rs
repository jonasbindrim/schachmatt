mod error;
mod export;
mod import;
mod san_turn;

/// `SAN` stands for `Standard algebraic notation` and is a standardized notation to describe chess moves.
/// A lot of other format chess related data format rely on `SAN` as a basis. This struct contains functionality
/// to import chess moves in the `SAN` and to export chess moves to the `SAN`.
///
/// Chess moves described in `SAN` are always context dependant. The same move in `SAN` can have different meanings
/// depending on the position they were played in.
pub struct San;

pub use error::SanParserError;
