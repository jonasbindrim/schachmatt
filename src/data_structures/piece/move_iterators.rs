#[derive(Debug)]
pub(crate) struct BasicIterator {
    pub start: i8,
    pub end: i8,
    pub increment: Option<i8>,
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
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: None,
        },
        row: BasicIterator {
            start: 1,
            end: 2,
            increment: Some(ROW_UP),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 1,
            increment: Some(COLUMN_RIGHT),
        },
        row: BasicIterator {
            start: 1,
            end: 1,
            increment: Some(ROW_UP),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -1,
            increment: Some(COLUMN_LEFT),
        },
        row: BasicIterator {
            start: 1,
            end: 1,
            increment: Some(ROW_UP),
        },
    },
];

pub(crate) const PAWN_BLACK_ITERATORS: [MoveIterator; 3] = [
    MoveIterator {
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: None,
        },
        row: BasicIterator {
            start: -1,
            end: -2,
            increment: Some(ROW_DOWN),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 1,
            increment: Some(COLUMN_RIGHT),
        },
        row: BasicIterator {
            start: -1,
            end: -1,
            increment: Some(ROW_DOWN),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -1,
            increment: Some(COLUMN_LEFT),
        },
        row: BasicIterator {
            start: -1,
            end: -1,
            increment: Some(ROW_DOWN),
        },
    },
];

pub(crate) const ROOK_ITERATORS: [MoveIterator; 4] = [
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 7,
            increment: Some(COLUMN_RIGHT),
        },
        row: BasicIterator {
            start: 0,
            end: 0,
            increment: None,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -7,
            increment: Some(COLUMN_LEFT),
        },
        row: BasicIterator {
            start: 0,
            end: 0,
            increment: None,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: None,
        },
        row: BasicIterator {
            start: 1,
            end: 7,
            increment: Some(ROW_UP),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: None,
        },
        row: BasicIterator {
            start: -1,
            end: -7,
            increment: Some(ROW_DOWN),
        },
    },
];

pub(crate) const BISHOP_ITERATORS: [MoveIterator; 4] = [
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 7,
            increment: Some(COLUMN_RIGHT),
        },
        row: BasicIterator {
            start: 1,
            end: 7,
            increment: Some(ROW_UP),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 7,
            increment: Some(COLUMN_RIGHT),
        },
        row: BasicIterator {
            start: -1,
            end: -7,
            increment: Some(ROW_DOWN),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -7,
            increment: Some(COLUMN_LEFT),
        },
        row: BasicIterator {
            start: 1,
            end: 7,
            increment: Some(ROW_UP),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -7,
            increment: Some(COLUMN_LEFT),
        },
        row: BasicIterator {
            start: -1,
            end: -7,
            increment: Some(ROW_DOWN),
        },
    },
];

pub(crate) const KNIGHT_ITERATORS: [MoveIterator; 8] = [
    MoveIterator {
        column: BasicIterator {
            start: 2,
            end: 2,
            increment: Some(COLUMN_RIGHT),
        },
        row: BasicIterator {
            start: 1,
            end: 1,
            increment: Some(ROW_UP),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 1,
            increment: Some(COLUMN_RIGHT),
        },
        row: BasicIterator {
            start: 2,
            end: 2,
            increment: Some(ROW_UP),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -1,
            increment: Some(COLUMN_LEFT),
        },
        row: BasicIterator {
            start: 2,
            end: 2,
            increment: Some(ROW_UP),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -2,
            end: -2,
            increment: Some(COLUMN_LEFT),
        },
        row: BasicIterator {
            start: 1,
            end: 1,
            increment: Some(ROW_UP),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -1,
            increment: Some(COLUMN_LEFT),
        },
        row: BasicIterator {
            start: -2,
            end: -2,
            increment: Some(ROW_DOWN),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -2,
            end: -2,
            increment: Some(COLUMN_LEFT),
        },
        row: BasicIterator {
            start: -1,
            end: -1,
            increment: Some(ROW_DOWN),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 1,
            increment: Some(COLUMN_RIGHT),
        },
        row: BasicIterator {
            start: -2,
            end: -2,
            increment: Some(ROW_DOWN),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 2,
            end: 2,
            increment: Some(COLUMN_RIGHT),
        },
        row: BasicIterator {
            start: -1,
            end: -1,
            increment: Some(ROW_DOWN),
        },
    },
];

pub(crate) const QUEEN_ITERATORS: [MoveIterator; 8] = [
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 7,
            increment: Some(COLUMN_RIGHT),
        },
        row: BasicIterator {
            start: 0,
            end: 0,
            increment: None,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -7,
            increment: Some(COLUMN_LEFT),
        },
        row: BasicIterator {
            start: 0,
            end: 0,
            increment: None,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: None,
        },
        row: BasicIterator {
            start: 1,
            end: 7,
            increment: Some(ROW_UP),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: None,
        },
        row: BasicIterator {
            start: -1,
            end: -7,
            increment: Some(ROW_DOWN),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 7,
            increment: Some(COLUMN_RIGHT),
        },
        row: BasicIterator {
            start: 1,
            end: 7,
            increment: Some(ROW_UP),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 7,
            increment: Some(COLUMN_RIGHT),
        },
        row: BasicIterator {
            start: -1,
            end: -7,
            increment: Some(ROW_DOWN),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -7,
            increment: Some(COLUMN_LEFT),
        },
        row: BasicIterator {
            start: 1,
            end: 7,
            increment: Some(ROW_UP),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -7,
            increment: Some(COLUMN_LEFT),
        },
        row: BasicIterator {
            start: -1,
            end: -7,
            increment: Some(ROW_DOWN),
        },
    },
];

pub(crate) const KING_ITERATORS: [MoveIterator; 8] = [
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 2,
            increment: Some(COLUMN_RIGHT),
        },
        row: BasicIterator {
            start: 0,
            end: 0,
            increment: None,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -2,
            increment: Some(COLUMN_LEFT),
        },
        row: BasicIterator {
            start: 0,
            end: 0,
            increment: None,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: None,
        },
        row: BasicIterator {
            start: 1,
            end: 1,
            increment: Some(ROW_UP),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: None,
        },
        row: BasicIterator {
            start: -1,
            end: -1,
            increment: Some(ROW_DOWN),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 1,
            increment: Some(COLUMN_RIGHT),
        },
        row: BasicIterator {
            start: 1,
            end: 1,
            increment: Some(ROW_UP),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 1,
            increment: Some(COLUMN_RIGHT),
        },
        row: BasicIterator {
            start: -1,
            end: -1,
            increment: Some(ROW_DOWN),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -1,
            increment: Some(COLUMN_LEFT),
        },
        row: BasicIterator {
            start: 1,
            end: 1,
            increment: Some(ROW_UP),
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -1,
            increment: Some(COLUMN_LEFT),
        },
        row: BasicIterator {
            start: -1,
            end: -1,
            increment: Some(ROW_DOWN),
        },
    },
];
