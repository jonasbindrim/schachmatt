#[derive(Debug)]
pub(crate) struct BasicIterator {
    pub start: i8,
    pub end: i8,
    pub increment: i8,
}

#[derive(Debug)]
pub(crate) struct MoveIterator {
    pub column: BasicIterator,
    pub row: BasicIterator,
}

pub(crate) const PAWN_WHITE_ITERATORS: [MoveIterator; 3] = [
    MoveIterator {
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: 0,
        },
        row: BasicIterator {
            start: 1,
            end: 2,
            increment: 1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 1,
            increment: 1,
        },
        row: BasicIterator {
            start: 1,
            end: 1,
            increment: 1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -1,
            increment: -1,
        },
        row: BasicIterator {
            start: 1,
            end: 1,
            increment: 1,
        },
    },
];

pub(crate) const PAWN_BLACK_ITERATORS: [MoveIterator; 3] = [
    MoveIterator {
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: 0,
        },
        row: BasicIterator {
            start: -1,
            end: -2,
            increment: -1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 1,
            increment: 1,
        },
        row: BasicIterator {
            start: -1,
            end: -1,
            increment: -1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -1,
            increment: -1,
        },
        row: BasicIterator {
            start: -1,
            end: -1,
            increment: -1,
        },
    },
];

pub(crate) const ROOK_ITERATORS: [MoveIterator; 4] = [
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 7,
            increment: 1,
        },
        row: BasicIterator {
            start: 0,
            end: 0,
            increment: 0,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -7,
            increment: -1,
        },
        row: BasicIterator {
            start: 0,
            end: 0,
            increment: 0,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: 0,
        },
        row: BasicIterator {
            start: 1,
            end: 7,
            increment: 1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: 0,
        },
        row: BasicIterator {
            start: -1,
            end: -7,
            increment: -1,
        },
    },
];

pub(crate) const BISHOP_ITERATORS: [MoveIterator; 4] = [
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 7,
            increment: 1,
        },
        row: BasicIterator {
            start: 1,
            end: 7,
            increment: 1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 7,
            increment: 1,
        },
        row: BasicIterator {
            start: -1,
            end: -7,
            increment: -1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -7,
            increment: -1,
        },
        row: BasicIterator {
            start: 1,
            end: 7,
            increment: 1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -7,
            increment: -1,
        },
        row: BasicIterator {
            start: -1,
            end: -7,
            increment: -1,
        },
    },
];

pub(crate) const KNIGHT_ITERATORS: [MoveIterator; 8] = [
    MoveIterator {
        column: BasicIterator {
            start: 2,
            end: 2,
            increment: 1,
        },
        row: BasicIterator {
            start: 1,
            end: 1,
            increment: 1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 1,
            increment: 1,
        },
        row: BasicIterator {
            start: 2,
            end: 2,
            increment: 1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -1,
            increment: -1,
        },
        row: BasicIterator {
            start: 2,
            end: 2,
            increment: 1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -2,
            end: -2,
            increment: -1,
        },
        row: BasicIterator {
            start: 1,
            end: 1,
            increment: 1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -1,
            increment: -1,
        },
        row: BasicIterator {
            start: -2,
            end: -2,
            increment: -1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -2,
            end: -2,
            increment: -1,
        },
        row: BasicIterator {
            start: -1,
            end: -1,
            increment: -1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 1,
            increment: 1,
        },
        row: BasicIterator {
            start: -2,
            end: -2,
            increment: -1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 2,
            end: 2,
            increment: 1,
        },
        row: BasicIterator {
            start: -1,
            end: -1,
            increment: -1,
        },
    },
];

pub(crate) const QUEEN_ITERATORS: [MoveIterator; 8] = [
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 7,
            increment: 1,
        },
        row: BasicIterator {
            start: 0,
            end: 0,
            increment: 0,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -7,
            increment: -1,
        },
        row: BasicIterator {
            start: 0,
            end: 0,
            increment: 0,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: 0,
        },
        row: BasicIterator {
            start: 1,
            end: 7,
            increment: 1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: 0,
        },
        row: BasicIterator {
            start: -1,
            end: -7,
            increment: -1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 7,
            increment: 1,
        },
        row: BasicIterator {
            start: 1,
            end: 7,
            increment: 1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 7,
            increment: 1,
        },
        row: BasicIterator {
            start: -1,
            end: -7,
            increment: -1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -7,
            increment: -1,
        },
        row: BasicIterator {
            start: 1,
            end: 7,
            increment: 1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -7,
            increment: -1,
        },
        row: BasicIterator {
            start: -1,
            end: -7,
            increment: -1,
        },
    },
];

pub(crate) const KING_ITERATORS: [MoveIterator; 8] = [
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 2,
            increment: 1,
        },
        row: BasicIterator {
            start: 0,
            end: 0,
            increment: 0,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -2,
            increment: -1,
        },
        row: BasicIterator {
            start: 0,
            end: 0,
            increment: 0,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: 0,
        },
        row: BasicIterator {
            start: 1,
            end: 1,
            increment: 1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 0,
            end: 0,
            increment: 0,
        },
        row: BasicIterator {
            start: -1,
            end: -1,
            increment: -1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 1,
            increment: 1,
        },
        row: BasicIterator {
            start: 1,
            end: 1,
            increment: 1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: 1,
            end: 1,
            increment: 1,
        },
        row: BasicIterator {
            start: -1,
            end: -1,
            increment: -1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -1,
            increment: -1,
        },
        row: BasicIterator {
            start: 1,
            end: 1,
            increment: 1,
        },
    },
    MoveIterator {
        column: BasicIterator {
            start: -1,
            end: -1,
            increment: -1,
        },
        row: BasicIterator {
            start: -1,
            end: -1,
            increment: -1,
        },
    },
];
