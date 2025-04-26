use core::fmt;

use crate::{Columns::COLUMN_H, Rows::ROW_8};

/// Represents a board location in a chess game.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Field {
    pub(super) column: u8, // Column (a-h) is represented as values (0-7)
    pub(super) row: u8,    // Row (1-8) is represented as values (0-7)
}

impl Field {
    /// Creates a field from a column and a row.
    /// Both values must be between 0-7 or None will be returned.
    /// For easy usage use the board constants as arguments (Board::)
    #[must_use]
    pub const fn new(column: u8, row: u8) -> Option<Self> {
        if column > COLUMN_H || row > ROW_8 {
            return None;
        }

        Some(Self { column, row })
    }

    /// Converts two usize params into a field.
    /// - `column` - The column of the new field
    /// - `row` - The row of the new field
    /// - `returns` - A field containing the column and row parameters
    #[must_use]
    pub fn new_from_usize(column: usize, row: usize) -> Option<Self> {
        let Ok(checked_column) = u8::try_from(column) else {
            return None;
        };

        let Ok(checked_row) = u8::try_from(row) else {
            return None;
        };

        Self::new(checked_column, checked_row)
    }

    /// Converts a field from a string into the field data type if possible.
    /// The string has to follow the format: column + row. Example: a3.
    /// - `field` - The string which should be converted into a `Field` object
    /// - `returns` - A `Field` object
    #[must_use]
    pub fn new_from_string(field: &str) -> Option<Self> {
        let byte_field = field.as_bytes();

        if byte_field.len() != 2 {
            return None;
        }

        let number = byte_field[1] - b'0' - 1;
        if byte_field[0] >= b'a' && byte_field[0] <= b'h' {
            return Self::new(byte_field[0] - b'a', number);
        } else if byte_field[0] >= b'A' && byte_field[0] <= b'H' {
            return Self::new(byte_field[0] - b'A', number);
        }

        Option::None
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (self.column + b'a') as char,
            &(self.row + 1).to_string()
        )
    }
}
