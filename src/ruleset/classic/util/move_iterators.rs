use crate::{Piece, PieceType, PlayerColor};

#[derive(Debug)]
pub(crate) struct BasicIterator {
    pub min_step: i8,
    pub max_step: i8,
    pub direction: Option<i8>,
}

impl BasicIterator {
    /// Creates a new basic iterator from the given values
    const fn new(min_step: i8, max_step: i8, direction: Option<i8>) -> Self {
        BasicIterator {
            min_step,
            max_step,
            direction,
        }
    }
}

#[derive(Debug)]
pub(crate) struct MoveIterator {
    pub(crate) column: BasicIterator,
    pub(crate) row: BasicIterator,
}

/// Returns the move iterators for the piece.
/// - `returns` - The move iterators for the piece
pub(crate) fn get_movement_modifiers(piece: &Piece) -> &'static [MoveIterator] {
    match piece.get_type() {
        PieceType::Pawn => match piece.get_color() {
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

// These constants are seen from the whites player pov
const ROW_UP: i8 = 1;
const ROW_DOWN: i8 = -1;
const COLUMN_RIGHT: i8 = 1;
const COLUMN_LEFT: i8 = -1;

pub(super) const PAWN_WHITE_ITERATORS: [MoveIterator; 3] = [
    MoveIterator {
        column: BasicIterator::new(0, 0, None),
        row: BasicIterator::new(1, 2, Some(ROW_UP)),
    },
    MoveIterator {
        column: BasicIterator::new(1, 1, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(1, 1, Some(ROW_UP)),
    },
    MoveIterator {
        column: BasicIterator::new(-1, -1, Some(COLUMN_LEFT)),
        row: BasicIterator::new(1, 1, Some(ROW_UP)),
    },
];

pub(super) const PAWN_BLACK_ITERATORS: [MoveIterator; 3] = [
    MoveIterator {
        column: BasicIterator::new(0, 0, None),
        row: BasicIterator::new(-1, -2, Some(ROW_DOWN)),
    },
    MoveIterator {
        column: BasicIterator::new(1, 1, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(-1, -1, Some(ROW_DOWN)),
    },
    MoveIterator {
        column: BasicIterator::new(-1, -1, Some(COLUMN_LEFT)),
        row: BasicIterator::new(-1, -1, Some(ROW_DOWN)),
    },
];

pub(super) const ROOK_ITERATORS: [MoveIterator; 4] = [
    MoveIterator {
        column: BasicIterator::new(1, 7, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(0, 0, None),
    },
    MoveIterator {
        column: BasicIterator::new(-1, -7, Some(COLUMN_LEFT)),
        row: BasicIterator::new(0, 0, None),
    },
    MoveIterator {
        column: BasicIterator::new(0, 0, None),
        row: BasicIterator::new(1, 7, Some(ROW_UP)),
    },
    MoveIterator {
        column: BasicIterator::new(0, 0, None),
        row: BasicIterator::new(-1, -7, Some(ROW_DOWN)),
    },
];

pub(super) const BISHOP_ITERATORS: [MoveIterator; 4] = [
    MoveIterator {
        column: BasicIterator::new(1, 7, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(1, 7, Some(ROW_UP)),
    },
    MoveIterator {
        column: BasicIterator::new(1, 7, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(-1, -7, Some(ROW_DOWN)),
    },
    MoveIterator {
        column: BasicIterator::new(-1, -7, Some(COLUMN_LEFT)),
        row: BasicIterator::new(1, 7, Some(ROW_UP)),
    },
    MoveIterator {
        column: BasicIterator::new(-1, -7, Some(COLUMN_LEFT)),
        row: BasicIterator::new(-1, -7, Some(ROW_DOWN)),
    },
];

pub(super) const KNIGHT_ITERATORS: [MoveIterator; 8] = [
    MoveIterator {
        column: BasicIterator::new(2, 2, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(1, 1, Some(ROW_UP)),
    },
    MoveIterator {
        column: BasicIterator::new(1, 1, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(2, 2, Some(ROW_UP)),
    },
    MoveIterator {
        column: BasicIterator::new(-1, -1, Some(COLUMN_LEFT)),
        row: BasicIterator::new(2, 2, Some(ROW_UP)),
    },
    MoveIterator {
        column: BasicIterator::new(-2, -2, Some(COLUMN_LEFT)),
        row: BasicIterator::new(1, 1, Some(ROW_UP)),
    },
    MoveIterator {
        column: BasicIterator::new(-1, -1, Some(COLUMN_LEFT)),
        row: BasicIterator::new(-2, -2, Some(ROW_DOWN)),
    },
    MoveIterator {
        column: BasicIterator::new(-2, -2, Some(COLUMN_LEFT)),
        row: BasicIterator::new(-1, -1, Some(ROW_DOWN)),
    },
    MoveIterator {
        column: BasicIterator::new(1, 1, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(-2, -2, Some(ROW_DOWN)),
    },
    MoveIterator {
        column: BasicIterator::new(2, 2, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(-1, -1, Some(ROW_DOWN)),
    },
];

pub(super) const QUEEN_ITERATORS: [MoveIterator; 8] = [
    MoveIterator {
        column: BasicIterator::new(1, 7, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(0, 0, None),
    },
    MoveIterator {
        column: BasicIterator::new(-1, -7, Some(COLUMN_LEFT)),
        row: BasicIterator::new(0, 0, None),
    },
    MoveIterator {
        column: BasicIterator::new(0, 0, None),
        row: BasicIterator::new(1, 7, Some(ROW_UP)),
    },
    MoveIterator {
        column: BasicIterator::new(0, 0, None),
        row: BasicIterator::new(-1, -7, Some(ROW_DOWN)),
    },
    MoveIterator {
        column: BasicIterator::new(1, 7, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(1, 7, Some(ROW_UP)),
    },
    MoveIterator {
        column: BasicIterator::new(1, 7, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(-1, -7, Some(ROW_DOWN)),
    },
    MoveIterator {
        column: BasicIterator::new(-1, -7, Some(COLUMN_LEFT)),
        row: BasicIterator::new(1, 7, Some(ROW_UP)),
    },
    MoveIterator {
        column: BasicIterator::new(-1, -7, Some(COLUMN_LEFT)),
        row: BasicIterator::new(-1, -7, Some(ROW_DOWN)),
    },
];

pub(super) const KING_ITERATORS: [MoveIterator; 8] = [
    MoveIterator {
        column: BasicIterator::new(1, 1, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(0, 0, None),
    },
    MoveIterator {
        column: BasicIterator::new(-1, -1, Some(COLUMN_LEFT)),
        row: BasicIterator::new(0, 0, None),
    },
    MoveIterator {
        column: BasicIterator::new(0, 0, None),
        row: BasicIterator::new(1, 1, Some(ROW_UP)),
    },
    MoveIterator {
        column: BasicIterator::new(0, 0, None),
        row: BasicIterator::new(-1, -1, Some(ROW_DOWN)),
    },
    MoveIterator {
        column: BasicIterator::new(1, 1, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(1, 1, Some(ROW_UP)),
    },
    MoveIterator {
        column: BasicIterator::new(1, 1, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(-1, -1, Some(ROW_DOWN)),
    },
    MoveIterator {
        column: BasicIterator::new(-1, -1, Some(COLUMN_LEFT)),
        row: BasicIterator::new(1, 1, Some(ROW_UP)),
    },
    MoveIterator {
        column: BasicIterator::new(-1, -1, Some(COLUMN_LEFT)),
        row: BasicIterator::new(-1, -1, Some(ROW_DOWN)),
    },
];
