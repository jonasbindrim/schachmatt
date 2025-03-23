use crate::Piece;

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum FieldOccupation {
    None,
    Piece(Piece),
}
