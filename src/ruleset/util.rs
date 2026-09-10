use crate::{Field, Fields::BOARD_FIELDS, Piece, Position};

/// This function iterates over every field of the position and calls the given function for each field.
/// The predicate is checked for every piece and if the predicate returns a value, the iteration is stopped and the value is returned.
/// If no value is returned by the predicate, the iteration continues until all fields have been checked and then returns false.
/// - `position` - The current position of the game
/// - `func` - The function which is called for every field of the board
/// - `returns` - The value returned by the given func
pub(super) fn check_predicate_for_each_piece<ClosureType>(position: &Position, mut func: ClosureType) -> bool
where
    ClosureType: FnMut(Piece, Field) -> Option<bool>,
{
    for field in BOARD_FIELDS {
        if let Some(piece) = position.get_field_occupation(&field)
            && let Some(value) = func(piece, field)
        {
            return value;
        }
    }
    false
}

/// This function iterates over every field of the position and calls the given function for each field.
/// - `position` - The current position of the game
/// - `func` - The function which is called for every field of the board
pub(super) fn for_each_piece<ClosureType>(position: &Position, mut func: ClosureType)
where
    ClosureType: FnMut(Piece, Field),
{
    for field in BOARD_FIELDS {
        if let Some(piece) = position.get_field_occupation(&field) {
            func(piece, field)
        }
    }
}