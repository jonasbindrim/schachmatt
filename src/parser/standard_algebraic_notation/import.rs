use crate::{
    CLASSIC_RULESET, Columns,
    Fields::{FIELD_A1, FIELD_E1},
    PieceType, PlayerColor, Position, Rows, Ruleset, Turn,
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

        possible_moves.into_iter().find(|&turn| {
            if piece_move.target_field != turn.target {
                return false;
            }

            if position
                .get_field_occupation(&turn.current)
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
                SanOriginField::Field(origin) => turn.current == *origin,
                SanOriginField::Column(column) => turn.current.get_column() == *column,
                SanOriginField::Row(row) => turn.current.get_row() == *row,
            }
        })
    }

    /// Tries to convert a `SanCastleDirection` into an actual `Turn` object that is playable in the given `position`.
    /// - `castle_direction` The internal representation of a move in `SAN`.
    /// - `position` The chess position the chess move was played in.
    /// - `ruleset` The chess ruleset used in the game the move was played in.
    fn handle_castling(
        castle_direction: &SanCastleDirection,
        position: &Position,
        ruleset: &Ruleset,
    ) -> Option<Turn> {
        let possible_moves = ruleset.get_possible_turns(position);
        let player_color = position.get_active_color();

        // Initiate with row for white
        let mut target_field = FIELD_A1;
        let mut starting_field = FIELD_E1;

        // Change row if color is black
        if player_color == PlayerColor::Black {
            starting_field.set_row(Rows::ROW_8);
            target_field.set_row(Rows::ROW_8);
        }

        match castle_direction {
            SanCastleDirection::Kingside => target_field.set_column(Columns::COLUMN_G),
            SanCastleDirection::Queenside => target_field.set_column(Columns::COLUMN_C),
        }

        possible_moves
            .into_iter()
            .find(|&turn| turn.target == target_field && turn.current == starting_field)
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
        let mut possible_moves = ruleset.get_possible_turns(position);

        possible_moves.retain(|turn| {
            pawn_move.target_field == turn.target
                && pawn_move.promotion_piece == turn.promotion
                && position
                    .get_field_occupation(&turn.current)
                    .unwrap()
                    .get_type()
                    == PieceType::Pawn
        });

        let Some(san_origin_field) = pawn_move.origin_field.as_ref() else {
            return Some(*possible_moves.first()?);
        };

        possible_moves
            .into_iter()
            .find(|&turn| match san_origin_field {
                SanOriginField::Field(field) => *field == turn.current,
                SanOriginField::Column(column) => turn.current.get_column() == *column,
                SanOriginField::Row(row) => turn.current.get_row() == *row,
            })
    }
}
