use crate::{Field, Turn};

use super::move_iterators::MoveIterator;

/// Stores the iterators for a single piece
#[cfg_attr(test, derive(Debug))]
pub(crate) struct PieceMoveIterator<'a> {
    move_iterator: &'a [MoveIterator],
    index: usize,
    increment: i8,
    base_field: Field,
}

impl<'a> PieceMoveIterator<'a> {
    /// Creates a new piece move iterator
    /// - `move_iterator` - The base move iterator for the given piece type
    /// - `base_field` - The field on which the piece resides
    pub(crate) fn new(
        move_iterator: &'a [MoveIterator],
        base_field: Field,
    ) -> PieceMoveIterator<'a> {
        PieceMoveIterator {
            move_iterator,
            index: 0,
            increment: 0,
            base_field,
        }
    }

    /// Returns the current turn stored in this iterator if its valid
    /// - `returns` - Turn stored in this iterator
    pub(crate) fn current(&mut self) -> Option<Turn> {
        let iterator = &self.move_iterator[self.index];

        // Calculate the row increment for the next move
        let row_increment = match iterator.row.increment {
            Some(incr) => {
                let increment = iterator.row.start + self.increment * incr;
                if (incr > 0 && increment > iterator.row.end) || (incr < 0 && increment < iterator.row.end) {
                    return None;
                }
                increment
            },
            None => iterator.row.start,
        };

        // Calculate the column increment for the next move
        let column_increment = match iterator.column.increment {
            Some(incr) => {
                let increment = iterator.column.start + self.increment * incr;
                if (incr > 0 && increment > iterator.column.end) || (incr < 0 && increment < iterator.column.end) {
                    return None;
                }
                increment
            },
            None => iterator.column.start,
        };

        // Calculate next iterator field
        let temp_row = self.base_field.row as i8 + row_increment;
        let temp_column = self.base_field.column as i8 + column_increment;

        // Check if turn is in bounds and return if so
        if Self::out_of_bounds_check(temp_row, temp_column) {
            return Option::None;
        }

        let turn = Turn {
            from: self.base_field,
            to: Field::new(temp_column as u8, temp_row as u8)?,
            promotion: None,
        };

        self.increment += 1;
        Some(turn)
    }

    /// Step over to the next movement iterator.
    /// - `returns` - Whether another iterator exists
    pub(crate) fn step(&mut self) -> bool {
        if self.move_iterator.len() - 1 > self.index {
            self.index += 1;
            self.increment = 0;
            return true;
        }
        false
    }

    /// Checks if a field would be out of bounds
    /// - `row` - The row to check
    /// - `column` - The column to check
    /// - `returns` - True if the field is out of bounds
    #[inline]
    fn out_of_bounds_check(row: i8, column: i8) -> bool {
        !(0..=7).contains(&row) || !(0..=7).contains(&column)
    }
}
