use core::fmt;
use std::error::Error;

/// Error used to indicate an unsuccessful conversion from or into a `String`.
#[derive(Debug)]
pub struct ParserError {
    error_message: String,
}

impl ParserError {
    /// Creates a new `ParserError` object.
    /// - `returns` - A `ParserError` object
    #[must_use]
    pub fn new(msg: &str) -> ParserError {
        ParserError {
            error_message: msg.to_string(),
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        &self.error_message
    }
}
