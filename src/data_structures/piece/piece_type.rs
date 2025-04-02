#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceType {
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
    pub fn export_piecetype_lowercase(self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Rook => 'r',
            PieceType::Bishop => 'b',
            PieceType::Knight => 'n',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        }
    }

    /// Converts the `PieceType` into its char representation.
    /// Pieces are represented by a single letter. All letters are uppercase.
    /// - `returns` - A char representing the piece
    #[must_use]
    pub fn export_piecetype_uppercase(self) -> char {
        match self {
            PieceType::Pawn => 'P',
            PieceType::Rook => 'R',
            PieceType::Bishop => 'B',
            PieceType::Knight => 'N',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
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
