#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum PieceType {
    None,
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

impl PieceType {
    /// Converts the `PieceType` into its char representation.
    /// Pieces are represented by a single letter. All letters are lowercase.
    /// - `returns` - A char representing the piece
    #[must_use]
    pub fn export_piecetype_lowercase(self) -> Option<char> {
        match self {
            PieceType::None => None,
            PieceType::Pawn => Some('p'),
            PieceType::Rook => Some('r'),
            PieceType::Bishop => Some('b'),
            PieceType::Knight => Some('n'),
            PieceType::Queen => Some('q'),
            PieceType::King => Some('k'),
        }
    }

    /// Converts the `PieceType` into its char representation.
    /// Pieces are represented by a single letter. All letters are uppercase.
    /// - `returns` - A char representing the piece
    #[must_use]
    pub fn export_piecetype_uppercase(self) -> Option<char> {
        match self {
            PieceType::None => None,
            PieceType::Pawn => Some('P'),
            PieceType::Rook => Some('R'),
            PieceType::Bishop => Some('B'),
            PieceType::Knight => Some('N'),
            PieceType::Queen => Some('Q'),
            PieceType::King => Some('K'),
        }
    }

    /// Creates a `PieceType` out of a letter
    /// - `piece_identifier` - The char representing the piece (lowercase only)
    /// - `returns` - A `PieceType` object
    #[must_use]
    pub fn import_piecetype(piece_identifier: char) -> Option<Self> {
        match piece_identifier {
            'b' => Some(PieceType::Bishop),
            'q' => Some(PieceType::Queen),
            'n' => Some(PieceType::Knight),
            'r' => Some(PieceType::Rook),
            'k' => Some(PieceType::King),
            'p' => Some(PieceType::Pawn),
            _ => None,
        }
    }
}
