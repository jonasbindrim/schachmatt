use crate::{Field, Piece, PlayerColor};

use super::util::castling_rights::CastlingRights;

pub type BoardSetup = [[Option<Piece>; COLUMN_AMOUNT]; ROW_AMOUNT];
pub(crate) const ROW_AMOUNT: usize = 8;
pub(crate) const COLUMN_AMOUNT: usize = 8;

/// A `Position` is defined as a state in a chess game.
#[derive(Clone, PartialEq, Debug)]
pub struct Position {
    // For the board position: The first array dimension is the row, the second one is the column
    pub(super) board_position: BoardSetup,
    pub(super) active_color: PlayerColor,
    pub(super) castling_white: CastlingRights,
    pub(super) castling_black: CastlingRights,
    pub(super) en_passant: Option<Field>,
    pub(super) halfmove_clock: u16,
    pub(super) fullmove_counter: u16,
}
