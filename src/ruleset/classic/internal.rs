use crate::{
    Field,
    Fields::*,
    GameResult, Piece, PieceType, PlayerColor, Position, Rows, Turn,
    chess::{
        castling_rights::CastlingRights,
        turn::{CastleDirection, NormalTurn},
    },
};

use super::util::{
    castle_data::*, move_iterators::get_movement_modifiers, move_legality::MoveLegality,
    piece_move_iterator::PieceMoveIterator,
};

/// Takes a turn which is a promotion turn and returns a vector of each possible resulting promotion turn.
/// - `turn` - The promotion turn
/// - `turns` - the vector of turns to wich turn should be pushed
fn create_promotion_turns(turn: NormalTurn) -> Vec<Turn> {
    let mut promotion_turns = Vec::<Turn>::with_capacity(4);
    let mut base_turn = turn;

    base_turn.promotion = Option::Some(PieceType::Rook);
    promotion_turns.push(Turn::Normal(base_turn));

    base_turn.promotion = Option::Some(PieceType::Queen);
    promotion_turns.push(Turn::Normal(base_turn));

    base_turn.promotion = Option::Some(PieceType::Bishop);
    promotion_turns.push(Turn::Normal(base_turn));

    base_turn.promotion = Option::Some(PieceType::Knight);
    promotion_turns.push(Turn::Normal(base_turn));

    promotion_turns
}

/// Returns the result of the game in the current position.
/// - `returns` - The game result in the current position
#[must_use]
pub(super) fn game_over_check(position: &Position) -> Option<GameResult> {
    // Check for insufficient material
    if !is_sufficient_material(position) {
        return Some(GameResult::Draw);
    }

    // Check all other rules
    if !get_possible_turns(position).is_empty() {
        if position.get_halfmove_clock() == 50 {
            return Some(GameResult::Draw);
        }
        return None;
    } else if is_in_check(position, position.get_active_color()) {
        return Some(GameResult::Decisive(position.get_active_color().reverse()));
    }
    Some(GameResult::Draw)
}

/// Calculates which turns are possible and returns all possible turns
pub(super) fn get_possible_turns(position: &Position) -> Vec<Turn> {
    let mut turns: Vec<Turn> = Vec::<Turn>::new();

    for field in BOARD_FIELDS {
        let Some(piece) = position.get_field_occupation(&field) else {
            continue;
        };

        if piece.get_color() != position.get_active_color() {
            continue;
        }

        // Check if current piece is a pawn
        let is_pawn = PieceType::Pawn == piece.get_type();

        let mut piece_iterator = PieceMoveIterator::new(get_movement_modifiers(&piece), field);

        loop {
            while let Some(mut turn) = piece_iterator.current() {
                // if turn is a promotion turn insert a dummy figure to make the move legal
                if is_pawn && matches!(turn.target.get_row(), Rows::ROW_8 | Rows::ROW_1) {
                    turn.promotion = Some(PieceType::Queen);
                }

                let full_turn = Turn::Normal(turn);
                match is_legal_move(position, full_turn, position.get_active_color(), true) {
                    MoveLegality::TemporarelyIllegal => continue,
                    MoveLegality::FullyIllegal => break,
                    MoveLegality::Legal => {
                        if is_pawn && matches!(turn.target.get_row(), Rows::ROW_8 | Rows::ROW_1) {
                            turns.append(&mut create_promotion_turns(turn));
                        } else {
                            turns.push(full_turn);
                        }
                    }
                    MoveLegality::LastLegal => {
                        if is_pawn && matches!(turn.target.get_row(), Rows::ROW_8 | Rows::ROW_1) {
                            turns.append(&mut create_promotion_turns(turn));
                        } else {
                            turns.push(full_turn);
                        }
                        break;
                    }
                }
            }
            if !piece_iterator.step() {
                break;
            }
        }
        continue;
    }

    // Check for castling moves
    let kingside_castle = Turn::Castle(CastleDirection::Kingside);
    let kingside_castle_legality =
        is_legal_move(position, kingside_castle, position.get_active_color(), true);
    if matches!(kingside_castle_legality, MoveLegality::Legal) {
        turns.push(kingside_castle);
    }

    let queenside_castle = Turn::Castle(CastleDirection::Queenside);
    let queenside_castle_legality = is_legal_move(
        position,
        queenside_castle,
        position.get_active_color(),
        true,
    );
    if matches!(queenside_castle_legality, MoveLegality::Legal) {
        turns.push(queenside_castle);
    }

    turns
}

/// Executes the given turn. This method does not check whether a turn is legal.
/// - `turn` - The turn which should be played
pub(super) fn internal_turn(original_position: &Position, turn: &Turn) -> Position {
    let mut position = original_position.clone();
    let active_color = position.get_active_color();

    match turn {
        Turn::Castle(castle_direction) => {
            let (king_ori, king_dest, rook_ori, rook_dest) = match castle_direction {
                CastleDirection::Kingside => match active_color {
                    PlayerColor::Black => (FIELD_E8, FIELD_G8, FIELD_H8, FIELD_F8),
                    PlayerColor::White => (FIELD_E1, FIELD_G1, FIELD_H1, FIELD_F1),
                },
                CastleDirection::Queenside => match active_color {
                    PlayerColor::Black => (FIELD_E8, FIELD_C8, FIELD_A8, FIELD_D8),
                    PlayerColor::White => (FIELD_E1, FIELD_C1, FIELD_A1, FIELD_D1),
                },
            };

            let king = position.get_field_occupation(&king_ori);
            position.set_field_occupation(&king_dest, king);
            position.set_field_occupation(&king_ori, None);
            let rook = position.get_field_occupation(&rook_ori);
            position.set_field_occupation(&rook_dest, rook);
            position.set_field_occupation(&rook_ori, None);

            position.set_castling_rights(active_color, CastlingRights::new(false, false));

            position.set_halfmove_clock(position.get_halfmove_clock() + 1);
            position.set_en_passant(None);
        }
        Turn::Normal(normal_turn) => {
            let from_field = position.get_field_occupation(&normal_turn.origin);
            let moving_piece = from_field.unwrap();
            let to_field = position.get_field_occupation(&normal_turn.target);

            // Increase move counter if no piece has been taken and no pawn has been moved
            if moving_piece.get_type() == PieceType::Pawn || to_field.is_some() {
                position.set_halfmove_clock(0);
            } else {
                position.set_halfmove_clock(position.get_halfmove_clock() + 1);
            }

            // Move the piece
            position.set_field_occupation(&normal_turn.target, Some(moving_piece));
            position.set_field_occupation(&normal_turn.origin, None);

            if PieceType::King == moving_piece.get_type() {
                position.set_castling_rights(active_color, CastlingRights::new(false, false));
            } else if PieceType::Rook == moving_piece.get_type() {
                // Remove castling rights if the rook moves
                match moving_piece.get_color() {
                    PlayerColor::Black => {
                        let mut castling_rights = position.get_castling_rights(active_color);
                        if normal_turn.origin == FIELD_A8 {
                            castling_rights.set(CastleDirection::Queenside, false);
                            position.set_castling_rights(active_color, castling_rights);
                        } else if normal_turn.origin == FIELD_H8 {
                            castling_rights.set(CastleDirection::Kingside, false);
                            position.set_castling_rights(active_color, castling_rights);
                        }
                    }
                    PlayerColor::White => {
                        let mut castling_rights = position.get_castling_rights(active_color);
                        if normal_turn.origin == FIELD_A1 {
                            castling_rights.set(CastleDirection::Queenside, false);
                            position.set_castling_rights(active_color, castling_rights);
                        } else if normal_turn.origin == FIELD_H1 {
                            castling_rights.set(CastleDirection::Kingside, false);
                            position.set_castling_rights(active_color, castling_rights);
                        }
                    }
                }
            } else if PieceType::Pawn == moving_piece.get_type()
                && matches!(normal_turn.target.get_row(), Rows::ROW_1 | Rows::ROW_8)
            {
                // Promote if possible
                position.set_field_occupation(
                    &normal_turn.target,
                    Some(Piece::new(normal_turn.promotion.unwrap(), active_color)),
                );
            }

            // Remove piece taken with en passant
            if let Some(field) = position.get_en_passant()
                && normal_turn.target.get_column() == field.get_column()
                && normal_turn.origin.get_row() == field.get_row()
            {
                position.set_field_occupation(&field, None);
                position.set_halfmove_clock(0);
            }

            // Update en passant field
            if PieceType::Pawn == moving_piece.get_type()
                && normal_turn
                    .origin
                    .get_row()
                    .abs_diff(normal_turn.target.get_row())
                    == 2
            {
                position.set_en_passant(Some(normal_turn.target));
            } else {
                position.set_en_passant(None);
            }
        }
    };

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
fn is_legal_move(
    position: &Position,
    turn: Turn,
    active_color: PlayerColor,
    check_for_check: bool,
) -> MoveLegality {
    let origin_field = match turn {
        Turn::Normal(normal_turn) => normal_turn.origin,
        Turn::Castle(_) => match active_color {
            PlayerColor::Black => FIELD_E8,
            PlayerColor::White => FIELD_E1,
        },
    };

    let Some(moving_piece) = position.get_field_occupation(&origin_field) else {
        return MoveLegality::FullyIllegal;
    };

    let active_color = moving_piece.get_color();

    // Check whether the move is a capture
    let mut is_capture = false;
    let mut legality_state = MoveLegality::Legal;

    match turn {
        Turn::Castle(direction) => {
            legality_state = is_castle_legal(position, direction, active_color)
        }
        Turn::Normal(normal_turn) => {
            let poss_target_occupation = position.get_field_occupation(&normal_turn.target);
            if let Some(target_occupation) = poss_target_occupation {
                if target_occupation.get_color() == active_color {
                    return MoveLegality::FullyIllegal;
                }
                is_capture = true;
            }
            if matches!(moving_piece.get_type(), PieceType::Pawn) {
                legality_state = is_pawn_move_legal(position, normal_turn, active_color, is_capture)
            }
        }
    }

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

/// Checks whether the fields used for castling fullfill the necessary conditions for a legal castle move.
/// - `position` - The current position
/// - `direction` - The direction in which to castle
/// - `active_color` - The currently active color
/// - `returns` - Whether or not the move is legal
fn is_castle_legal(
    position: &Position,
    direction: CastleDirection,
    active_color: PlayerColor,
) -> MoveLegality {
    let relevant_fields: (&[Field], [Field; 3]) = match active_color {
        PlayerColor::Black => match direction {
            CastleDirection::Kingside => (&CASTLE_BK_BLOCKED, CASTLE_BK_CHECKED),
            CastleDirection::Queenside => (&CASTLE_BQ_BLOCKED, CASTLE_BQ_CHECKED),
        },
        PlayerColor::White => match direction {
            CastleDirection::Kingside => (&CASTLE_WK_BLOCKED, CASTLE_WK_CHECKED),
            CastleDirection::Queenside => (&CASTLE_WQ_BLOCKED, CASTLE_WQ_CHECKED),
        },
    };

    if !position.get_castling_rights(active_color).get(direction) {
        return MoveLegality::FullyIllegal;
    }

    let (blocked_fields, checked_fields) = relevant_fields;
    if is_castle_illegal(position, blocked_fields, checked_fields, active_color) {
        return MoveLegality::FullyIllegal;
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
    turn: NormalTurn,
    active_color: PlayerColor,
    is_capture: bool,
) -> MoveLegality {
    // Forward moves
    if turn.origin.get_column() == turn.target.get_column() {
        if is_capture {
            return MoveLegality::FullyIllegal;
        }

        if turn.origin.get_row().abs_diff(turn.target.get_row()) == 2
            && match active_color {
                PlayerColor::Black => turn.origin.get_row() != Rows::ROW_7,
                PlayerColor::White => turn.origin.get_row() != Rows::ROW_2,
            }
        {
            return MoveLegality::FullyIllegal;
        }
    }

    // Check if a pawn can capture diagonally
    if turn.origin.get_column() != turn.target.get_column()
        && position.get_field_occupation(&turn.target).is_none()
    {
        let Some(field) = position.get_en_passant() else {
            return MoveLegality::FullyIllegal;
        };

        if turn.target.get_column() != field.get_column()
            || turn.origin.get_row() != field.get_row()
        {
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
    castling_fields_blocked(position, blocked_fields)
        || fields_under_attack(position, player_color, checked_fields)
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
pub(super) fn is_in_check(position: &Position, player_color: PlayerColor) -> bool {
    for field in BOARD_FIELDS {
        let occupation = position.get_field_occupation(&field);
        if let Some(piece) = occupation
            && piece.get_color() != player_color
        {
            let mut piece_iterator = PieceMoveIterator::new(get_movement_modifiers(&piece), field);

            loop {
                while let Some(turn) = piece_iterator.current() {
                    // Handling of the next loops
                    match is_legal_move(position, Turn::Normal(turn), player_color.reverse(), false)
                    {
                        MoveLegality::Legal => {
                            if position.get_field_occupation(&turn.target).is_none() {
                                continue;
                            }
                            break;
                        }
                        MoveLegality::LastLegal => {
                            if let Some(target_piece) = position.get_field_occupation(&turn.target)
                                && PieceType::King == target_piece.get_type()
                                && target_piece.get_color() == player_color
                            {
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
    false
}

/// Checks whether the given fields are under attack by the enemy color
/// - `player_color` - The color at turn
/// - `fields` - The fields to check for being attacked
/// - `returns` - Whether one of the fields is being attacked
fn fields_under_attack(position: &Position, player_color: PlayerColor, fields: [Field; 3]) -> bool {
    for field in BOARD_FIELDS {
        let occupation = position.get_field_occupation(&field);

        if let Some(piece) = occupation
            && piece.get_color() != player_color
        {
            let mut piece_iterator = PieceMoveIterator::new(get_movement_modifiers(&piece), field);

            loop {
                while let Some(turn) = piece_iterator.current() {
                    // Handling of the next loops
                    match is_legal_move(position, Turn::Normal(turn), player_color.reverse(), false)
                    {
                        MoveLegality::Legal => {
                            if fields.contains(&turn.target) {
                                return true;
                            }
                            continue;
                        }
                        MoveLegality::LastLegal => {
                            if fields.contains(&turn.target) {
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
        for piece in row.iter().flatten() {
            if let Some(value) = func(*piece) {
                return value;
            }
        }
    }
    false
}
