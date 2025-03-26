use crate::{
    Board, FEN, Field, GameResult, Position, Turn,
    data_structures::piece::{piece_move_iterator::PieceMoveIterator, piece_type::PieceType},
    util::error::error_messages::ILLEGAL_TURN_ERROR,
};

use super::{position_internal::BOARD_FIELDS, util::move_legality::MoveLegality};

impl Position {
    /// Creates a new game
    /// - `returns` - A new position with the default board setup
    /// # Panics
    /// This panic indicates an error in the library.
    #[must_use]
    pub fn new() -> Position {
        FEN::import(FEN::DEFAULT_BOARD_SETUP).unwrap()
    }

    /// Checks which moves are possible for the player which has to move and
    /// returns an array containing all the possible moves.
    /// - `returns` - An array of all the possible moves
    /// # Panics
    /// This panic indicates an error in the library.
    #[must_use]
    pub fn get_possible_moves(&self) -> Vec<Turn> {
        let mut turns: Vec<Turn> = Vec::<Turn>::new();

        for (row, column) in BOARD_FIELDS {
            let piece = match self.board_position[row][column] {
                None => continue,
                Some(piece) => piece,
            };
            if piece.get_color() != self.active_color {
                continue;
            }

            // Check if current piece is a pawn
            let is_pawn = PieceType::Pawn == piece.get_type();

            let mut piece_iterator = PieceMoveIterator::new(
                piece.movement_modifiers(),
                Field::new_from_usize(column, row).unwrap(),
            );

            loop {
                while let Some(mut turn) = piece_iterator.current() {
                    // if turn is a promotion turn insert a dummy figure to make the move legal
                    if is_pawn && (turn.to.row == Board::ROW_8 || turn.to.row == Board::ROW_1) {
                        turn.promotion = Some(PieceType::Queen);
                    }

                    match self.is_legal_move(turn, true) {
                        MoveLegality::TemporarelyIllegal => continue,
                        MoveLegality::FullyIllegal => break,
                        MoveLegality::Legal => {
                            if is_pawn
                                && (turn.to.row == Board::ROW_8 || turn.to.row == Board::ROW_1)
                            {
                                turns.append(&mut Position::push_turn(turn));
                            } else {
                                turns.push(turn);
                            }
                        }
                        MoveLegality::LastLegal => {
                            if is_pawn
                                && (turn.to.row == Board::ROW_8 || turn.to.row == Board::ROW_1)
                            {
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
    pub fn turn(&mut self, action: &Turn) -> Result<(), &str> {
        let possible_moves = self.get_possible_moves();
        if !possible_moves.contains(action) {
            return Err(ILLEGAL_TURN_ERROR);
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
