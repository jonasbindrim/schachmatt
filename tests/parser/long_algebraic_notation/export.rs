#[cfg(test)]
mod long_algebraic_notation_export_tests {
    use schachmatt::{CastleDirection, Fields::*, Lan, NormalTurn, PieceType, Turn};

    #[test]
    pub fn export_lan_pawn_push() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_D2, FIELD_D4, None));
        assert_eq!(Lan::export(&test_turn), "d2d4");
    }

    #[test]
    pub fn export_lan_pawn_push_promotion() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_A7, FIELD_A8, Some(PieceType::Queen)));
        assert_eq!(Lan::export(&test_turn), "a7a8q");
    }

    #[test]
    pub fn export_lan_pawn_capture() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_A2, FIELD_B3, None));
        assert_eq!(Lan::export(&test_turn), "a2b3");
    }

    #[test]
    pub fn export_lan_pawn_capture_en_passant() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_A5, FIELD_B6, None));
        assert_eq!(Lan::export(&test_turn), "a5b6");
    }

    #[test]
    pub fn export_lan_pawn_capture_with_promotion() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_A7, FIELD_B8, Some(PieceType::Rook)));
        assert_eq!(Lan::export(&test_turn), "a7b8r");
    }

    #[test]
    pub fn export_lan_castle_white_king() {
        let test_turn = Turn::Castle(CastleDirection::Kingside);
        assert_eq!(Lan::export(&test_turn), "O-O");
    }

    #[test]
    pub fn export_lan_castle_black_queen() {
        let test_turn = Turn::Castle(CastleDirection::Queenside);
        assert_eq!(Lan::export(&test_turn), "O-O-O");
    }

    #[test]
    pub fn export_lan_piece_move() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_A1, FIELD_A8, None));
        assert_eq!(Lan::export(&test_turn), "a1a8");
    }

    #[test]
    pub fn export_lan_piece_move_capture() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_F2, FIELD_D4, None));
        assert_eq!(Lan::export(&test_turn), "f2d4");
    }

    #[test]
    pub fn export_lan_piece_move_capture_ambigious_white() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_F3, FIELD_E5, None));
        assert_eq!(Lan::export(&test_turn), "f3e5");
    }

    #[test]
    pub fn export_lan_piece_move_capture_ambigious_black() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_D6, FIELD_E4, None));
        assert_eq!(Lan::export(&test_turn), "d6e4");
    }

    #[test]
    pub fn export_lan_piece_move_capture_double_ambigious_black() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_D6, FIELD_E4, None));
        assert_eq!(Lan::export(&test_turn), "d6e4");
    }
}
