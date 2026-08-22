use crate::{Field, PieceType};

/// A `Turn` is the internal representation of a single move in a game of chess.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Turn {
    Normal(NormalTurn),
    Castle(CastleDirection),
}

/// A normal turn is the moving of a single piece in a game of chess.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct NormalTurn {
    pub(crate) origin: Field,
    pub(crate) target: Field,
    pub(crate) promotion: Option<PieceType>,
}

impl NormalTurn {
    /// Creates a new `NormalTurn` object.
    /// - `origin` - The field from which a piece moves
    /// - `target` - The field to which a piece moves
    /// - `promotion_piece` - Used to describe the promotion piece if a pawn promotes
    /// - `returns` - A new `NormalTurn` object
    #[must_use]
    pub fn new(origin: Field, target: Field, promotion_piece: Option<PieceType>) -> Self {
        NormalTurn {
            origin,
            target,
            promotion: promotion_piece,
        }
    }
}

/// Describes a chess turn where a player castles to either side.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CastleDirection {
    Kingside,
    Queenside,
}
