use crate::{
    Board, PlayerColor, Position,
    position::{position_struct::BoardSetup, util::castling_rights::CastlingRights},
};

/// Converts a `Position` into a string in FEN notation.
/// - `position` - The position that gets converted
/// - `returns` - A fen string representing the given position
#[must_use]
pub fn export_to_fen(position: &Position) -> String {
    // 1. Piece placement data
    let piece_placement_data = export_piece_placement_data(&position.board_position);

    // 2. Active Color
    let active_color = match position.active_color {
        PlayerColor::Black => 'b',
        PlayerColor::White => 'w',
    };

    // 3. Castling availability
    let castling_availability =
        export_castling_data(position.castling_white, position.castling_black);

    // 4. En-Passant
    let en_passant: String = match position.en_passant {
        Some(value) => value.to_string(),
        None => String::from("-"),
    };

    // 5. Halfmove clock
    let halfmove_clock = position.halfmove_clock;

    // 6. Fullmove counter
    let fullmove_counter = &position.fullmove_counter.to_string();

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

    for row in ((Board::ROW_1 as usize)..=(Board::ROW_8 as usize)).rev() {
        let mut empty_counter = 0;
        for column in (Board::COLUMN_A as usize)..=(Board::COLUMN_H as usize) {
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
        if row != Board::ROW_1 as usize {
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

    if white_castling.kingside {
        castling_data.push('K');
    }

    if white_castling.queenside {
        castling_data.push('Q');
    }

    if black_castling.kingside {
        castling_data.push('k');
    }

    if black_castling.queenside {
        castling_data.push('q');
    }

    if castling_data.is_empty() {
        return String::from("-");
    }

    castling_data
}
