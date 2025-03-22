use crate::{Field, PieceType};

/// A `Turn` is defined as a combination of two fields and an optional promotion piece.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub struct Turn {
    pub(crate) from: Field,
    pub(crate) to: Field,
    pub(crate) promotion: Option<PieceType>,
}

impl Turn {
    /// Creates a new `Turn` object.
    /// - `from_field` - The field from which a piece moves
    /// - `to_field` - The field to which a piece moves
    /// - `promotion_piece` - Used to describe the promotion piece if a pawn promotes
    /// - `returns` - A new `Turn` object
    #[must_use]
    pub fn new(from_field: Field, to_field: Field, promotion_piece: Option<PieceType>) -> Self {
        Turn {
            from: from_field,
            to: to_field,
            promotion: promotion_piece,
        }
    }
}
