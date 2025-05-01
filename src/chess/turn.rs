use crate::{Field, PieceType};

/// A `Turn` is defined as a combination of two fields and an optional promotion piece.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Turn {
    pub(crate) current: Field,
    pub(crate) target: Field,
    pub(crate) promotion: Option<PieceType>,
}

impl Turn {
    /// Creates a new `Turn` object.
    /// - `current` - The field from which a piece moves
    /// - `target` - The field to which a piece moves
    /// - `promotion_piece` - Used to describe the promotion piece if a pawn promotes
    /// - `returns` - A new `Turn` object
    #[must_use]
    pub fn new(current: Field, target: Field, promotion_piece: Option<PieceType>) -> Self {
        Turn {
            current,
            target,
            promotion: promotion_piece,
        }
    }
}
