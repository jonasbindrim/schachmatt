use crate::{position::util::castling_rights::CastlingRights, Piece, PlayerColor, Position};

/// Converts a `Position` into a string in FEN notation.
/// - `position` - The position that gets converted
/// - `returns` - A fen string representing the given position
#[must_use]
pub fn export_to_fen(position: &Position) -> String {
    let mut fen_string = String::new();

    // 1. Piece placement data
    fen_string.push_str(&export_piece_placement_data(&position.board_position));
    fen_string.push(' ');

    // 2. Active Color
    match position.active_color {
        PlayerColor::Black => fen_string.push('b'),
        PlayerColor::White => fen_string.push('w'),
    }
    fen_string.push(' ');

    // 3. Castling availability
    fen_string.push_str(&export_castling_data(
        position.castling_white,
        position.castling_black,
    ));
    fen_string.push(' ');

    // 4. En-Passant
    match position.en_passant {
        Some(value) => fen_string.push_str(&value.to_string()),
        None => fen_string.push('-'),
    }
    fen_string.push(' ');

    // 5. Halfmove clock
    fen_string.push_str(&position.halfmove_clock.to_string());
    fen_string.push(' ');

    // 6. Fullmove counter
    fen_string.push_str(&position.fullmove_counter.to_string());

    fen_string
}

/// Converts the board position of a game into a fen compatible position string
/// - `board_position` - A two-dimensional array describing the current board
/// - `returns` - The board position part of a fen
fn export_piece_placement_data(board_position: &[[Piece; 8]; 8]) -> String {
    let mut piece_data: String = String::new();

    for row in (0..8).rev() {
        let mut empty_counter = 0;
        for column in 0..8 {
            match board_position[row][column].export_piece() {
                Some(letter) => {
                    if empty_counter != 0 {
                        piece_data.push_str(&empty_counter.to_string());
                        empty_counter = 0;
                    }
                    piece_data.push(letter);
                }
                None => empty_counter += 1,
            }
        }
        if empty_counter != 0 {
            piece_data.push_str(&empty_counter.to_string());
        }
        if row != 0 {
            piece_data.push('/');
        }
    }
    piece_data
}

/// Converts the castling data of a game into a fen compatible position string
/// - `white_castling` - Whether white is allowed to castle
/// - `black_castling` - Whether black is allowed to castle
/// - `returns` - The castling fen string part
fn export_castling_data(white_castling: CastlingRights, black_castling: CastlingRights) -> String {
    let mut castling_data: String = String::new();
    let mut counter = 0;
    if white_castling.kingside {
        castling_data.push('K');
        counter += 1;
    }
    if white_castling.queenside {
        castling_data.push('Q');
        counter += 1;
    }
    if black_castling.kingside {
        castling_data.push('k');
        counter += 1;
    }
    if black_castling.queenside {
        castling_data.push('q');
        counter += 1;
    }
    if counter == 0 {
        castling_data.push('-');
    }
    castling_data
}
