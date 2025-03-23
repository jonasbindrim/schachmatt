mod move_iterators;
pub(crate) mod piece_move_iterator;
pub mod piece_type;

use crate::PlayerColor;

use self::{
    move_iterators::{
        BISHOP_ITERATORS, KING_ITERATORS, KNIGHT_ITERATORS, MoveIterator, PAWN_BLACK_ITERATORS,
        PAWN_WHITE_ITERATORS, QUEEN_ITERATORS, ROOK_ITERATORS,
    },
    piece_type::PieceType,
};

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub struct Piece {
    piece_type: PieceType,
    piece_color: PlayerColor,
}

impl Piece {
    /// Creates a new `Piece`-struct.
    #[must_use]
    pub fn new(piece_type: PieceType, piece_color: PlayerColor) -> Self {
        Piece {
            piece_type,
            piece_color,
        }
    }

    // /// Returns the player color of the piece.
    // /// - `returns` - The player color of the piece
    #[must_use]
    pub fn get_color(&self) -> PlayerColor {
        self.piece_color
    }

    /// Returns the piece type of the piece.
    /// - `returns` - The piece type of the piece
    #[must_use]
    pub fn get_type(&self) -> PieceType {
        self.piece_type
    }

    /// Converts the `Piece` into its char representation.
    /// Pieces are represented by a single letter. Uppercase letters are whites pieces, lowercase letters are blacks pieces.
    /// - `returns` - A char representing the piece
    #[must_use]
    pub fn export_piece(&self) -> char {
        let letter = self.piece_type.export_piecetype_lowercase();
        let color = self.piece_color;

        match color {
            PlayerColor::Black => letter,
            PlayerColor::White => letter.to_ascii_uppercase(),
        }
    }

    /// Converts a char to its corresponding piece in the algebraic chess notation
    /// - `letter` - The letter representing a piece
    /// - `returns` - The `Piece` object represented by the given letter
    #[must_use]
    pub fn import_piece(letter: char) -> Option<Self> {
        let piece_type = PieceType::import_piecetype(letter.to_ascii_lowercase())?;

        let piece_color = if letter.is_ascii_lowercase() {
            PlayerColor::Black
        } else {
            PlayerColor::White
        };

        Some(Piece {
            piece_type,
            piece_color
        })
    }

    /// Returns the move iterators for the piece.
    /// - `returns` - The move iterators for the piece
    pub(crate) fn movement_modifiers(&self) -> &[MoveIterator] {
        match self.get_type() {
            PieceType::Pawn => match self.get_color() {
                PlayerColor::Black => &PAWN_BLACK_ITERATORS,
                PlayerColor::White => &PAWN_WHITE_ITERATORS,
            },
            PieceType::Rook => &ROOK_ITERATORS,
            PieceType::Bishop => &BISHOP_ITERATORS,
            PieceType::Knight => &KNIGHT_ITERATORS,
            PieceType::Queen => &QUEEN_ITERATORS,
            PieceType::King => &KING_ITERATORS,
        }
    }
}
