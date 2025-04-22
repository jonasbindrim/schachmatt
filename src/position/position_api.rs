use crate::{
    piece::piece_move_iterator::PieceMoveIterator, Board, Field, GameResult, PieceType, PlayerColor, Position, PositionError, Turn, FEN, LAN
};

use super::{position_internal::BOARD_FIELDS, position_struct::BoardSetup, util::{castling_rights::CastlingRights, move_legality::MoveLegality}};

impl Position {
    /// Creates a new position
    /// - `returns` - A new position with the default board setup
    /// # Panics
    /// This panic indicates an error in the library.
    #[must_use]
    pub fn new() -> Position {
        FEN::import(FEN::DEFAULT_BOARD_SETUP).unwrap()
    }

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

    /// Checks which moves are possible for the player which has to move and
    /// returns an array containing all the possible moves.
    /// - `returns` - An array of all the possible moves
    /// # Panics
    /// This panic indicates an error in the library.
    #[must_use]
    pub fn get_possible_moves(&self) -> Vec<Turn> {
        let mut turns: Vec<Turn> = Vec::<Turn>::new();

        for field in BOARD_FIELDS {
            let Some(piece) = self.get_field_occupation(&field) else {
                continue;
            };

            if piece.get_color() != self.active_color {
                continue;
            }

            // Check if current piece is a pawn
            let is_pawn = PieceType::Pawn == piece.get_type();

            let mut piece_iterator = PieceMoveIterator::new(piece.movement_modifiers(), field);

            loop {
                while let Some(mut turn) = piece_iterator.current() {
                    // if turn is a promotion turn insert a dummy figure to make the move legal
                    if is_pawn && matches!(turn.target.row, Board::ROW_8 | Board::ROW_1) {
                        turn.promotion = Some(PieceType::Queen);
                    }

                    match self.is_legal_move(turn, true) {
                        MoveLegality::TemporarelyIllegal => continue,
                        MoveLegality::FullyIllegal => break,
                        MoveLegality::Legal => {
                            if is_pawn && matches!(turn.target.row, Board::ROW_8 | Board::ROW_1) {
                                turns.append(&mut Position::push_turn(turn));
                            } else {
                                turns.push(turn);
                            }
                        }
                        MoveLegality::LastLegal => {
                            if is_pawn && matches!(turn.target.row, Board::ROW_8 | Board::ROW_1) {
                                turns.append(&mut Position::push_turn(turn));
                            } else {
                                turns.push(turn);
                            }
                            break;
                        }
                    }
                }
                if !piece_iterator.step() {
                    break;
                }
            }
            continue;
        }

        turns
    }

    /// Executes the given turn. Returns an error if the given turn is an illegal move.
    /// - `action` - The turn which should be played
    /// # Panics
    /// This panic indicates an error in the library.
    pub fn turn(&mut self, action: &Turn) -> Result<(), PositionError> {
        let possible_moves = self.get_possible_moves();
        if !possible_moves.contains(action) {
            return Err(PositionError::IllegalTurnError(LAN::export(action)));
        }

        self.internal_turn(action);
        Ok(())
    }

    /// Returns the result of the game in the current position.
    /// - `returns` - The game result in the current position
    #[must_use]
    pub fn game_over_check(&self) -> Option<GameResult> {
        // Check for insufficient material
        if !self.is_sufficient_material() {
            return Some(GameResult::Draw);
        }

        // Check all other rules
        if !self.get_possible_moves().is_empty() {
            if self.halfmove_clock == 50 {
                return Some(GameResult::Draw);
            }
            return None;
        } else if self.is_in_check(self.active_color) {
            return Some(GameResult::Over(self.active_color.reverse()));
        }
        Some(GameResult::Draw)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}
