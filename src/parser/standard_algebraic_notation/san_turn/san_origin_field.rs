use pest::iterators::Pair;

use crate::{Field, parser::standard_algebraic_notation::san_turn::Rule};

/// Contains the different ways of describing the field a turn is made from.
pub enum SanOriginField {
    Field(Field),
    Column(u8),
    Row(u8)
}

impl SanOriginField {
    /// Takes the pest rule for origin fields and converts it into a `SanOriginField` object.
    pub(super) fn from_origin_field_rule(from_field_rule: Pair<Rule>) -> Self {
        let data = from_field_rule.as_str().as_bytes();

        if data.len() == 2 {
            let from_column = data[0] - b'a';
            let from_row = data[1] - b'1';
            return Self::Field(Field::new(from_column, from_row).unwrap());
        }

        if data[0] >= b'a' && data[0] <= b'h' {
            return Self::Column(data[0] - b'a');
        }

        Self::Row(data[0] - b'1')
    }
}
