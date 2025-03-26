use crate::{
    Board, Piece, PlayerColor, Position, Turn, data_structures::piece::piece_type::PieceType,
};

/// Converts a `Turn` into its corresponding SAN representation.
/// - `turn` - The turn object that will be converted
/// - `current_position` - The position in which the turn is player
/// - `result` - The resulting san string
/// # Panics
/// This panic indicates an error in the library.
#[must_use]
pub fn from_turn(turn: Turn, current_position: &Position) -> String {
    let mut san_turn = String::new();

    let mut is_capture = current_position.get_field_occupation(turn.to).is_some();
    let from_field = current_position.get_field_occupation(turn.from);

    let Some(moving_piece) = from_field else {
        todo!() // TODO: Handle illegal move
    };

    if moving_piece.get_type() == PieceType::Pawn {
        let check_field: i8 = {
            match current_position.active_color {
                PlayerColor::Black => (turn.to.row as i8) + 1,
                PlayerColor::White => (turn.to.row as i8) - 1,
            }
        };

        if let Some(field) = current_position.en_passant {
            if turn.to.column == field.column && check_field == field.row as i8 {
                is_capture = true;
            }
        }

        if is_capture {
            san_turn.push((turn.from.column + b'a') as char);
        }
        to_move(&mut san_turn, turn, current_position, is_capture);
    } else {
        if PieceType::King == moving_piece.get_type() {
            // Is kingside castle
            if turn.from.column + 2 == turn.to.column {
                return String::from("O-O");
            }

            if turn.from.column == Board::COLUMN_E && turn.from.column - 2 == turn.to.column {
                return String::from("O-O-O");
            }
        }
        san_turn.push(PieceType::export_piecetype_uppercase(
            moving_piece.get_type(),
        ));
        add_field_descriptor(&mut san_turn, turn, current_position);
        to_move(&mut san_turn, turn, current_position, is_capture);
    }

    san_turn
}

/// Checks and adds the needed amount of descriptors for a turn
/// - `base` - The base string which gets data appended to
/// - `turn` - The turn which was played
/// - `current_position` - The position the turn was played in
fn add_field_descriptor(base: &mut String, turn: Turn, current_position: &Position) {
    let column = turn.from.column;
    let row = turn.from.row;

    let occupation = current_position.get_field_occupation(turn.from);
    if !is_unique_descriptor(turn, current_position, occupation, None, None) {
        if is_unique_descriptor(turn, current_position, occupation, Some(column), None) {
            base.push((column + b'a') as char);
        } else if is_unique_descriptor(turn, current_position, occupation, None, Some(row)) {
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
fn to_move(base: &mut String, turn: Turn, current_position: &Position, is_capture: bool) {
    // Add capture
    if is_capture {
        base.push('x');
    }

    // Add target_field
    base.push_str(&turn.to.to_string());

    // Check if promotion
    if let Some(piece) = turn.promotion {
        base.push_str(&format!(
            "={}",
            PieceType::export_piecetype_uppercase(piece)
        ));
    }

    // Check if is in check
    let mut copy_position = current_position.clone();
    copy_position.turn(&turn).unwrap();
    if copy_position.is_in_check(copy_position.get_active_color()) {
        if copy_position.get_possible_moves().is_empty() {
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
/// - `returns` - Whether the move indicator is unique
fn is_unique_descriptor(
    checked_turn: Turn,
    current_position: &Position,
    occupation: Option<Piece>,
    column: Option<u8>,
    row: Option<u8>,
) -> bool {
    let possible_moves = current_position.get_possible_moves();

    let mut counter = 0;
    for turn in possible_moves {
        if turn.to == checked_turn.to
            && occupation == current_position.get_field_occupation(turn.from)
        {
            match column {
                Some(column_value) => {
                    if turn.from.column == column_value {
                        match row {
                            Some(row_value) => {
                                if turn.from.row == row_value {
                                    counter += 1;
                                }
                            }
                            None => counter += 1,
                        }
                    }
                }
                None => match row {
                    Some(row_value) => {
                        if turn.from.row == row_value {
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
