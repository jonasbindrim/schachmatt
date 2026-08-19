use pest::iterators::Pair;

use crate::{
    Field, PieceType,
    parser::standard_algebraic_notation::san_turn::{Rule, san_origin_field::SanOriginField},
};

/// Describes the maximal amount of data used to describe a pawn move in SAN.
pub struct SanPawnMove {
    pub target_field: Field,
    pub promotion_piece: Option<PieceType>,
    pub origin_field: Option<SanOriginField>,
}

impl SanPawnMove {
    /// Takes the pest rule for a pawn move and converts it into a `SanPawnMove` object.
    pub(super) fn from_pawn_move_rule(pawn_move_rule: Pair<Rule>) -> Self {
        let mut target_field: Option<Field> = None;
        let mut promotion_piece: Option<PieceType> = None;
        let mut origin_field: Option<SanOriginField> = None;

        // Create target field and promotion target
        for subrule in pawn_move_rule.into_inner() {
            match subrule.as_rule() {
                Rule::target_field => target_field = Field::new_from_string(subrule.as_str()),
                Rule::promotion_piece => {
                    let letter = (subrule.as_str().as_bytes()[0] as char).to_ascii_lowercase();
                    let piece_type = PieceType::import_piecetype(letter).unwrap();
                    promotion_piece = Some(piece_type);
                }
                Rule::origin_field => {
                    origin_field = Some(SanOriginField::from_origin_field_rule(subrule))
                }
                _ => unreachable!(),
            }
        }

        Self {
            target_field: target_field.unwrap(),
            promotion_piece,
            origin_field,
        }
    }
}
