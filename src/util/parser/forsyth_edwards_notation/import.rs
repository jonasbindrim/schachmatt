use crate::{
    Board, Field, Piece, PlayerColor, Position,
    position::{
        position_struct::{BoardSetup, COLUMN_AMOUNT, ROW_AMOUNT},
        util::castling_rights::CastlingRights,
    },
};

use super::error::FenParserError;

pub(crate) const DEFAULT_BOARD_SETUP: &str =
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

const PIECE_PLACEMENT_DATA: &str = "Piece placement data";
const ACTIVE_COLOR: &str = "Active color";
const CASTLING_AVAILABILITY: &str = "Castling availability";
const EN_PASSANT_TARGET_SQUARE: &str = "En passant target square";
const HALFMOVE_CLOCK: &str = "Halfmove clock";
const FULLMOVE_NUMBER: &str = "Fullmove number";

/// Converts a string in FEN notation into a `Position`.
/// which is described in the fen notation.
/// - `fen_input` - A string containing a position as a fen notation
/// - `returns` - The position described in the input string or an error
/// # Errors
/// Returns a `FenParserError` when the input cannot be converted into `Position`.
pub fn import_from_fen(fen_input: &str) -> Result<Position, FenParserError> {
    // Split fen string and check if amount of blocks is correct
    let fen_parts: Vec<&str> = fen_input.split_ascii_whitespace().collect();

    // 1. Piece placement data
    let Some(piece_placement_input) = fen_parts.first() else {
        return Err(FenParserError::FenDataMissing {
            missing_part: PIECE_PLACEMENT_DATA.to_string(),
            fen_input: fen_input.to_string(),
        });
    };
    let mut board_position = [[None; COLUMN_AMOUNT]; ROW_AMOUNT];
    compute_piece_placement_data(piece_placement_input, &mut board_position)?;

    // 2. Active color
    let Some(active_color) = fen_parts.get(1) else {
        return Err(FenParserError::FenDataMissing {
            missing_part: ACTIVE_COLOR.to_string(),
            fen_input: fen_input.to_string(),
        });
    };
    let active_color: PlayerColor = string_to_active_color(active_color)?;

    // 3. Castling availability
    let Some(castling_availability) = fen_parts.get(2) else {
        return Err(FenParserError::FenDataMissing {
            missing_part: CASTLING_AVAILABILITY.to_string(),
            fen_input: fen_input.to_string(),
        });
    };
    let (white_castle, black_castle) = string_to_castling_rights(castling_availability)?;

    // 4. En passant target square
    let Some(en_passant) = fen_parts.get(3) else {
        return Err(FenParserError::FenDataMissing {
            missing_part: EN_PASSANT_TARGET_SQUARE.to_string(),
            fen_input: fen_input.to_string(),
        });
    };
    let en_passant = get_en_passant_square(en_passant)?;

    // 5. Halfmove clock
    let Some(halfmove_clock) = fen_parts.get(4) else {
        return Err(FenParserError::FenDataMissing {
            missing_part: EN_PASSANT_TARGET_SQUARE.to_string(),
            fen_input: fen_input.to_string(),
        });
    };
    let halfmove_clock = match halfmove_clock.parse::<u16>() {
        Ok(value) => value,
        Err(_) => {
            return Err(FenParserError::UnrecognisedContent {
                input: halfmove_clock.to_string(),
                fen_part: HALFMOVE_CLOCK.to_string(),
            });
        }
    };

    // 6. Fullmove counter
    let Some(fullmove_counter) = fen_parts.get(5) else {
        return Err(FenParserError::FenDataMissing {
            missing_part: EN_PASSANT_TARGET_SQUARE.to_string(),
            fen_input: fen_input.to_string(),
        });
    };
    let fullmove_counter = match fullmove_counter.parse::<u16>() {
        Ok(value) => value,
        Err(_) => {
            return Err(FenParserError::UnrecognisedContent {
                input: halfmove_clock.to_string(),
                fen_part: FULLMOVE_NUMBER.to_string(),
            });
        }
    };

    Ok(Position {
        board_position,
        active_color,
        castling_white: white_castle,
        castling_black: black_castle,
        en_passant,
        halfmove_clock,
        fullmove_counter,
    })
}

/// Computes the piece placement data and adds the correct pieces to the board
/// - `piece_data` - The piece data of a fen string
/// - `board` - The board which gets filled with the `piece_data`
fn compute_piece_placement_data(
    piece_data: &str,
    board: &mut BoardSetup,
) -> Result<(), FenParserError> {
    // Split the different rows at '/'
    let rows: Vec<&str> = piece_data.split('/').collect();

    for row_counter in ((Board::ROW_1 as usize)..=(Board::ROW_8 as usize)).rev() {
        let mut piece_counter: usize = 0;
        let Some(current_row) = rows.get(row_counter) else {
            return Err(FenParserError::InvalidPiecePlacementData(format!(
                "Row {} is missing",
                row_counter + 1
            )));
        };
        for char_index in current_row.as_bytes() {
            if *char_index >= b'1' && *char_index <= b'8' {
                // Any amount of none pieces
                let empty_fields_amount = char_index - b'0';
                if piece_counter + empty_fields_amount as usize > COLUMN_AMOUNT {
                    return Err(FenParserError::InvalidPiecePlacementData(format!(
                        "Row {} contains more than {COLUMN_AMOUNT} fields",
                        row_counter + 1
                    )));
                }
                piece_counter += empty_fields_amount as usize;
            } else if let Some(piece) = Piece::import_piece(*char_index as char) {
                // Any real (not-none) piece
                if piece_counter > (COLUMN_AMOUNT - 1) {
                    return Err(FenParserError::InvalidPiecePlacementData(format!(
                        "Row {} contains more than {COLUMN_AMOUNT} fields",
                        row_counter + 1
                    )));
                }

                board[Board::COLUMN_H as usize - row_counter][piece_counter] = Some(piece);
                piece_counter += 1;
            } else {
                return Err(FenParserError::UnrecognisedContent {
                    input: (*char_index as char).to_string(),
                    fen_part: PIECE_PLACEMENT_DATA.to_string(),
                });
            }
        }
    }
    Ok(())
}

/// Converts the active color part of a fen string into a `PlayerColor`
/// - `color_data` - The player at turn part of a fen string
/// - `returns` - The player at turn or an error
fn string_to_active_color(color_data: &str) -> Result<PlayerColor, FenParserError> {
    match color_data {
        "w" => Ok(PlayerColor::White),
        "b" => Ok(PlayerColor::Black),
        _ => Err(FenParserError::UnrecognisedContent {
            input: color_data.to_string(),
            fen_part: ACTIVE_COLOR.to_string(),
        }),
    }
}

/// This function converts a string to a field
fn get_en_passant_square(field_data: &str) -> Result<Option<Field>, FenParserError> {
    // Check if no field is set
    if field_data == "-" {
        return Ok(None);
    }

    // Convert field
    match Field::new_from_string(field_data) {
        Some(field) => Ok(Some(field)),
        None => Err(FenParserError::UnrecognisedContent {
            input: field_data.to_string(),
            fen_part: EN_PASSANT_TARGET_SQUARE.to_string(),
        }),
    }
}

/// This functions converts a string into the rights to castle
/// - `castling_data` - The castling part of a fen string
/// - `returns` - The castling rights in the following order  
///     0: White castling rights
///     1: Black castling rights
fn string_to_castling_rights(
    castling_data: &str,
) -> Result<(CastlingRights, CastlingRights), FenParserError> {
    let letters = castling_data.as_bytes();

    let mut castling_black = CastlingRights {
        queenside: false,
        kingside: false,
    };

    let mut castling_white = CastlingRights {
        queenside: false,
        kingside: false,
    };

    for position in letters {
        match *position as char {
            'K' => castling_white.kingside = true,
            'Q' => castling_white.queenside = true,
            'k' => castling_black.kingside = true,
            'q' => castling_black.queenside = true,
            '-' => break,
            _ => {
                return Err(FenParserError::UnrecognisedContent {
                    input: (*position as char).to_string(),
                    fen_part: CASTLING_AVAILABILITY.to_string(),
                });
            }
        }
    }

    Ok((castling_white, castling_black))
}
