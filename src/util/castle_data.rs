use crate::{Board::*, Field};

pub(crate) static CASTLE_BQ_CHECKED: [Field; 3] = [FIELD_E8, FIELD_D8, FIELD_C8];
pub(crate) static CASTLE_BQ_BLOCKED: [Field; 3] = [FIELD_D8, FIELD_C8, FIELD_B8];
pub(crate) static CASTLE_BK_CHECKED: [Field; 3] = [FIELD_E8, FIELD_F8, FIELD_G8];
pub(crate) static CASTLE_BK_BLOCKED: [Field; 2] = [FIELD_F8, FIELD_G8];
pub(crate) static CASTLE_WQ_CHECKED: [Field; 3] = [FIELD_E1, FIELD_D1, FIELD_C1];
pub(crate) static CASTLE_WQ_BLOCKED: [Field; 3] = [FIELD_D1, FIELD_C1, FIELD_B1];
pub(crate) static CASTLE_WK_CHECKED: [Field; 3] = [FIELD_E1, FIELD_F1, FIELD_G1];
pub(crate) static CASTLE_WK_BLOCKED: [Field; 2] = [FIELD_F1, FIELD_G1];
