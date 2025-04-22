use crate::{Field, Piece, PlayerColor};

use super::util::castling_rights::CastlingRights;

pub type BoardSetup = [[Option<Piece>; COLUMN_AMOUNT]; ROW_AMOUNT];
pub(crate) const ROW_AMOUNT: usize = 8;
pub(crate) const COLUMN_AMOUNT: usize = 8;

/// A `Position` is defined as a state in a chess game.
#[derive(Clone, PartialEq, Debug)]
pub struct Position {
    // For the board position: The first array dimension is the row, the second one is the column
    pub(crate) board_position: BoardSetup,
    pub(crate) active_color: PlayerColor,
    pub(crate) castling_white: CastlingRights,
    pub(crate) castling_black: CastlingRights,
    pub(crate) en_passant: Option<Field>,
    pub(crate) halfmove_clock: u16,
    pub(crate) fullmove_counter: u16,
}
