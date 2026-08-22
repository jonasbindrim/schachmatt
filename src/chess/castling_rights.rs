use crate::chess::turn::CastleDirection;

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

    /// Sets the castling rights for the given direction.
    pub fn set(&mut self, direction: CastleDirection, can_castle: bool) {
        match direction {
            CastleDirection::Kingside => self.kingside = can_castle,
            CastleDirection::Queenside => self.queenside = can_castle,
        };
    }

    /// Get the castling rights for the given direction.
    pub fn get(&self, direction: CastleDirection) -> bool {
        match direction {
            CastleDirection::Kingside => self.kingside,
            CastleDirection::Queenside => self.queenside,
        }
    }
}
