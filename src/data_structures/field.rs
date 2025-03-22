/// Represents a board location in a chess game.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub struct Field {
    pub(crate) column: u8, // Column (a-h) is represented as values (0-7)
    pub(crate) row: u8,    // Row (1-8) is represented as values (0-7)
}

impl ToString for Field {
    /// Converts a field into into a string with the following format: column + row. Example: a3.
    /// - `returns` - A field represented as a string.
    fn to_string(&self) -> String {
        let mut notation = String::new();
        notation.push((self.column + b'a') as char);
        notation.push_str(&(self.row + 1).to_string());
        notation
    }
}

impl Field {
    /// Converts a field from a string into the field data type if possible.
    /// The string has to follow the format: column + row. Example: a3.
    /// - `field` - The string which should be converted into a `Field` object
    /// - `returns` - A `Field` object
    #[must_use]
    pub fn from_string(field: &str) -> Option<Field> {
        let byte_field = field.as_bytes();
        if byte_field.len() == 2 {
            let letter = byte_field[0] as char;
            let number = (byte_field[1]) - b'0';
            if ('a'..='h').contains(&letter) && (1..=8).contains(&number) {
                return Some(Field {
                    column: (letter as u8) - b'a',
                    row: number - 1,
                });
            }
        }
        Option::None
    }

    /// Converts two usize params into a field.
    /// - `column` - The column of the new field
    /// - `row` - The row of the new field
    /// - `returns` - A field containing the column and row parameters
    pub(crate) fn from_usize(column: usize, row: usize) -> Field {
        Field {
            column: u8::try_from(column).unwrap(),
            row: u8::try_from(row).unwrap(),
        }
    }
}
