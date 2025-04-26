use crate::{constants::BOARD_FIELDS, piece::piece_move_iterator::PieceMoveIterator, position::util::{castling_rights::CastlingRights, move_legality::MoveLegality}, util::castle_data::{CASTLE_BK_BLOCKED, CASTLE_BK_CHECKED, CASTLE_BQ_BLOCKED, CASTLE_BQ_CHECKED, CASTLE_WK_BLOCKED, CASTLE_WK_CHECKED, CASTLE_WQ_BLOCKED, CASTLE_WQ_CHECKED}, Board::{self, *}, Field, Piece, PieceType, PlayerColor, Position, Turn};

/// Takes a turn which is a promotion turn and returns a vector of each possible resulting promotion turn.
/// - `turn` - The promotion turn
/// - `turns` - the vector of turns to wich turn should be pushed
fn push_turn(turn: Turn) -> Vec<Turn> {
    let mut promotion_turns = Vec::<Turn>::with_capacity(4);
    let mut base_turn = turn;

    base_turn.promotion = Option::Some(PieceType::Rook);
    promotion_turns.push(base_turn);

    base_turn.promotion = Option::Some(PieceType::Queen);
    promotion_turns.push(base_turn);

    base_turn.promotion = Option::Some(PieceType::Bishop);
    promotion_turns.push(base_turn);

    base_turn.promotion = Option::Some(PieceType::Knight);
    promotion_turns.push(base_turn);

    promotion_turns
}

/// Executes the given turn. This method does not check whether a turn is legal.
/// - `action` - The turn which should be played
/// # Panics
/// This panic indicates an error in the library.
pub(super) fn internal_turn(original_position: &Position, action: &Turn) -> Position {
    let position = original_position.clone();
    let from_field = position.get_field_occupation(&action.current);
    let moving_piece = from_field.unwrap();
    let to_field = position.get_field_occupation(&action.target);
    let active_color = position.get_active_color();

    // Increase move counter if no piece has been taken and no pawn has been moved
    if moving_piece.get_type() == PieceType::Pawn || to_field.is_some() {
        position.set_halfmove_counter(0);
    } else {
        position.set_halfmove_counter(position.get_halfmove_counter() + 1);
    }

    // Move the piece
    position.set_field_occupation(&action.target, Some(moving_piece));
    position.set_field_occupation(&action.current, None);

    if PieceType::King == moving_piece.get_type() {
        match moving_piece.get_color() {
            PlayerColor::Black => {
                if action.current == FIELD_E8 {
                    if action.target == FIELD_C8 {
                        position.set_field_occupation(
                            &FIELD_D8,
                            position.get_field_occupation(&FIELD_A8),
                        );
                        position.set_field_occupation(&FIELD_A8, None);
                    } else if action.target == FIELD_G8 {
                        position.set_field_occupation(
                            &FIELD_F8,
                            position.get_field_occupation(&FIELD_H8),
                        );
                        position.set_field_occupation(&FIELD_H8, None);
                    }
                }
                position.set_castling_rights(
                    PlayerColor::Black,
                    CastlingRights {queenside: false, kingside: false},
                );
            }
            PlayerColor::White => {
                if action.current == FIELD_E1 {
                    if action.target == FIELD_C1 {
                        position.set_field_occupation(
                            &FIELD_D1,
                            position.get_field_occupation(&FIELD_A1),
                        );
                        position.set_field_occupation(&FIELD_A1, None);
                    } else if action.target == FIELD_G1 {
                        position.set_field_occupation(
                            &FIELD_F1,
                            position.get_field_occupation(&FIELD_H1),
                        );
                        position.set_field_occupation(&FIELD_H1, None);
                    }
                }
                position.set_castling_rights(
                    PlayerColor::White,
                    CastlingRights {queenside: false, kingside: false},
                );
            }
        }
    }

    // Remove castling rights if the rook moves
    if PieceType::Rook == moving_piece.get_type() {
        match moving_piece.get_color() {
            PlayerColor::Black => {
                if action.current == FIELD_A8 {
                    self.castling_black.queenside = false;
                } else if action.current == FIELD_H8 {
                    self.castling_black.kingside = false;
                }
            }
            PlayerColor::White => {
                if action.current == FIELD_A1 {
                    self.castling_white.queenside = false;
                } else if action.current == FIELD_H1 {
                    self.castling_white.kingside = false;
                }
            }
        }
    }

    // Promote if possible
    if PieceType::Pawn == moving_piece.get_type()
        && matches!(action.target.row, Board::ROW_1 | Board::ROW_8)
    {
        position.set_field_occupation(
            &action.target,
            Some(Piece::new(action.promotion.unwrap(), active_color)),
        );
    }

    // Remove piece taken with en passant
    if let Some(field) = position.get_en_passant() {
        if action.target.column == field.column && action.current.row == field.row {
            position.set_field_occupation(&field, None);
            position.set_halfmove_counter(0);
        }
    }

    // Update en passant field
    if PieceType::Pawn == moving_piece.get_type()
        && action.current.row.abs_diff(action.target.row) == 2
    {
        position.set_en_passant(Some(action.target));
    } else {
        position.set_en_passant(None);
    }

    // Raise fullmove counter
    if active_color == PlayerColor::Black {
        position.set_fullmove_counter(position.get_fullmove_counter() + 1);
    }

    // Change color at turn
    position.set_active_color(active_color.reverse());
    position
}

/// Checks if the move which is specified by the two fields is a legal move
/// - `turn` - The turn which is checked
/// - `player_color` - The player that performs the turn
/// - `returns` - Returns whether the turn is legal
fn is_legal_move(position: &Position, turn: Turn, check_for_check: bool) -> MoveLegality {
    let Some(moving_piece) = position.get_field_occupation(&turn.current) else {
        return MoveLegality::FullyIllegal;
    };

    let active_color = moving_piece.get_color();

    // Check if move is capture and whether it captures an enemy piece
    let is_capture = match position.get_field_occupation(&turn.target) {
        Some(piece) => {
            if piece.get_color() == active_color {
                return MoveLegality::FullyIllegal;
            }

            true
        }
        None => false,
    };

    let legality_state = match moving_piece.get_type() {
        PieceType::Pawn => is_pawn_move_legal(position, turn, active_color, is_capture),
        PieceType::King => is_king_move_legal(position, turn, active_color),
        _ => MoveLegality::Legal,
    };

    if matches!(
        legality_state,
        MoveLegality::FullyIllegal | MoveLegality::TemporarelyIllegal
    ) {
        return legality_state;
    }

    // Play move and check if a king is checked
    if check_for_check {
        let resulting_position: Position = internal_turn(position, &turn);
        if is_in_check(&resulting_position, active_color) {
            return MoveLegality::TemporarelyIllegal;
        }
    }

    if is_capture {
        return MoveLegality::LastLegal;
    }
    MoveLegality::Legal
}

/// Checks whether a given king move is legal.
/// - `turn` - The turn played by the king
/// - `active_color` - The currently active color
/// - `returns` - Whether or not the move is legal
fn is_king_move_legal(position: &Position, turn: Turn, active_color: PlayerColor) -> MoveLegality {
    let step = turn.target.column as i8 - turn.current.column as i8;
    if step == 2 {
        if turn.current.row == Board::ROW_8
            && active_color == PlayerColor::Black
            && position.get_castling_rights(PlayerColor::Black).kingside
        {
            if is_castle_illegal(position, &CASTLE_BK_BLOCKED, CASTLE_BK_CHECKED, active_color) {
                return MoveLegality::FullyIllegal;
            }
        } else if turn.current.row == Board::ROW_1
            && active_color == PlayerColor::White
            && position.get_castling_rights(PlayerColor::White).kingside
        {
            if is_castle_illegal(position, &CASTLE_WK_BLOCKED, CASTLE_WK_CHECKED, active_color) {
                return MoveLegality::FullyIllegal;
            }
        } else {
            return MoveLegality::FullyIllegal;
        }
    } else if step == -2 {
        if turn.current.row == Board::ROW_8
            && active_color == PlayerColor::Black
            && position.get_castling_rights(PlayerColor::Black).queenside
        {
            if is_castle_illegal(position, &CASTLE_BQ_BLOCKED, CASTLE_BQ_CHECKED, active_color) {
                return MoveLegality::FullyIllegal;
            }
        } else if turn.current.row == Board::ROW_1
            && active_color == PlayerColor::White
            && position.get_castling_rights(PlayerColor::White).queenside
        {
            if is_castle_illegal(position, &CASTLE_WQ_BLOCKED, CASTLE_WQ_CHECKED, active_color) {
                return MoveLegality::FullyIllegal;
            }
        } else {
            return MoveLegality::FullyIllegal;
        }
    }
    MoveLegality::Legal
}

/// Checks whether a given pawn move is legal or illegal.
/// - `turn` - The turn which gets checked
/// - `active_color` - The color playing the move
/// - `is_capture` - Whether or not the move is a capture move
/// - `returns` - Whether the move is deemed to be legal or not
fn is_pawn_move_legal(
    position: &Position,
    turn: Turn,
    active_color: PlayerColor,
    is_capture: bool,
) -> MoveLegality {
    // Forward moves
    if turn.current.column == turn.target.column {
        if is_capture {
            return MoveLegality::FullyIllegal;
        }

        if turn.current.row.abs_diff(turn.target.row) == 2
            && match active_color {
                PlayerColor::Black => turn.current.row != Board::ROW_7,
                PlayerColor::White => turn.current.row != Board::ROW_2,
            }
        {
            return MoveLegality::FullyIllegal;
        }
    }

    // Check if a pawn can capture diagonally
    if turn.current.column != turn.target.column
        && position.get_field_occupation(&turn.target).is_none()
    {
        let Some(field) = position.get_en_passant() else {
            return MoveLegality::FullyIllegal;
        };

        if turn.target.column != field.column || turn.current.row != field.row {
            return MoveLegality::FullyIllegal;
        }
    }
    MoveLegality::Legal
}

/// Runs the check if castling is allowed in the current position
/// - `blocked_fields` - An array of fields which must not be blocked
/// - `checked_fields` - An array of fields which must not be under attack
/// - `player_color` - The player to check castling rights for
/// - `returns` - Whether the player is allowed to castle
fn is_castle_illegal(
    position: &Position,
    blocked_fields: &[Field],
    checked_fields: [Field; 3],
    player_color: PlayerColor,
) -> bool {
    castling_fields_blocked(position, blocked_fields) || fields_under_attack(position, player_color, checked_fields)
}

/// Checks and returns if one of the given fields is blocked by another piece
/// - `fields` - The fields to check for blockades
/// - `returns` - Whether one of the fields is blocked
fn castling_fields_blocked(position: &Position, fields: &[Field]) -> bool {
    for field in fields {
        if position.get_field_occupation(field).is_some() {
            return true;
        }
    }
    false
}

/// Returns if the given player is currently checked
/// - `player_color` - The player to check for being checked
/// - `returns` - Whether the given player is currently checked
fn is_in_check(position: &Position, player_color: PlayerColor) -> bool {
    for field in BOARD_FIELDS {
        let occupation = position.get_field_occupation(&field);
        if let Some(piece) = occupation {
            if piece.get_color() != player_color {
                let mut piece_iterator =
                    PieceMoveIterator::new(piece.movement_modifiers(), field);

                loop {
                    while let Some(turn) = piece_iterator.current() {
                        // Handling of the next loops
                        match is_legal_move(position, turn, false) {
                            MoveLegality::Legal => {
                                if position.get_field_occupation(&turn.target).is_none() {
                                    continue;
                                }
                                break;
                            }
                            MoveLegality::LastLegal => {
                                if let Some(target_piece) =
                                    position.get_field_occupation(&turn.target)
                                {
                                    if PieceType::King == target_piece.get_type() {
                                        return true;
                                    }
                                }
                                break;
                            }
                            MoveLegality::TemporarelyIllegal => continue,
                            MoveLegality::FullyIllegal => break,
                        }
                    }
                    if !piece_iterator.step() {
                        break;
                    }
                }
            }
        }
    }
    false
}

/// Checks whether the given fields are under attack by the enemie color
/// - `player_color` - The color at turn
/// - `fields` - The fields to check for being attacked
/// - `returns` - Whether one of the fields is being attacked
fn fields_under_attack(
    position: &Position,
    player_color: PlayerColor,
    fields: [Field; 3],
) -> bool {
    for field in BOARD_FIELDS {
        let occupation = position.get_field_occupation(&field);

        if let Some(piece) = occupation {
            if piece.get_color() != player_color {
                let mut piece_iterator =
                    PieceMoveIterator::new(piece.movement_modifiers(), field);

                loop {
                    while let Some(turn) = piece_iterator.current() {
                        // Dont check castling options
                        if PieceType::King == piece.get_type()
                            && turn.current.column.abs_diff(turn.target.column) == 2
                        {
                            continue;
                        }

                        // Handling of the next loops
                        match is_legal_move(position, turn, false) {
                            MoveLegality::Legal => {
                                if fields.contains(&field) {
                                    return true;
                                }
                                continue;
                            }
                            MoveLegality::LastLegal => {
                                if fields.contains(&field) {
                                    return true;
                                }
                                break;
                            }
                            MoveLegality::TemporarelyIllegal => continue,
                            MoveLegality::FullyIllegal => break,
                        }
                    }
                    if !piece_iterator.step() {
                        break;
                    }
                }
            }
        }
    }
    false
}

/// Returns whether enough material is on the board to checkmate
/// - `returns` - Whether enough material is on the board to checkmate
fn is_sufficient_material(position: &Position) -> bool {
    let mut white_bishop: bool = false;
    let mut white_knight: bool = false;
    let mut black_bishop: bool = false;
    let mut black_knight: bool = false;

    let material_handling = |piece: Piece| -> Option<bool> {
        match piece.get_type() {
            PieceType::Pawn | PieceType::Rook | PieceType::Queen => return Some(true),
            PieceType::Bishop => match piece.get_color() {
                PlayerColor::Black => {
                    if black_bishop || black_knight || white_knight {
                        return Some(true);
                    }
                    black_bishop = true;
                }
                PlayerColor::White => {
                    if white_bishop || white_knight || black_knight {
                        return Some(true);
                    }
                    white_bishop = true;
                }
            },
            PieceType::Knight => {
                if black_bishop || black_knight || white_bishop || white_knight {
                    return Some(true);
                }
                match piece.get_color() {
                    PlayerColor::Black => black_knight = true,
                    PlayerColor::White => white_knight = true,
                }
            }
            _ => {}
        }
        None
    };

    iterate_board_by_piece(position, material_handling)
}

/// This function iterates over every field of the position and calls the given function for each field
/// - `func` - The function which is called for every field of the board
/// - `returns` - The value returned by the given func
fn iterate_board_by_piece<ClosureType>(position: &Position, mut func: ClosureType) -> bool
where
    ClosureType: FnMut(Piece) -> Option<bool>,
{
    for row in position.get_board_position() {
        for piece in row.into_iter().flatten() {
            if let Some(value) = func(*piece) {
                return value;
            }
        }
    }
    false
}