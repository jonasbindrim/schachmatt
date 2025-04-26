use crate::{
    position::{position_struct::BoardSetup, util::castling_rights::CastlingRights}, Columns, PlayerColor, Position, Rows
};

/// Converts a `Position` into a string in FEN notation.
/// - `position` - The position that gets converted
/// - `returns` - A fen string representing the given position
#[must_use]
pub fn export_to_fen(position: &Position) -> String {
    // 1. Piece placement data
    let piece_placement_data = export_piece_placement_data(&position.get_board_position());

    // 2. Active Color
    let active_color = match position.get_active_color() {
        PlayerColor::Black => 'b',
        PlayerColor::White => 'w',
    };

    // 3. Castling availability
    let castling_availability = export_castling_data(
        position.get_castling_rights(PlayerColor::White),
        position.get_castling_rights(PlayerColor::Black),
    );

    // 4. En-Passant
    let en_passant: String = match position.get_en_passant() {
        Some(value) => value.to_string(),
        None => String::from("-"),
    };

    // 5. Halfmove clock
    let halfmove_clock = position.get_halfmove_counter();

    // 6. Fullmove counter
    let fullmove_counter = &position.get_fullmove_counter().to_string();

    format!(
        "{} {} {} {} {} {}",
        piece_placement_data,
        active_color,
        castling_availability,
        en_passant,
        halfmove_clock,
        fullmove_counter
    )
}

/// Converts the board position of a game into a fen compatible position string
/// - `board_position` - A two-dimensional array describing the current board
/// - `returns` - The board position part of a fen
fn export_piece_placement_data(board_position: &BoardSetup) -> String {
    let mut piece_data: String = String::new();

    for row in ((Rows::ROW_1 as usize)..=(Rows::ROW_8 as usize)).rev() {
        let mut empty_counter = 0;
        for column in (Columns::COLUMN_A as usize)..=(Columns::COLUMN_H as usize) {
            let Some(piece) = board_position[row][column] else {
                empty_counter += 1;
                continue;
            };

            if empty_counter != 0 {
                piece_data.push_str(&empty_counter.to_string());
                empty_counter = 0;
            }
            piece_data.push(piece.export_piece());
        }
        if empty_counter != 0 {
            piece_data.push_str(&empty_counter.to_string());
        }
        if row != Rows::ROW_1 as usize {
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

    if white_castling.get_kingside() {
        castling_data.push('K');
    }

    if white_castling.get_queenside() {
        castling_data.push('Q');
    }

    if black_castling.get_kingside() {
        castling_data.push('k');
    }

    if black_castling.get_queenside() {
        castling_data.push('q');
    }

    if castling_data.is_empty() {
        return String::from("-");
    }

    castling_data
}
