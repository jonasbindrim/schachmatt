/// Defines the two player colors
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum PlayerColor {
    Black,
    White,
}

impl PlayerColor {
    /// Returns the player color not currently represented by self.
    /// - `returns` - The reversed player color
    #[must_use]
    pub fn reverse(&self) -> PlayerColor {
        match self {
            PlayerColor::Black => PlayerColor::White,
            PlayerColor::White => PlayerColor::Black,
        }
    }
}
