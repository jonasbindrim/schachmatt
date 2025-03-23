use crate::{
    position::{position_struct::BoardSetup, util::castling_rights::CastlingRights}, util::error::{error_messages::FEN_IMPORT_ERROR, parser_error::ParserError}, Field, Piece, PlayerColor, Position
};

pub(crate) const DEFAULT_BOARD_SETUP: &str =
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// Converts a string in FEN notation into a `Position`.
/// which is described in the fen notation.
/// - `fen_notation` - The fen string to be converted
/// - `returns` - The position described in the fen string or an error
/// # Errors
/// Returns a `ParserError` when the string cannot be converted into `Position`.
/// # Panics
/// This panic indicates an error in the library.
pub fn import_from_fen(fen_notation: &str) -> Result<Position, ParserError> {
    // Split fen string and check if amount of blocks is correct
    let fen_parts: Vec<&str> = fen_notation.split_ascii_whitespace().collect();

    if fen_parts.len() != 6 {
        return Err(ParserError::new(FEN_IMPORT_ERROR));
    }

    // 1. Board position
    let mut board_position = [[None; 8]; 8];
    if let Some(error) = string_to_piece_data(fen_parts.first().unwrap(), &mut board_position) {
        return Err(error);
    }

    // 2. Active color
    let active_color: PlayerColor = string_to_active_color(fen_parts.get(1).unwrap())?;

    // 3. Castling availability
    let (black_kingside, black_queenside, white_kingside, white_queenside) =
        string_to_castling_rights(fen_parts.get(2).unwrap())?;

    // 4. En passant target square
    let en_passant = string_to_field(fen_parts.get(3).unwrap())?;

    // 5. Halfmove clock
    let halfmove_clock = string_to_u16(fen_parts.get(4).unwrap())?;

    // 6. Fullmove number
    let fullmove_counter = string_to_u16(fen_parts.get(5).unwrap())?;

    Ok(Position {
        board_position,
        active_color,
        castling_white: CastlingRights {
            queenside: white_queenside,
            kingside: white_kingside,
        },
        castling_black: CastlingRights {
            queenside: black_queenside,
            kingside: black_kingside,
        },
        en_passant,
        halfmove_clock,
        fullmove_counter,
    })
}

/// Converts the string part the fen notation into a two dimensional array of pieces
/// - `piece_data` - The piece data of a fen string
/// - `board` - The board which gets filled with the `piece_data`
/// - `returns` - An error if the conversion fails
fn string_to_piece_data(
    piece_data: &str,
    board: &mut BoardSetup,
) -> Option<ParserError> {
    // Split the different rows at '/'
    let rows: Vec<&str> = piece_data.split('/').collect();

    // Return error if data doesnt contain exactly 8 rows
    if rows.len() != 8 {
        return Some(ParserError::new(FEN_IMPORT_ERROR));
    }

    for row_counter in (0..8).rev() {
        let mut piece_counter: usize = 0;
        for char_index in rows[row_counter].as_bytes() {
            if *char_index >= b'1' && *char_index <= b'8' {
                // Any amount of none pieces
                let empty_fields_amount = char_index - b'0';
                if piece_counter + empty_fields_amount as usize > 8 {
                    return Some(ParserError::new(FEN_IMPORT_ERROR));
                }
                piece_counter += empty_fields_amount as usize;
            } else if let Some(piece) = Piece::import_piece(*char_index as char) {
                // Any real (not-none) piece
                if piece_counter > 7 {
                    return Some(ParserError::new(FEN_IMPORT_ERROR));
                }

                board[7 - row_counter][piece_counter] = Some(piece);
                piece_counter += 1;
            } else {
                return Some(ParserError::new(FEN_IMPORT_ERROR));
            }
        }
    }
    None
}

/// Converts the active color part of a fen string into a `PlayerColor`
/// - `color_data` - The player at turn part of a fen string
/// - `returns` - The player at turn or an error
fn string_to_active_color(color_data: &str) -> Result<PlayerColor, ParserError> {
    match color_data {
        "w" => Ok(PlayerColor::White),
        "b" => Ok(PlayerColor::Black),
        _ => Err(ParserError::new(FEN_IMPORT_ERROR)),
    }
}

/// This function converts a string into a u16 if possible or returns an error
/// - `number_data` - Number as string
/// - `returns` - Number as u16 or an error
fn string_to_u16(number_data: &str) -> Result<u16, ParserError> {
    match number_data.parse::<u16>() {
        Ok(value) => Ok(value),
        Err(_) => Err(ParserError::new(FEN_IMPORT_ERROR)),
    }
}

/// This function converts a string to a field
fn string_to_field(field_data: &str) -> Result<Option<Field>, ParserError> {
    // Check if no field is set
    if field_data.len() == 1 {
        match field_data {
            "-" => return Ok(None),
            _ => return Err(ParserError::new(FEN_IMPORT_ERROR)),
        }
    }

    // Convert field
    match Field::from_string(field_data) {
        Some(field) => Ok(Some(field)),
        None => Err(ParserError::new(FEN_IMPORT_ERROR)),
    }
}

/// This functions converts a string into the rights to castle
/// - `castling_data` - The castling part of a fen string
/// - `returns` - The castling rights in the following order  
///     0: Black kingside castling rights  
///     1: Black queenside castling rights  
///     2: White kingside castling rights  
///     3: White queenside castling rights
fn string_to_castling_rights(castling_data: &str) -> Result<(bool, bool, bool, bool), ParserError> {
    let letters = castling_data.as_bytes();
    if letters.len() > 4 {
        return Err(ParserError::new(FEN_IMPORT_ERROR));
    }

    let mut castling_black_kingside = false;
    let mut castling_black_queenside = false;
    let mut castling_white_kingside = false;
    let mut castling_white_queenside = false;

    for position in letters {
        match *position as char {
            'K' => castling_white_kingside = true,
            'Q' => castling_white_queenside = true,
            'k' => castling_black_kingside = true,
            'q' => castling_black_queenside = true,
            '-' => break,
            _ => return Err(ParserError::new(FEN_IMPORT_ERROR)),
        }
    }

    Ok((
        castling_black_kingside,
        castling_black_queenside,
        castling_white_kingside,
        castling_white_queenside,
    ))
}
