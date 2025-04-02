#[cfg(test)]
mod tests {
    use schachmatt::{Board::*, LAN, PieceType, Turn};

    #[test]
    pub fn import_lan_pawn_push() {
        let turn = LAN::import("d2d4").unwrap();
        assert_eq!(turn, Turn::new(FIELD_D2, FIELD_D4, None));
    }

    #[test]
    pub fn import_lan_push_ambigious() {
        let turn = LAN::import("e4e5").unwrap();
        assert_eq!(turn, Turn::new(FIELD_E4, FIELD_E5, None));
    }

    #[test]
    pub fn import_lan_pawn_push_promotion() {
        let turn = LAN::import("a7a8q").unwrap();
        assert_eq!(turn, Turn::new(FIELD_A7, FIELD_A8, Some(PieceType::Queen)));
    }

    #[test]
    pub fn import_lan_pawn_capture() {
        let turn = LAN::import("a2xb3").unwrap();
        assert_eq!(turn, Turn::new(FIELD_A2, FIELD_B3, None));
    }

    #[test]
    pub fn import_lan_pawn_capture_en_passant() {
        let turn = LAN::import("a5xb6").unwrap();
        assert_eq!(turn, Turn::new(FIELD_A5, FIELD_B6, None));
    }

    #[test]
    pub fn import_lan_pawn_capture_with_promotion() {
        let turn = LAN::import("a7xb8r").unwrap();
        assert_eq!(turn, Turn::new(FIELD_A7, FIELD_B8, Some(PieceType::Rook)));
    }

    #[test]
    pub fn import_lan_castle_white_king() {
        let turn = LAN::import("e1g1").unwrap();
        assert_eq!(turn, Turn::new(FIELD_E1, FIELD_G1, None));
    }

    #[test]
    pub fn import_lan_castle_black_queen() {
        let turn = LAN::import("e8c8").unwrap();
        assert_eq!(turn, Turn::new(FIELD_E8, FIELD_C8, None));
    }

    #[test]
    pub fn import_lan_piece_move() {
        let turn = LAN::import("Ra1a8").unwrap();
        assert_eq!(turn, Turn::new(FIELD_A1, FIELD_A8, None));
    }

    #[test]
    pub fn import_lan_piece_move_capture() {
        let turn = LAN::import("Bf2xd4").unwrap();
        assert_eq!(turn, Turn::new(FIELD_F2, FIELD_D4, None));
    }

    #[test]
    pub fn import_lan_piece_move_capture_ambigious_white() {
        let turn = LAN::import("f3xe5").unwrap();
        assert_eq!(turn, Turn::new(FIELD_F3, FIELD_E5, None));
    }

    #[test]
    pub fn import_lan_piece_move_capture_ambigious_black() {
        let turn = LAN::import("Nd6e4").unwrap();
        assert_eq!(turn, Turn::new(FIELD_D6, FIELD_E4, None));
    }

    #[test]
    pub fn import_lan_piece_move_capture_double_ambigious_black() {
        let turn = LAN::import("Nd6xe4").unwrap();
        assert_eq!(turn, Turn::new(FIELD_D6, FIELD_E4, None));
    }
}
