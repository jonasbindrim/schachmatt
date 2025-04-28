pub mod columns {
    pub const COLUMN_A: u8 = 0;
    pub const COLUMN_B: u8 = 1;
    pub const COLUMN_C: u8 = 2;
    pub const COLUMN_D: u8 = 3;
    pub const COLUMN_E: u8 = 4;
    pub const COLUMN_F: u8 = 5;
    pub const COLUMN_G: u8 = 6;
    pub const COLUMN_H: u8 = 7;
    pub const COLUMN_AMOUNT: usize = 8;
}

pub mod rows {
    pub const ROW_1: u8 = 0;
    pub const ROW_2: u8 = 1;
    pub const ROW_3: u8 = 2;
    pub const ROW_4: u8 = 3;
    pub const ROW_5: u8 = 4;
    pub const ROW_6: u8 = 5;
    pub const ROW_7: u8 = 6;
    pub const ROW_8: u8 = 7;
    pub const ROW_AMOUNT: usize = 8;
}

pub mod fields {
    use crate::Field;

    use super::{columns::*, rows::*};

    pub const FIELD_A1: Field = Field::new(COLUMN_A, ROW_1).unwrap();
    pub const FIELD_B1: Field = Field::new(COLUMN_B, ROW_1).unwrap();
    pub const FIELD_C1: Field = Field::new(COLUMN_C, ROW_1).unwrap();
    pub const FIELD_D1: Field = Field::new(COLUMN_D, ROW_1).unwrap();
    pub const FIELD_E1: Field = Field::new(COLUMN_E, ROW_1).unwrap();
    pub const FIELD_F1: Field = Field::new(COLUMN_F, ROW_1).unwrap();
    pub const FIELD_G1: Field = Field::new(COLUMN_G, ROW_1).unwrap();
    pub const FIELD_H1: Field = Field::new(COLUMN_H, ROW_1).unwrap();

    pub const FIELD_A2: Field = Field::new(COLUMN_A, ROW_2).unwrap();
    pub const FIELD_B2: Field = Field::new(COLUMN_B, ROW_2).unwrap();
    pub const FIELD_C2: Field = Field::new(COLUMN_C, ROW_2).unwrap();
    pub const FIELD_D2: Field = Field::new(COLUMN_D, ROW_2).unwrap();
    pub const FIELD_E2: Field = Field::new(COLUMN_E, ROW_2).unwrap();
    pub const FIELD_F2: Field = Field::new(COLUMN_F, ROW_2).unwrap();
    pub const FIELD_G2: Field = Field::new(COLUMN_G, ROW_2).unwrap();
    pub const FIELD_H2: Field = Field::new(COLUMN_H, ROW_2).unwrap();

    pub const FIELD_A3: Field = Field::new(COLUMN_A, ROW_3).unwrap();
    pub const FIELD_B3: Field = Field::new(COLUMN_B, ROW_3).unwrap();
    pub const FIELD_C3: Field = Field::new(COLUMN_C, ROW_3).unwrap();
    pub const FIELD_D3: Field = Field::new(COLUMN_D, ROW_3).unwrap();
    pub const FIELD_E3: Field = Field::new(COLUMN_E, ROW_3).unwrap();
    pub const FIELD_F3: Field = Field::new(COLUMN_F, ROW_3).unwrap();
    pub const FIELD_G3: Field = Field::new(COLUMN_G, ROW_3).unwrap();
    pub const FIELD_H3: Field = Field::new(COLUMN_H, ROW_3).unwrap();

    pub const FIELD_A4: Field = Field::new(COLUMN_A, ROW_4).unwrap();
    pub const FIELD_B4: Field = Field::new(COLUMN_B, ROW_4).unwrap();
    pub const FIELD_C4: Field = Field::new(COLUMN_C, ROW_4).unwrap();
    pub const FIELD_D4: Field = Field::new(COLUMN_D, ROW_4).unwrap();
    pub const FIELD_E4: Field = Field::new(COLUMN_E, ROW_4).unwrap();
    pub const FIELD_F4: Field = Field::new(COLUMN_F, ROW_4).unwrap();
    pub const FIELD_G4: Field = Field::new(COLUMN_G, ROW_4).unwrap();
    pub const FIELD_H4: Field = Field::new(COLUMN_H, ROW_4).unwrap();

    pub const FIELD_A5: Field = Field::new(COLUMN_A, ROW_5).unwrap();
    pub const FIELD_B5: Field = Field::new(COLUMN_B, ROW_5).unwrap();
    pub const FIELD_C5: Field = Field::new(COLUMN_C, ROW_5).unwrap();
    pub const FIELD_D5: Field = Field::new(COLUMN_D, ROW_5).unwrap();
    pub const FIELD_E5: Field = Field::new(COLUMN_E, ROW_5).unwrap();
    pub const FIELD_F5: Field = Field::new(COLUMN_F, ROW_5).unwrap();
    pub const FIELD_G5: Field = Field::new(COLUMN_G, ROW_5).unwrap();
    pub const FIELD_H5: Field = Field::new(COLUMN_H, ROW_5).unwrap();

    pub const FIELD_A6: Field = Field::new(COLUMN_A, ROW_6).unwrap();
    pub const FIELD_B6: Field = Field::new(COLUMN_B, ROW_6).unwrap();
    pub const FIELD_C6: Field = Field::new(COLUMN_C, ROW_6).unwrap();
    pub const FIELD_D6: Field = Field::new(COLUMN_D, ROW_6).unwrap();
    pub const FIELD_E6: Field = Field::new(COLUMN_E, ROW_6).unwrap();
    pub const FIELD_F6: Field = Field::new(COLUMN_F, ROW_6).unwrap();
    pub const FIELD_G6: Field = Field::new(COLUMN_G, ROW_6).unwrap();
    pub const FIELD_H6: Field = Field::new(COLUMN_H, ROW_6).unwrap();

    pub const FIELD_A7: Field = Field::new(COLUMN_A, ROW_7).unwrap();
    pub const FIELD_B7: Field = Field::new(COLUMN_B, ROW_7).unwrap();
    pub const FIELD_C7: Field = Field::new(COLUMN_C, ROW_7).unwrap();
    pub const FIELD_D7: Field = Field::new(COLUMN_D, ROW_7).unwrap();
    pub const FIELD_E7: Field = Field::new(COLUMN_E, ROW_7).unwrap();
    pub const FIELD_F7: Field = Field::new(COLUMN_F, ROW_7).unwrap();
    pub const FIELD_G7: Field = Field::new(COLUMN_G, ROW_7).unwrap();
    pub const FIELD_H7: Field = Field::new(COLUMN_H, ROW_7).unwrap();

    pub const FIELD_A8: Field = Field::new(COLUMN_A, ROW_8).unwrap();
    pub const FIELD_B8: Field = Field::new(COLUMN_B, ROW_8).unwrap();
    pub const FIELD_C8: Field = Field::new(COLUMN_C, ROW_8).unwrap();
    pub const FIELD_D8: Field = Field::new(COLUMN_D, ROW_8).unwrap();
    pub const FIELD_E8: Field = Field::new(COLUMN_E, ROW_8).unwrap();
    pub const FIELD_F8: Field = Field::new(COLUMN_F, ROW_8).unwrap();
    pub const FIELD_G8: Field = Field::new(COLUMN_G, ROW_8).unwrap();
    pub const FIELD_H8: Field = Field::new(COLUMN_H, ROW_8).unwrap();

    pub static BOARD_FIELDS: [Field; 64] = [
        FIELD_A1, FIELD_B1, FIELD_C1, FIELD_D1, FIELD_E1, FIELD_F1, FIELD_G1, FIELD_H1, FIELD_A2,
        FIELD_B2, FIELD_C2, FIELD_D2, FIELD_E2, FIELD_F2, FIELD_G2, FIELD_H2, FIELD_A3, FIELD_B3,
        FIELD_C3, FIELD_D3, FIELD_E3, FIELD_F3, FIELD_G3, FIELD_H3, FIELD_A4, FIELD_B4, FIELD_C4,
        FIELD_D4, FIELD_E4, FIELD_F4, FIELD_G4, FIELD_H4, FIELD_A5, FIELD_B5, FIELD_C5, FIELD_D5,
        FIELD_E5, FIELD_F5, FIELD_G5, FIELD_H5, FIELD_A6, FIELD_B6, FIELD_C6, FIELD_D6, FIELD_E6,
        FIELD_F6, FIELD_G6, FIELD_H6, FIELD_A7, FIELD_B7, FIELD_C7, FIELD_D7, FIELD_E7, FIELD_F7,
        FIELD_G7, FIELD_H7, FIELD_A8, FIELD_B8, FIELD_C8, FIELD_D8, FIELD_E8, FIELD_F8, FIELD_G8,
        FIELD_H8,
    ];
}

pub mod metadata {
    pub static METADATA_KEY_RESULT: &str = "Result";
    pub static METADATA_KEY_EVENT: &str = "Event";
    pub static METADATA_KEY_SITE: &str = "Site";
    pub static METADATA_KEY_DATE: &str = "Date";
    pub static METADATA_KEY_ROUND: &str = "Round";
    pub static METADATA_KEY_WHITE: &str = "White";
    pub static METADATA_KEY_BLACK: &str = "Black";
    pub static METADATA_KEY_FEN: &str = "Fen";
}
