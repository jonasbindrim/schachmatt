use crate::Field;

pub(crate) static CASTLE_BQ_CHECKED: [Field; 3] = [
    Field { column: 4, row: 7 },
    Field { column: 3, row: 7 },
    Field { column: 2, row: 7 },
];

pub(crate) static CASTLE_BQ_BLOCKED: [Field; 3] = [
    Field { column: 3, row: 7 },
    Field { column: 2, row: 7 },
    Field { column: 1, row: 7 },
];

pub(crate) static CASTLE_BK_CHECKED: [Field; 3] = [
    Field { column: 4, row: 7 },
    Field { column: 5, row: 7 },
    Field { column: 6, row: 7 },
];

pub(crate) static CASTLE_BK_BLOCKED: [Field; 2] =
    [Field { column: 5, row: 7 }, Field { column: 6, row: 7 }];

pub(crate) static CASTLE_WQ_CHECKED: [Field; 3] = [
    Field { column: 4, row: 0 },
    Field { column: 3, row: 0 },
    Field { column: 2, row: 0 },
];

pub(crate) static CASTLE_WQ_BLOCKED: [Field; 3] = [
    Field { column: 3, row: 0 },
    Field { column: 2, row: 0 },
    Field { column: 1, row: 0 },
];

pub(crate) static CASTLE_WK_CHECKED: [Field; 3] = [
    Field { column: 4, row: 0 },
    Field { column: 5, row: 0 },
    Field { column: 6, row: 0 },
];

pub(crate) static CASTLE_WK_BLOCKED: [Field; 2] =
    [Field { column: 5, row: 0 }, Field { column: 6, row: 0 }];
