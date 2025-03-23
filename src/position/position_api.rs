use crate::{
    FEN, Field, GameResult, Piece, PlayerColor, Position, Turn,
    data_structures::piece::{piece_move_iterator::PieceMoveIterator, piece_type::PieceType},
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

            let mut piece_iterator =
                PieceMoveIterator::new(piece.movement_modifiers(), Field::from_usize(column, row));

            loop {
                while let Some(mut turn) = piece_iterator.current() {
                    // if turn is a promotion turn insert a dummy figure to make the move legal
                    if is_pawn && (turn.to.row == 7 || turn.to.row == 0) {
                        turn.promotion = Some(PieceType::Queen);
                    }

                    match self.is_legal_move(turn, true) {
                        MoveLegality::TemporarelyIllegal => continue,
                        MoveLegality::FullyIllegal => break,
                        MoveLegality::Legal => {
                            if is_pawn && (turn.to.row == 7 || turn.to.row == 0) {
                                turns.append(&mut Position::push_turn(turn));
                            } else {
                                turns.push(turn);
                            }
                        }
                        MoveLegality::LastLegal => {
                            if is_pawn && (turn.to.row == 7 || turn.to.row == 0) {
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

    /// Executes the given turn.
    /// - `action` - The turn which should be played
    /// # Panics
    /// This panic indicates an error in the library.
    pub fn turn(&mut self, action: &Turn) {
        let from_field = self.get_field_occupation(action.from);
        let Some(moving_piece) = from_field else {
            todo!() // TODO: Do something if illegal move is played
        };

        let to_field = self.get_field_occupation(action.to);

        // Increase move counter if no piece has been taken and no pawn has been moved
        if moving_piece.get_type() == PieceType::Pawn || to_field.is_some() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        // Move the piece
        self.board_position[action.to.row as usize][action.to.column as usize] = Some(moving_piece);
        self.board_position[action.from.row as usize][action.from.column as usize] = None;

        if PieceType::King == moving_piece.get_type() {
            match moving_piece.get_color() {
                PlayerColor::Black => {
                    if action.from.row == 7 && action.from.column == 4 {
                        if action.to.row == 7 && action.to.column == 2 {
                            self.board_position[7][3] = self.board_position[7][0];
                            self.board_position[7][0] = None;
                        } else if action.to.row == 7 && action.to.column == 6 {
                            self.board_position[7][5] = self.board_position[7][7];
                            self.board_position[7][7] = None;
                        }
                    }
                    self.castling_black.kingside = false;
                    self.castling_black.queenside = false;
                }
                PlayerColor::White => {
                    if action.from.row == 0 && action.from.column == 4 {
                        if action.to.row == 0 && action.to.column == 2 {
                            self.board_position[0][3] = self.board_position[0][0];
                            self.board_position[0][0] = None;
                        } else if action.to.row == 0 && action.to.column == 6 {
                            self.board_position[0][5] = self.board_position[0][7];
                            self.board_position[0][7] = None;
                        }
                    }
                    self.castling_white.kingside = false;
                    self.castling_white.queenside = false;
                }
            }
        }

        // Remove castling rights if the rook moves
        if PieceType::Rook == moving_piece.get_type() {
            match moving_piece.get_color() {
                PlayerColor::Black => {
                    if action.from.row == 7 && action.from.column == 0 {
                        self.castling_black.queenside = false;
                    } else if action.from.row == 7 && action.from.column == 7 {
                        self.castling_black.kingside = false;
                    }
                }
                PlayerColor::White => {
                    if action.from.row == 0 && action.from.column == 0 {
                        self.castling_white.queenside = false;
                    } else if action.from.row == 0 && action.from.column == 7 {
                        self.castling_white.kingside = false;
                    }
                }
            }
        }

        // Promote if possible
        if PieceType::Pawn == moving_piece.get_type() && (action.to.row == 0 || action.to.row == 7)
        {
            self.board_position[action.to.row as usize][action.to.column as usize] =
                Some(Piece::new(action.promotion.unwrap(), self.active_color));
        }

        // Remove piece taken with en passant
        if let Some(field) = self.en_passant {
            if action.to.column == field.column && action.from.row == field.row {
                self.board_position[field.row as usize][field.column as usize] = None;
                self.halfmove_clock = 0;
            }
        }

        // Empty en-passant field
        self.en_passant = None;

        // Check if a pawn has moved two field for possible en passant
        if PieceType::Pawn == moving_piece.get_type()
            && action.from.row.abs_diff(action.to.row) == 2
        {
            self.en_passant = Some(action.to);
        }

        // Raise fullmove counter
        if self.active_color == PlayerColor::Black {
            self.fullmove_counter += 1;
        }

        // Change color at turn
        self.active_color = self.active_color.reverse();
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
