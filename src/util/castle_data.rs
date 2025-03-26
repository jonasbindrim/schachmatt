use crate::{Board, Field};

pub(crate) static CASTLE_BQ_CHECKED: [Field; 3] = [
    Field::new(Board::COLUMN_E, Board::ROW_8).unwrap(),
    Field::new(Board::COLUMN_D, Board::ROW_8).unwrap(),
    Field::new(Board::COLUMN_C, Board::ROW_8).unwrap(),
];

pub(crate) static CASTLE_BQ_BLOCKED: [Field; 3] = [
    Field::new(Board::COLUMN_D, Board::ROW_8).unwrap(),
    Field::new(Board::COLUMN_C, Board::ROW_8).unwrap(),
    Field::new(Board::COLUMN_B, Board::ROW_8).unwrap(),
];

pub(crate) static CASTLE_BK_CHECKED: [Field; 3] = [
    Field::new(Board::COLUMN_E, Board::ROW_8).unwrap(),
    Field::new(Board::COLUMN_F, Board::ROW_8).unwrap(),
    Field::new(Board::COLUMN_G, Board::ROW_8).unwrap(),
];

pub(crate) static CASTLE_BK_BLOCKED: [Field; 2] = [
    Field::new(Board::COLUMN_F, Board::ROW_8).unwrap(),
    Field::new(Board::COLUMN_G, Board::ROW_8).unwrap(),
];

pub(crate) static CASTLE_WQ_CHECKED: [Field; 3] = [
    Field::new(Board::COLUMN_E, Board::ROW_1).unwrap(),
    Field::new(Board::COLUMN_D, Board::ROW_1).unwrap(),
    Field::new(Board::COLUMN_C, Board::ROW_1).unwrap(),
];

pub(crate) static CASTLE_WQ_BLOCKED: [Field; 3] = [
    Field::new(Board::COLUMN_D, Board::ROW_1).unwrap(),
    Field::new(Board::COLUMN_C, Board::ROW_1).unwrap(),
    Field::new(Board::COLUMN_B, Board::ROW_1).unwrap(),
];

pub(crate) static CASTLE_WK_CHECKED: [Field; 3] = [
    Field::new(Board::COLUMN_E, Board::ROW_1).unwrap(),
    Field::new(Board::COLUMN_F, Board::ROW_1).unwrap(),
    Field::new(Board::COLUMN_G, Board::ROW_1).unwrap(),
];

pub(crate) static CASTLE_WK_BLOCKED: [Field; 2] = [
    Field::new(Board::COLUMN_F, Board::ROW_1).unwrap(),
    Field::new(Board::COLUMN_G, Board::ROW_1).unwrap(),
];
