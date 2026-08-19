use pest::Parser;

use crate::{
    SanParserError,
    parser::standard_algebraic_notation::san_turn::{
        san_castle_direction::SanCastleDirection, san_pawn_move::SanPawnMove,
        san_piece_move::SanPieceMove,
    },
};

pub(super) mod san_castle_direction;
pub(super) mod san_origin_field;
pub(super) mod san_pawn_move;
pub(super) mod san_piece_move;

#[derive(Parser)]
#[grammar = "parser/standard_algebraic_notation/standard_algebraic_notation.pest"]
struct SanPestParser;

/// Contains all information that are part of the `Standard Algebraic Notation`.
/// This is basically an internal representation of the `SAN` format.
pub(super) enum SanTurn {
    Castling(SanCastleDirection),
    PawnMove(SanPawnMove),
    PieceMove(SanPieceMove),
}

impl SanTurn {
    /// Converts a string that contains a turn in 'Standard Algebraic Notation' and converts
    /// it into the internal 'SanTurn' representation.
    /// - `raw` - The string to be converted
    /// - `returns` - The resulting `SanTurn` or an error if `raw` is not a valid `SAN` move.
    pub(super) fn import(raw: &str) -> Result<Self, SanParserError> {
        let Ok(mut parsed_data) = SanPestParser::parse(Rule::turn, raw) else {
            return Err(SanParserError::InvalidData(raw.to_string()));
        };

        let Some(turn_type) = parsed_data.next().unwrap().into_inner().next() else {
            return Err(SanParserError::InvalidData(raw.to_string()));
        };

        match turn_type.as_rule() {
            Rule::pawn_move => Ok(Self::PawnMove(SanPawnMove::from_pawn_move_rule(
                turn_type,
            ))),
            Rule::castling => Ok(Self::Castling(SanCastleDirection::from_castling_rule(
                &turn_type,
            ))),
            Rule::piece_move_full => Ok(Self::PieceMove(SanPieceMove::from_piece_move_full(
                turn_type,
            ))),
            _ => Err(SanParserError::InvalidData(raw.to_string())),
        }
    }
}
