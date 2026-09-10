use crate::{Field, Fields::*, PlayerColor};

static BLACK_CASTLE_DATA: CastleData = CastleData {
    queenside_checked: &[FIELD_E8, FIELD_D8, FIELD_C8],
    queenside_blocked: &[FIELD_D8, FIELD_C8, FIELD_B8],
    kingside_checked: &[FIELD_E8, FIELD_F8, FIELD_G8],
    kingside_blocked: &[FIELD_F8, FIELD_G8],
};

static WHITE_CASTLE_DATA: CastleData = CastleData {
    queenside_checked: &[FIELD_E1, FIELD_D1, FIELD_C1],
    queenside_blocked: &[FIELD_D1, FIELD_C1, FIELD_B1],
    kingside_checked: &[FIELD_E1, FIELD_F1, FIELD_G1],
    kingside_blocked: &[FIELD_F1, FIELD_G1],
};

pub struct CastleData {
    pub queenside_checked: &'static [Field],
    pub queenside_blocked: &'static [Field],
    pub kingside_checked: &'static [Field],
    pub kingside_blocked: &'static [Field],
}

impl CastleData {
    /// Returns the castle data for the given player color.+
    /// - `player_color` - The player color to get the castle data for
    /// - `returns` - The castle data for the given player color
    pub fn get_for_color(player_color: PlayerColor) -> &'static Self {
        match player_color {
            PlayerColor::Black => &BLACK_CASTLE_DATA,
            PlayerColor::White => &WHITE_CASTLE_DATA,
        }
    }
}
