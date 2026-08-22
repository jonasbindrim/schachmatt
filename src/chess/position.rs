use thiserror::Error;

use crate::{
    Columns::COLUMN_AMOUNT, DEFAULT_BOARD_SETUP, Fen, Field, GameResult, Lan, Piece, PlayerColor,
    Rows::ROW_AMOUNT, Ruleset, Turn, chess::castling_rights::CastlingRights,
};

pub type BoardSetup = [[Option<Piece>; COLUMN_AMOUNT]; ROW_AMOUNT];

#[derive(Error, Debug)]
pub enum PositionError {
    #[error("The given turn is not legal in the current position: {0}")]
    IllegalTurnError(String),
}

/// A `Position` is defined as a state in a chess game and contains all information
/// to unambiguously identify a position.
#[derive(Clone, PartialEq, Debug)]
pub struct Position {
    board_position: BoardSetup,
    active_color: PlayerColor,
    castling_white: CastlingRights,
    castling_black: CastlingRights,
    en_passant: Option<Field>,
    halfmove_clock: u16,
    fullmove_counter: u16,
}

impl Position {
    /// Creates a new position with the given parameters.
    #[must_use]
    pub fn new(
        board_position: BoardSetup,
        active_color: PlayerColor,
        castling_white: CastlingRights,
        castling_black: CastlingRights,
        en_passant: Option<Field>,
        halfmove_clock: u16,
        fullmove_counter: u16,
    ) -> Position {
        Position {
            board_position,
            active_color,
            castling_white,
            castling_black,
            en_passant,
            halfmove_clock,
            fullmove_counter,
        }
    }

    /// Returns a reference to the current board position.
    #[must_use]
    pub fn get_board_position(&self) -> &BoardSetup {
        &self.board_position
    }

    /// Sets the board position to the given `BoardSetup`.
    pub fn set_board_position(&mut self, board_position: BoardSetup) {
        self.board_position = board_position;
    }

    /// Returns the piece at the specified `Field`.
    #[must_use]
    pub fn get_field_occupation(&self, field: &Field) -> Option<Piece> {
        self.board_position[field.get_row() as usize][field.get_column() as usize]
    }

    /// Sets the piece at the specified `Field`.
    pub fn set_field_occupation(&mut self, field: &Field, piece: Option<Piece>) {
        self.board_position[field.get_row() as usize][field.get_column() as usize] = piece;
    }

    /// Returns the currently active color.
    /// - `returns` - The currently active color
    #[must_use]
    pub fn get_active_color(&self) -> PlayerColor {
        self.active_color
    }

    /// Sets the currently active color.
    pub fn set_active_color(&mut self, active_color: PlayerColor) {
        self.active_color = active_color;
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

    /// Sets the castling rights for the specified `PlayerColor`.
    pub fn set_castling_rights(&mut self, color: PlayerColor, castling_rights: CastlingRights) {
        match color {
            PlayerColor::Black => self.castling_black = castling_rights,
            PlayerColor::White => self.castling_white = castling_rights,
        }
    }

    /// Returns the `Field` which can be captured using the en passant rule.
    /// - `returns` - The `Field` which can be captured using the en passant rule
    #[must_use]
    pub fn get_en_passant(&self) -> Option<Field> {
        self.en_passant
    }

    /// Sets the `Field` which can be captured using the en passant rule.
    pub fn set_en_passant(&mut self, en_passant: Option<Field>) {
        self.en_passant = en_passant;
    }

    /// Returns the amount of halfmoves played since the last pawn move or capture.
    /// - `returns` - The amount of halfmoves played since the last pawn move or capture
    #[must_use]
    pub fn get_halfmove_clock(&self) -> u16 {
        self.halfmove_clock
    }

    /// Sets the amount of halfmoves played since the last pawn move or capture.
    pub fn set_halfmove_clock(&mut self, halfmove_clock: u16) {
        self.halfmove_clock = halfmove_clock;
    }

    /// Returns the amount moves played to reach this position.
    /// - `returns` - The amount of moves played to reach this position
    #[must_use]
    pub fn get_fullmove_counter(&self) -> u16 {
        self.fullmove_counter
    }

    /// Sets the amount of moves played to reach this position.
    pub fn set_fullmove_counter(&mut self, fullmove_counter: u16) {
        self.fullmove_counter = fullmove_counter;
    }

    /// Returns a list of all possible legal turns from the current position
    pub fn get_possible_turns(&self, ruleset: &Ruleset) -> Vec<Turn> {
        ruleset.get_possible_turns(self)
    }

    /// Executes the given turn. Returns an error if the given turn is an illegal move.
    /// - `turn` - The turn which should be played
    pub fn turn(&self, ruleset: &Ruleset, turn: &Turn) -> Result<Position, PositionError> {
        let possible_moves = self.get_possible_turns(ruleset);
        if !possible_moves.contains(turn) {
            return Err(PositionError::IllegalTurnError(Lan::export(turn)));
        }

        Ok(ruleset.execute_turn(self, turn))
    }

    /// Returns the result of the game in the current position.
    /// - `returns` - The game result in the current position
    #[must_use]
    pub fn game_over_check(&self, ruleset: &Ruleset) -> Option<GameResult> {
        ruleset.game_over_check(self)
    }
}

impl Default for Position {
    /// Creates a `Position` with the classical board setup.
    fn default() -> Self {
        Fen::import(DEFAULT_BOARD_SETUP).unwrap()
    }
}
