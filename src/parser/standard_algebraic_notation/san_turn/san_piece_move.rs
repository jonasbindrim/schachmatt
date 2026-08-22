use pest::iterators::Pair;

use crate::{
    Field, PieceType,
    parser::standard_algebraic_notation::san_turn::{Rule, san_origin_field::SanOriginField},
};

/// Describes the maximal amount of data used to describe a piece move in SAN.
pub struct SanPieceMove {
    pub target_field: Field,
    pub piece_type: PieceType,
    pub origin_field: Option<SanOriginField>,
}

impl SanPieceMove {
    /// Takes the pest rule for a full piece move and converts it into a `SanPieceMove` object.
    pub(super) fn from_piece_move_full(piece_move_full_rule: Pair<Rule>) -> Self {
        let mut piece_type: Option<PieceType> = None;
        let mut target_field: Option<Field> = None;
        let mut origin_field: Option<SanOriginField> = None;

        for subrule in piece_move_full_rule.into_inner() {
            match subrule.as_rule() {
                Rule::piece_symbol => {
                    let piecetype_letter =
                        (subrule.as_str().as_bytes()[0] as char).to_ascii_lowercase();
                    piece_type = PieceType::import_piecetype(piecetype_letter);
                }
                Rule::piece_move => {
                    let (target, origin) = Self::from_piece_move_rule(subrule);
                    target_field = Some(target);
                    origin_field = origin;
                }
                _ => unreachable!(),
            }
        }

        SanPieceMove {
            target_field: target_field.unwrap(),
            piece_type: piece_type.unwrap(),
            origin_field,
        }
    }

    fn from_piece_move_rule(piece_move_rule: Pair<Rule>) -> (Field, Option<SanOriginField>) {
        let mut target_field: Option<Field> = None;
        let mut origin_field: Option<SanOriginField> = None;

        for subrule in piece_move_rule.into_inner() {
            match subrule.as_rule() {
                Rule::target_field => target_field = Some(Self::from_target_field_rule(subrule)),
                Rule::origin_field => {
                    origin_field = Some(SanOriginField::from_origin_field_rule(subrule))
                }
                _ => unreachable!(),
            }
        }

        (target_field.unwrap(), origin_field)
    }

    fn from_target_field_rule(from_field_data: Pair<Rule>) -> Field {
        let data = from_field_data.as_str().as_bytes();
        let from_column = data[0] - b'a';
        let from_row = data[1] - b'1';
        Field::new(from_column, from_row).unwrap()
    }
}
