use crate::{data_structures::piece::piece_type::PieceType, Piece, PlayerColor, Position, Turn};

/// Converts a `Turn` into its corresponding SAN representation.
/// - `turn` - The turn object that will be converted
/// - `current_position` - The position in which the turn is player
/// - `result` - The resulting san string
/// # Panics
/// This panic indicates an error in the library.
#[must_use]
pub fn from_turn(turn: Turn, current_position: &Position) -> String {
    let mut san_turn = String::new();

    let target_field =
        current_position.board_position[turn.to.row as usize][turn.to.column as usize];
    let moving_piece =
        current_position.board_position[turn.from.row as usize][turn.from.column as usize];

    match moving_piece.get_type() {
        PieceType::None => unreachable!(),
        PieceType::Pawn => {
            let check_field: i8 = {
                match current_position.active_color {
                    PlayerColor::Black => (turn.to.row as i8) + 1,
                    PlayerColor::White => (turn.to.row as i8) - 1,
                }
            };

            let mut is_capture: bool = PieceType::None != target_field.get_type();
            if let Some(field) = current_position.en_passant {
                if turn.to.column == field.column && check_field == field.row as i8 {
                    is_capture = true;
                }
            }

            if is_capture {
                san_turn.push((turn.from.column + b'a') as char);
            }
            to_move(&mut san_turn, turn, current_position, is_capture);
        }
        _ => {
            if PieceType::King == moving_piece.get_type() {
                // Is kingside castle
                if turn.from.column + 2 == turn.to.column {
                    return String::from("O-O");
                } else if turn.from.column == 4 && turn.from.column - 2 == turn.to.column {
                    return String::from("O-O-O");
                }
            }
            san_turn.push(PieceType::export_piecetype_uppercase(moving_piece.get_type()).unwrap());
            add_field_descriptor(&mut san_turn, turn, current_position);

            let captured_piece =
                current_position.board_position[turn.to.row as usize][turn.to.column as usize];
            let is_capture = PieceType::None != captured_piece.get_type();
            to_move(&mut san_turn, turn, current_position, is_capture);
        }
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

    let piece = current_position.board_position[row as usize][column as usize];
    if !is_unique_descriptor(turn, current_position, piece, None, None) {
        if is_unique_descriptor(turn, current_position, piece, Some(column), None) {
            base.push((column + b'a') as char);
        } else if is_unique_descriptor(turn, current_position, piece, None, Some(row)) {
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
        base.push('=');
        base.push(PieceType::export_piecetype_uppercase(piece).unwrap());
    }

    // Check if is in check
    let mut copy_position = current_position.clone();
    copy_position.turn(&turn);
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
    piece: Piece,
    column: Option<u8>,
    row: Option<u8>,
) -> bool {
    let possible_moves = current_position.get_possible_moves();

    let mut counter = 0;
    for turn in possible_moves {
        if turn.to == checked_turn.to
            && piece
                == current_position.board_position[turn.from.row as usize]
                    [turn.from.column as usize]
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
