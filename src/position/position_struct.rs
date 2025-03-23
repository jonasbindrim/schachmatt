use crate::{Field, Piece, PlayerColor};

use super::util::castling_rights::CastlingRights;

pub type BoardSetup = [[Option<Piece>; 8]; 8];

/// A `Position` is defined as a state in a chess game.
#[derive(Clone, PartialEq)]
#[cfg_attr(test, derive(Debug))]
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

impl Position {
    /// Returns a copy of the current board position.
    /// - `returns` - A copy of the current board position
    #[must_use]
    pub fn get_board_position(&self) -> BoardSetup {
        self.board_position
    }

    /// Returns the currently active color.
    /// - `returns` - The currently active color
    #[must_use]
    pub fn get_active_color(&self) -> PlayerColor {
        self.active_color
    }

    /// Returns the castling rights for the specified `PlayerColor`.
    /// - `color` - The color for which to return the `CastlingRights`
    /// - `returns` - The castling right for the specified `PlayerColor`
    #[must_use]
    pub fn get_castling_rights(&self, color: PlayerColor) -> CastlingRights {
        match color {
            PlayerColor::Black => self.castling_black,
            PlayerColor::White => self.castling_white,
        }
    }

    /// Returns the `Field` which can be captured using the en passant rule.
    /// - `returns` - The `Field` which can be captured using the en passant rule
    #[must_use]
    pub fn get_en_passant(&self) -> Option<Field> {
        self.en_passant
    }

    /// Returns the amount of halfmoves played to reach this position.
    /// - `returns` - The amount of halfmoves played to reach this position
    #[must_use]
    pub fn get_halfmove_counter(&self) -> u16 {
        self.halfmove_clock
    }

    /// Returns the amount moves played to reach this position.
    /// - `returns` - The amount of moves played to reach this position
    #[must_use]
    pub fn get_fullmove_counter(&self) -> u16 {
        self.fullmove_counter
    }
}
