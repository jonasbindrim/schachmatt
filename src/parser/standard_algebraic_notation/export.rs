use crate::{
    CLASSIC_RULESET, Piece, PieceType, PlayerColor, Position, Ruleset, Turn,
    chess::turn::{CastleDirection, NormalTurn},
};

use super::San;

impl San {
    /// Exports a `Turn` into its corresponding SAN representation.
    /// This function assumes that the classical chess ruleset is used.
    /// If that is not the case, refer to the `export_by_ruleset` function.
    /// - `turn` The turn object that will be converted
    /// - `current_position` The position in which the turn is played
    /// - `returns` The resulting san string
    #[must_use]
    pub fn export(turn: &Turn, current_position: &Position) -> String {
        Self::export_by_ruleset(turn, current_position, &CLASSIC_RULESET)
    }

    /// Exports a `Turn` into its corresponding SAN representation.
    /// - `turn` The turn object that will be converted
    /// - `current_position` The position in which the turn is played
    /// - `ruleset` The chess ruleset used in the game the move was played in.
    /// - `returns` The resulting san string
    #[must_use]
    pub fn export_by_ruleset(
        turn: &Turn,
        current_position: &Position,
        ruleset: &Ruleset,
    ) -> String {
        // Check if the turn is a castling turn
        let turn = match turn {
            Turn::Normal(normal_turn) => normal_turn,
            Turn::Castle(castle_direction) => {
                return match castle_direction {
                    CastleDirection::Kingside => String::from("O-O"),
                    CastleDirection::Queenside => String::from("O-O-O"),
                };
            }
        };

        let mut san_turn = String::new();

        let mut is_capture = current_position
            .get_field_occupation(&turn.target)
            .is_some();
        let from_field = current_position.get_field_occupation(&turn.origin);

        let Some(moving_piece) = from_field else {
            todo!() // TODO: Handle illegal move
        };

        if moving_piece.get_type() == PieceType::Pawn {
            let check_field: i8 = {
                match current_position.get_active_color() {
                    PlayerColor::Black => (turn.target.get_row() as i8) + 1,
                    PlayerColor::White => (turn.target.get_row() as i8) - 1,
                }
            };

            if let Some(field) = current_position.get_en_passant()
                && turn.target.get_column() == field.get_column()
                && check_field == field.get_row() as i8
            {
                is_capture = true;
            }

            if is_capture {
                san_turn.push((turn.origin.get_column() + b'a') as char);
            }
            Self::to_move(&mut san_turn, turn, current_position, is_capture, ruleset);
        } else {
            san_turn.push(PieceType::export_piecetype_uppercase(
                moving_piece.get_type(),
            ));
            Self::add_field_descriptor(&mut san_turn, turn, current_position, ruleset);
            Self::to_move(&mut san_turn, turn, current_position, is_capture, ruleset);
        }

        san_turn
    }

    /// Checks and adds the needed amount of descriptors for a turn
    /// - `base` - The base string which gets data appended to
    /// - `turn` - The turn which was played
    /// - `current_position` - The position the turn was played in
    /// - `ruleset` - The ruleset to use for checking the turn
    fn add_field_descriptor(
        base: &mut String,
        turn: &NormalTurn,
        current_position: &Position,
        ruleset: &Ruleset,
    ) {
        let column = turn.origin.get_column();
        let row = turn.origin.get_row();

        let occupation = current_position.get_field_occupation(&turn.origin);
        if !Self::is_unique_descriptor(turn, current_position, occupation, None, None, ruleset) {
            if Self::is_unique_descriptor(
                turn,
                current_position,
                occupation,
                Some(column),
                None,
                ruleset,
            ) {
                base.push((column + b'a') as char);
            } else if Self::is_unique_descriptor(
                turn,
                current_position,
                occupation,
                None,
                Some(row),
                ruleset,
            ) {
                base.push((row + b'1') as char);
            } else {
                base.push((column + b'a') as char);
                base.push((row + b'1') as char);
            }
        }
    }

    /// Adds capture `target_field`, promotion and checks to a san string
    /// - `base` - The base string of the output
    /// - `turn` - The turn which was played
    /// - `current_position` - The position the turn was played at
    /// - `is_capture` - Indicates whether the current turn is a capturing turn
    /// - `ruleset` - The ruleset to use for checking the turn
    fn to_move(
        base: &mut String,
        turn: &NormalTurn,
        current_position: &Position,
        is_capture: bool,
        ruleset: &Ruleset,
    ) {
        // Add capture
        if is_capture {
            base.push('x');
        }

        // Add target_field
        base.push_str(&turn.target.to_string());

        // Check if promotion
        if let Some(piece) = turn.promotion {
            base.push_str(&format!(
                "={}",
                PieceType::export_piecetype_uppercase(piece)
            ));
        }

        // Check if is in check
        let copy_position = ruleset.execute_turn(&current_position.clone(), &Turn::Normal(*turn));
        if ruleset.is_in_check(&copy_position, copy_position.get_active_color()) {
            if ruleset.get_possible_turns(&copy_position).is_empty() {
                base.push('#');
            } else {
                base.push('+');
            }
        }
    }

    /// Checks whether the move description is already unique
    /// - `checked_turn` - The turn which gets tested for uniqueness
    /// - `current_position` - The current position of the game
    /// - `piece` - The moving piece
    /// - `column` - The column the piece is located at
    /// - `row` - The row the piece is located at
    /// - `ruleset` - The ruleset to use for checking the turn
    /// - `returns` - Whether the move indicator is unique
    fn is_unique_descriptor(
        checked_turn: &NormalTurn,
        current_position: &Position,
        occupation: Option<Piece>,
        column: Option<u8>,
        row: Option<u8>,
        ruleset: &Ruleset,
    ) -> bool {
        let possible_moves = ruleset.get_possible_turns(current_position);
        let possible_moves: Vec<&NormalTurn> = possible_moves
            .iter()
            .filter_map(|turn| match turn {
                Turn::Normal(normal_turn) => Some(normal_turn),
                Turn::Castle(_) => None,
            })
            .collect();

        let mut counter = 0;
        for turn in possible_moves {
            if turn.target == checked_turn.target
                && occupation == current_position.get_field_occupation(&turn.origin)
            {
                match column {
                    Some(column_value) => {
                        if turn.origin.get_column() == column_value {
                            match row {
                                Some(row_value) => {
                                    if turn.origin.get_row() == row_value {
                                        counter += 1;
                                    }
                                }
                                None => counter += 1,
                            }
                        }
                    }
                    None => match row {
                        Some(row_value) => {
                            if turn.origin.get_row() == row_value {
                                counter += 1;
                            }
                        }
                        None => counter += 1,
                    },
                }
            }
        }
        counter == 1
    }
}
