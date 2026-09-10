use crate::{
    Field,
    Fields::*,
    GameResult, Piece, PieceType, PlayerColor, Position, Rows, Turn,
    chess::{
        castling_rights::CastlingRights,
        turn::{CastleDirection, NormalTurn},
    },
    ruleset::util::{check_predicate_for_each_piece, for_each_piece},
};

use super::util::{
    castle_data::*, move_iterators::get_movement_modifiers, move_legality::MoveLegality,
    piece_move_iterator::PieceMoveIterator,
};

/// Takes a turn which is a promotion turn and returns a vector of each possible resulting promotion turn.
/// - `turn` - The promotion turn
fn create_promotion_turns(mut turn: NormalTurn) -> Vec<Turn> {
    let mut promotion_turns = Vec::<Turn>::with_capacity(4);
    let promotions_pieces = [
        PieceType::Rook,
        PieceType::Queen,
        PieceType::Bishop,
        PieceType::Knight,
    ];

    for piece in promotions_pieces {
        turn.promotion = Option::Some(piece);
        promotion_turns.push(Turn::Normal(turn));
    }

    promotion_turns
}

/// Returns the result of the game in the current position.
/// The following rules are checked:
/// 1. Checkmate
/// 2. Fifty-move rule
/// 3. Insufficient material
/// 4. Stalemate
/// - `position` - The current position of the game
/// - `returns` - The game result in the current position
#[must_use]
pub(super) fn game_over_check(position: &Position) -> Option<GameResult> {
    let has_possible_moves = !get_possible_turns(position).is_empty();
    let is_checked = is_in_check(position, position.get_active_color());

    // Check for checkmate
    if is_checked && !has_possible_moves {
        return Some(GameResult::Decisive(position.get_active_color().reverse()));
    }

    // Check movecounter for fifty-move rule
    if position.get_halfmove_clock() == 50 {
        return Some(GameResult::Draw);
    }

    // Check for insufficient material
    if !is_sufficient_material(position) {
        return Some(GameResult::Draw);
    }

    match has_possible_moves {
        true => None,
        false => Some(GameResult::Draw), // Stalemate,
    }
}

/// Generates a list of turns which are possible in the given positions and returns all possible turns.
/// - `position` - The current position of the game
/// - `returns` - A vector of all possible turns in the current position
pub(super) fn get_possible_turns(position: &Position) -> Vec<Turn> {
    let mut turns: Vec<Turn> = Vec::<Turn>::new();

    for_each_piece(position, |piece, field| {
        if piece.get_color() != position.get_active_color() {
            return;
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
    });

    // Check for castling moves
    let castle_turns = [
        Turn::Castle(CastleDirection::Kingside),
        Turn::Castle(CastleDirection::Queenside),
    ];
    for castle_turn in castle_turns {
        let legality = is_legal_move(position, castle_turn, position.get_active_color(), true);
        if matches!(legality, MoveLegality::Legal) {
            turns.push(castle_turn);
        }
    }

    turns
}

/// Executes the given turn. This method does not check whether a turn is legal.
/// - `original_position` - The position before the turn is executed
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

            move_piece(&mut position, &king_ori, &king_dest);
            move_piece(&mut position, &rook_ori, &rook_dest);

            position.set_castling_rights(active_color, CastlingRights::new(false, false));

            position.set_halfmove_clock(position.get_halfmove_clock() + 1);
            position.set_en_passant(None);
        }
        Turn::Normal(normal_turn) => {
            let moving_piece_type = position
                .get_field_occupation(&normal_turn.origin)
                .unwrap()
                .get_type();
            let target_occupation = position.get_field_occupation(&normal_turn.target);
            let mut is_capture = target_occupation.is_some();

            // Move the piece
            move_piece(&mut position, &normal_turn.origin, &normal_turn.target);

            match moving_piece_type {
                PieceType::Pawn => {
                    // Promote if possible
                    if matches!(normal_turn.target.get_row(), Rows::ROW_1 | Rows::ROW_8) {
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
                        is_capture = true;
                    }
                }
                PieceType::Rook => {
                    // Remove castling rights if the rook moves
                    let mut castling_rights = position.get_castling_rights(active_color);
                    match normal_turn.origin {
                        FIELD_A1 | FIELD_A8 => {
                            castling_rights.set(CastleDirection::Queenside, false)
                        }
                        FIELD_H1 | FIELD_H8 => {
                            castling_rights.set(CastleDirection::Kingside, false)
                        }
                        _ => {}
                    }
                    position.set_castling_rights(active_color, castling_rights);
                }
                PieceType::King => {
                    position.set_castling_rights(active_color, CastlingRights::new(false, false));
                }
                _ => {}
            };

            // Increase move counter if no piece has been taken and no pawn has been moved
            if moving_piece_type == PieceType::Pawn || is_capture {
                position.set_halfmove_clock(0);
            } else {
                position.set_halfmove_clock(position.get_halfmove_clock() + 1);
            }

            // Update en passant field
            if PieceType::Pawn == moving_piece_type
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

/// Moves a piece from the origin field to the destination field.
/// This function does not check whether the move is legal.
/// - `position` - The current position of the game
/// - `origin` - The field from which the piece is moved
/// - `destination` - The field to which the piece is moved
fn move_piece(position: &mut Position, origin: &Field, destination: &Field) {
    let moving_piece = position.get_field_occupation(origin);
    position.set_field_occupation(destination, moving_piece);
    position.set_field_occupation(origin, None);
}

/// Checks if the move which is specified by the two fields is a legal move
/// - `position` - The current position of the game
/// - `turn` - The turn which is checked
/// - `player_color` - The player that performs the turn
/// - `check_for_check` - Whether the function should check if the move would put the player in check
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
    let castle_data = CastleData::get_for_color(active_color);

    if !position.get_castling_rights(active_color).get(direction) {
        return MoveLegality::FullyIllegal;
    }

    let (blocked_fields, checked_fields) = match direction {
        CastleDirection::Queenside => {
            (castle_data.queenside_blocked, castle_data.queenside_checked)
        }
        CastleDirection::Kingside => (castle_data.kingside_blocked, castle_data.kingside_checked),
    };

    let are_fields_blocked = castling_fields_blocked(position, blocked_fields);
    let are_fields_attacked = fields_under_attack(position, active_color, checked_fields);
    if are_fields_blocked || are_fields_attacked {
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

/// Checks and returns if one of the given fields is blocked by another piece
/// - `position` - The current position
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
/// - `position` - The current position of the game
/// - `player_color` - The player to check for being checked
/// - `returns` - Whether the given player is currently checked
pub(super) fn is_in_check(position: &Position, player_color: PlayerColor) -> bool {
    check_predicate_for_each_piece(position, |piece, field| {
        if piece.get_color() == player_color {
            return None;
        }

        let mut piece_iterator = PieceMoveIterator::new(get_movement_modifiers(&piece), field);

        loop {
            while let Some(turn) = piece_iterator.current() {
                // Handling of the next loops
                match is_legal_move(position, Turn::Normal(turn), player_color.reverse(), false) {
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
                            return Some(true);
                        }
                        break;
                    }
                    MoveLegality::TemporarelyIllegal => continue,
                    MoveLegality::FullyIllegal => break,
                }
            }
            if !piece_iterator.step() {
                return None;
            }
        }
    })
}

/// Checks whether the given fields are under attack by the enemy color
/// - `position` - The current position of the game
/// - `player_color` - The color at turn
/// - `fields` - The fields to check for being attacked
/// - `returns` - Whether one of the fields is being attacked
fn fields_under_attack(position: &Position, player_color: PlayerColor, fields: &[Field]) -> bool {
    check_predicate_for_each_piece(position, |piece, field| {
        if piece.get_color() == player_color {
            return None;
        }

        let mut piece_iterator = PieceMoveIterator::new(get_movement_modifiers(&piece), field);

        loop {
            while let Some(turn) = piece_iterator.current() {
                // Handling of the next loops
                match is_legal_move(position, Turn::Normal(turn), player_color.reverse(), false) {
                    MoveLegality::Legal => {
                        if fields.contains(&turn.target) {
                            return Some(true);
                        }
                        continue;
                    }
                    MoveLegality::LastLegal => {
                        if fields.contains(&turn.target) {
                            return Some(true);
                        }
                        break;
                    }
                    MoveLegality::TemporarelyIllegal => continue,
                    MoveLegality::FullyIllegal => break,
                }
            }
            if !piece_iterator.step() {
                return None;
            }
        }
    })
}

/// Returns whether enough material is on the board to checkmate
/// - `position` - The current position of the game
/// - `returns` - Whether enough material is on the board to checkmate
fn is_sufficient_material(position: &Position) -> bool {
    let mut has_knight = false;
    let mut white_bishop = false;
    let mut black_bishop = false;

    check_predicate_for_each_piece(position, |piece: Piece, _| -> Option<bool> {
        match piece.get_type() {
            PieceType::Pawn | PieceType::Rook | PieceType::Queen => return Some(true),
            PieceType::Bishop => match piece.get_color() {
                PlayerColor::Black => {
                    if black_bishop || has_knight {
                        return Some(true);
                    }
                    black_bishop = true;
                }
                PlayerColor::White => {
                    if white_bishop || has_knight {
                        return Some(true);
                    }
                    white_bishop = true;
                }
            },
            PieceType::Knight => {
                if black_bishop || white_bishop || has_knight {
                    return Some(true);
                }
                has_knight = true;
            }
            _ => {}
        }
        None
    })
}
