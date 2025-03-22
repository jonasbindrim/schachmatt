use crate::{
    Field, Piece, PlayerColor, Position, Turn, data_structures::piece::piece_type::PieceType,
};

use pest::{Parser, iterators::Pair};

#[derive(Parser)]
#[grammar = "util/parser/standard_algebraic_notation/san.pest"]
struct SanStruct;

/// Converts a string in SAN representation to a `Turn` object.
/// - `raw` - The raw san string
/// - `current_position` - The position the turn was played in
/// - `returns` - The `Turn` as an object
/// # Panics
/// This panic indicates an error in the library.
pub fn from_string(raw: &str, current_position: &mut Position) -> Option<Turn> {
    // Cut potential "+" from raw string data as it doesnt convey any needed information

    let mut san_data = raw;
    if let Some(index) = san_data.find('+') {
        san_data = &san_data[0..index];
    }

    // Parse SAN data
    let Ok(mut parsed_data) = SanStruct::parse(Rule::turn, san_data) else {
        return None;
    };

    if let Some(turn_type) = parsed_data.next().unwrap().into_inner().next() {
        match turn_type.as_rule() {
            Rule::pawn_move => {
                return import_pawn_movement(turn_type, current_position);
            }
            Rule::castling => {
                return import_handle_castling(&turn_type, current_position);
            }
            Rule::piece_move_full => {
                return import_piece_move_full(turn_type, current_position);
            }
            _ => return None,
        }
    }
    unreachable!()
}

/// Converts the full piece move into a turn
/// - `san_data` - The pest parsed san data
/// - `position` - The current game position
/// - `returns` - The resulting turn
fn import_piece_move_full(san_data: Pair<Rule>, position: &Position) -> Option<Turn> {
    let possible_moves = position.get_possible_moves();

    let mut piece_type: Option<Piece> = None;
    let mut target_field: Option<Field> = None;
    let mut from_column: Option<u8> = None;
    let mut from_row: Option<u8> = None;

    for parts in san_data.into_inner() {
        match parts.as_rule() {
            Rule::piece_symbol => {
                let letter = parts.as_str().as_bytes()[0] as char;
                let piece = PieceType::import_piecetype(letter.to_ascii_lowercase());

                if let Some(piece) = piece {
                    piece_type = Some(Piece::new(piece, Some(position.get_active_color())));
                } else {
                    piece_type = None;
                }
            }
            Rule::piece_move => {
                let (target, column, row) = import_piece_move(parts);
                target_field = Some(target);
                from_column = column;
                from_row = row;
            }
            _ => return None,
        }
    }

    for turn in possible_moves {
        if target_field.unwrap() == turn.to
            && position.board_position[turn.from.row as usize][turn.from.column as usize]
                == piece_type.unwrap()
        {
            let mut ok: bool = true;
            if let Some(column_value) = from_column {
                if turn.from.column != column_value {
                    ok = false;
                }
            }
            if let Some(row_value) = from_row {
                if turn.from.row != row_value {
                    ok = false;
                }
            }
            if ok {
                return Some(turn);
            }
        }
    }
    None
}

/// Converts a simple piece move
/// - `san_data` - The pest parsed san data
/// - `returns` - The target field
fn import_piece_move(san_data: Pair<Rule>) -> (Field, Option<u8>, Option<u8>) {
    let mut from_column: Option<u8> = None;
    let mut from_row: Option<u8> = None;

    for part in san_data.into_inner() {
        match part.as_rule() {
            Rule::to_field => {
                return (
                    Field::from_string(part.as_str()).unwrap(),
                    from_column,
                    from_row,
                );
            }
            Rule::from_field => {
                let data = part.as_str().as_bytes();
                if data[0] >= b'a' && data[0] <= b'h' {
                    from_column = Some(data[0] - b'a');
                    if data.len() > 1 {
                        from_row = Some(data[1] - b'1');
                    }
                } else {
                    from_row = Some(data[0] - b'1');
                }
            }
            _ => unreachable!(),
        }
    }
    unreachable!();
}

/// Converts the san castling moves into turns
/// - `san_data` - The pest parsed turn data
/// - `position` - The position in which the turn was played
/// - `returns` - The resulting `Turn`
fn import_handle_castling(san_data: &Pair<Rule>, position: &Position) -> Option<Turn> {
    let possible_moves = position.get_possible_moves();
    let player_color = position.get_active_color();

    // Initiate with row for white
    let mut target_field: Field = Field { column: 0, row: 0 };
    let mut starting_field: Field = Field { column: 4, row: 0 };

    // Change row if color is black
    if player_color == PlayerColor::Black {
        starting_field.row = 7;
        target_field.row = 7;
    }

    // Check if castle is king or queenside
    match san_data.as_str() {
        "O-O" | "0-0" => {
            target_field.column = 6;
        }
        "O-O-O" | "0-0-0" => {
            target_field.column = 2;
        }
        _ => return None,
    };

    Some(
        possible_moves
            .into_iter()
            .find(|&turn| turn.to == target_field && turn.from == starting_field)
            .unwrap(),
    )
}

/// Convert the san pawn moves into turns
fn import_pawn_movement(san_data: Pair<Rule>, position: &Position) -> Option<Turn> {
    let possible_moves = position.get_possible_moves();

    let mut target_field: Option<Field> = None;
    let mut promotion_piece: Option<PieceType> = None;
    let mut from_column: Option<u8> = None;
    let mut from_row: Option<u8> = None;

    // Create target field and promotion target
    for pawn_push in san_data.into_inner() {
        match pawn_push.as_rule() {
            Rule::to_field => target_field = Field::from_string(pawn_push.as_str()),
            Rule::promotion_piece => {
                let letter = (pawn_push.as_str().as_bytes()[0] as char).to_ascii_lowercase();
                let piece_type = PieceType::import_piecetype(letter).unwrap();
                promotion_piece = Some(piece_type);
            }
            Rule::from_field => {
                let data = pawn_push.as_str().as_bytes();
                from_column = Some(data[0] - b'a');
                if data.len() > 1 {
                    from_row = Some(data[1] - 1);
                }
            }
            _ => unreachable!(),
        }
    }

    for turn in possible_moves {
        if target_field.unwrap() == turn.to
            && promotion_piece == turn.promotion
            && PieceType::Pawn
                == position.board_position[turn.from.row as usize][turn.from.column as usize]
                    .get_type()
        {
            match from_column {
                Some(column) => {
                    // Is a capture move
                    match from_row {
                        Some(row) => {
                            if column == turn.from.column && row == turn.from.row {
                                return Some(turn);
                            }
                        }
                        None => {
                            if column == turn.from.column {
                                return Some(turn);
                            }
                        }
                    }
                }
                None => return Some(turn),
            }
        }
    }
    None
}
