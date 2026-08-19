use pest::iterators::Pair;

use crate::parser::standard_algebraic_notation::san_turn::Rule;

/// Describes the direction of a castle move.
pub enum SanCastleDirection {
    Kingside,
    Queenside,
}

impl SanCastleDirection {
    /// Takes the pest rule for castling and converts it into a `SanCastleDirection` object.
    pub(super) fn from_castling_rule(castling_rule: &Pair<Rule>) -> Self {
        match castling_rule.as_str() {
            "O-O" | "0-0" => Self::Kingside,
            "O-O-O" | "0-0-0" => Self::Queenside,
            _ => unreachable!(),
        }
    }
}
