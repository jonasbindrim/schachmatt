use crate::{Board, Field};

pub(crate) static CASTLE_BQ_CHECKED: [Field; 3] = [
    Field {
        column: Board::COLUMN_E,
        row: Board::ROW_8,
    },
    Field {
        column: Board::COLUMN_D,
        row: Board::ROW_8,
    },
    Field {
        column: Board::COLUMN_C,
        row: Board::ROW_8,
    },
];

pub(crate) static CASTLE_BQ_BLOCKED: [Field; 3] = [
    Field {
        column: Board::COLUMN_D,
        row: Board::ROW_8,
    },
    Field {
        column: Board::COLUMN_C,
        row: Board::ROW_8,
    },
    Field {
        column: Board::COLUMN_B,
        row: Board::ROW_8,
    },
];

pub(crate) static CASTLE_BK_CHECKED: [Field; 3] = [
    Field {
        column: Board::COLUMN_E,
        row: Board::ROW_8,
    },
    Field {
        column: Board::COLUMN_F,
        row: Board::ROW_8,
    },
    Field {
        column: Board::COLUMN_G,
        row: Board::ROW_8,
    },
];

pub(crate) static CASTLE_BK_BLOCKED: [Field; 2] = [
    Field {
        column: Board::COLUMN_F,
        row: Board::ROW_8,
    },
    Field {
        column: Board::COLUMN_G,
        row: Board::ROW_8,
    },
];

pub(crate) static CASTLE_WQ_CHECKED: [Field; 3] = [
    Field {
        column: Board::COLUMN_E,
        row: Board::ROW_1,
    },
    Field {
        column: Board::COLUMN_D,
        row: Board::ROW_1,
    },
    Field {
        column: Board::COLUMN_C,
        row: Board::ROW_1,
    },
];

pub(crate) static CASTLE_WQ_BLOCKED: [Field; 3] = [
    Field {
        column: Board::COLUMN_D,
        row: Board::ROW_1,
    },
    Field {
        column: Board::COLUMN_C,
        row: Board::ROW_1,
    },
    Field {
        column: Board::COLUMN_B,
        row: Board::ROW_1,
    },
];

pub(crate) static CASTLE_WK_CHECKED: [Field; 3] = [
    Field {
        column: Board::COLUMN_E,
        row: Board::ROW_1,
    },
    Field {
        column: Board::COLUMN_F,
        row: Board::ROW_1,
    },
    Field {
        column: Board::COLUMN_G,
        row: Board::ROW_1,
    },
];

pub(crate) static CASTLE_WK_BLOCKED: [Field; 2] = [
    Field {
        column: Board::COLUMN_F,
        row: Board::ROW_1,
    },
    Field {
        column: Board::COLUMN_G,
        row: Board::ROW_1,
    },
];
