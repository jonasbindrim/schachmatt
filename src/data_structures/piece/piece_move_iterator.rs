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
        let row_increment = match iterator.row.direction {
            Some(incr) => {
                let increment = iterator.row.min_step + self.increment * incr;
                if (incr > 0 && increment > iterator.row.max_step)
                    || (incr < 0 && increment < iterator.row.max_step)
                {
                    return None;
                }
                increment
            }
            None => iterator.row.min_step,
        };

        // Calculate the column increment for the next move
        let column_increment = match iterator.column.direction {
            Some(incr) => {
                let increment = iterator.column.min_step + self.increment * incr;
                if (incr > 0 && increment > iterator.column.max_step)
                    || (incr < 0 && increment < iterator.column.max_step)
                {
                    return None;
                }
                increment
            }
            None => iterator.column.min_step,
        };

        // Calculate next iterator field
        let target_row = (self.base_field.row as i8 + row_increment) as u8;
        let target_column = (self.base_field.column as i8 + column_increment) as u8;

        let turn = Turn {
            current: self.base_field,
            target: Field::new(target_column, target_row)?,
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
}
