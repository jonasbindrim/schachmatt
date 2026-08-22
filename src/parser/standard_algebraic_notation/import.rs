use crate::{
    CLASSIC_RULESET, PieceType, Position, Ruleset, Turn,
    chess::turn::{CastleDirection, NormalTurn},
    parser::standard_algebraic_notation::san_turn::{
        SanTurn, san_castle_direction::SanCastleDirection, san_origin_field::SanOriginField,
        san_pawn_move::SanPawnMove, san_piece_move::SanPieceMove,
    },
};

use super::{San, SanParserError};

impl San {
    /// Imports a string containing a chess move in `SAN` into a `Turn`.
    /// This function assumes that the classical chess ruleset is used.
    /// If that is not the case, refer to the `import_by_ruleset` function.
    /// - `raw` The string containing the chess move in `SAN`.
    /// - `position` The chess position the chess move was played in.
    /// - `returns` A `Turn` or a `SanParserError`. An error will be returned if either the `raw` argument does not contain a valid `SAN` move or if the move is illegal in the given `position`.
    pub fn import(raw: &str, position: &Position) -> Result<Turn, SanParserError> {
        Self::import_by_ruleset(raw, position, &CLASSIC_RULESET)
    }

    /// Imports a string containing a chess move in `SAN` into a `Turn`.
    /// - `raw` The string containing the chess move in `SAN`.
    /// - `position` The chess position the chess move was played in.
    /// - `ruleset` The chess ruleset used in the game the move was played in.
    /// - `returns` A `Turn` or a `SanParserError`. An error will be returned if either the `raw` argument does not contain a valid `SAN` move or if the move is illegal in the given `position`.
    pub fn import_by_ruleset(
        raw: &str,
        position: &Position,
        ruleset: &Ruleset,
    ) -> Result<Turn, SanParserError> {
        let san_turn: SanTurn = SanTurn::import(raw)?;

        let turn = match san_turn {
            SanTurn::Castling(castle) => Self::handle_castling(&castle, position, ruleset),
            SanTurn::PawnMove(pawn_move) => Self::handle_pawn_move(&pawn_move, position, ruleset),
            SanTurn::PieceMove(piece_move) => {
                Self::handle_piece_move(&piece_move, position, ruleset)
            }
        };

        if let Some(turn) = turn {
            return Ok(turn);
        }

        Err(SanParserError::InvalidMove(raw.to_string()))
    }

    /// Tries to convert a `SanPieceMove` into an actual `Turn` object that is playable in the given `position`.
    /// - `piece_move` The internal representation of a move in `SAN`.
    /// - `position` The chess position the chess move was played in.
    /// - `ruleset` The chess ruleset used in the game the move was played in.
    fn handle_piece_move(
        piece_move: &SanPieceMove,
        position: &Position,
        ruleset: &Ruleset,
    ) -> Option<Turn> {
        let possible_moves = ruleset.get_possible_turns(position);
        let possible_moves: Vec<NormalTurn> = possible_moves
            .into_iter()
            .filter_map(|turn| match turn {
                Turn::Normal(normal_turn) => Some(normal_turn),
                Turn::Castle(_) => None,
            })
            .collect();

        possible_moves
            .into_iter()
            .find(|&turn| {
                if piece_move.target_field != turn.target {
                    return false;
                }

                if position
                    .get_field_occupation(&turn.origin)
                    .unwrap()
                    .get_type()
                    != piece_move.piece_type
                {
                    return false;
                }

                let Some(origin) = piece_move.origin_field.as_ref() else {
                    return true;
                };

                match origin {
                    SanOriginField::Field(origin) => turn.origin == *origin,
                    SanOriginField::Column(column) => turn.origin.get_column() == *column,
                    SanOriginField::Row(row) => turn.origin.get_row() == *row,
                }
            })
            .map(Turn::Normal)
    }

    /// Tries to convert a `SanCastleDirection` into an actual `Turn` object that is playable in the given `position`.
    /// - `castle_direction` The internal representation of a move in `SAN`.
    /// - `position` The chess position the chess move was played in.
    /// - `ruleset` The chess ruleset used in the game the move was played in.
    fn handle_castling(
        san_castle_direction: &SanCastleDirection,
        position: &Position,
        ruleset: &Ruleset,
    ) -> Option<Turn> {
        let possible_moves = ruleset.get_possible_turns(position);

        let castle_direction = match san_castle_direction {
            SanCastleDirection::Kingside => CastleDirection::Kingside,
            SanCastleDirection::Queenside => CastleDirection::Queenside,
        };

        possible_moves
            .into_iter()
            .find(|&turn| turn == Turn::Castle(castle_direction))
    }

    /// Tries to convert a `SanPawnMove` into an actual `Turn` object that is playable in the given `position`.
    /// - `pawn_move` The internal representation of a move in `SAN`.
    /// - `position` The chess position the chess move was played in.
    /// - `ruleset` The chess ruleset used in the game the move was played in.
    fn handle_pawn_move(
        pawn_move: &SanPawnMove,
        position: &Position,
        ruleset: &Ruleset,
    ) -> Option<Turn> {
        let possible_moves = ruleset.get_possible_turns(position);
        let mut possible_moves: Vec<NormalTurn> = possible_moves
            .into_iter()
            .filter_map(|turn| match turn {
                Turn::Normal(normal_turn) => Some(normal_turn),
                Turn::Castle(_) => None,
            })
            .collect();

        possible_moves.retain(|turn| {
            pawn_move.target_field == turn.target
                && pawn_move.promotion_piece == turn.promotion
                && position
                    .get_field_occupation(&turn.origin)
                    .unwrap()
                    .get_type()
                    == PieceType::Pawn
        });

        let Some(san_origin_field) = pawn_move.origin_field.as_ref() else {
            return Some(Turn::Normal(*possible_moves.first()?));
        };

        possible_moves
            .into_iter()
            .find(|&turn| match san_origin_field {
                SanOriginField::Field(field) => *field == turn.origin,
                SanOriginField::Column(column) => turn.origin.get_column() == *column,
                SanOriginField::Row(row) => turn.origin.get_row() == *row,
            })
            .map(Turn::Normal)
    }
}
