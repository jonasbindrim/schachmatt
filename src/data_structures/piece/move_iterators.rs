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
    pub column: BasicIterator,
    pub row: BasicIterator,
}

// These constants are seen from the whites player pov
const ROW_UP: i8 = 1;
const ROW_DOWN: i8 = -1;
const COLUMN_RIGHT: i8 = 1;
const COLUMN_LEFT: i8 = -1;

pub(crate) const PAWN_WHITE_ITERATORS: [MoveIterator; 3] = [
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

pub(crate) const PAWN_BLACK_ITERATORS: [MoveIterator; 3] = [
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

pub(crate) const ROOK_ITERATORS: [MoveIterator; 4] = [
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

pub(crate) const BISHOP_ITERATORS: [MoveIterator; 4] = [
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

pub(crate) const KNIGHT_ITERATORS: [MoveIterator; 8] = [
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

pub(crate) const QUEEN_ITERATORS: [MoveIterator; 8] = [
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

pub(crate) const KING_ITERATORS: [MoveIterator; 8] = [
    MoveIterator {
        column: BasicIterator::new(1, 2, Some(COLUMN_RIGHT)),
        row: BasicIterator::new(0, 0, None),
    },
    MoveIterator {
        column: BasicIterator::new(-1, -2, Some(COLUMN_LEFT)),
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
