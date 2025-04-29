/// Stores information on whether a player is allowed to castle kingside or queenside.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct CastlingRights {
    queenside: bool,
    kingside: bool,
}

impl CastlingRights {

    /// Creates a new instance of CastlingRights with the specified queenside and kingside rights.
    pub fn new(queenside: bool, kingside: bool) -> Self {
        CastlingRights {
            queenside,
            kingside,
        }
    }

    /// Sets the queenside castling right.
    pub fn set_queenside(&mut self, can_castle: bool) {
        self.queenside = can_castle;
    }

    /// Sets the kingside castling right.
    pub fn set_kingside(&mut self, can_castle: bool) {
        self.kingside = can_castle;
    }

    /// Get the queenside castling right.
    pub fn get_queenside(&self) -> bool {
        self.queenside
    }

    /// Get the kingside castling right.
    pub fn get_kingside(&self) -> bool {
        self.kingside
    }
}
