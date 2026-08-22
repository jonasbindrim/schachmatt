#[cfg(test)]
mod long_algebraic_notation_export_tests {
    use schachmatt::{CastleDirection, Fen, Fields::*, Lan, NormalTurn, PieceType, Position, Turn};

    #[test]
    pub fn export_lan_pawn_push() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_D2, FIELD_D4, None));
        let position = Position::default();
        assert_eq!(Lan::export(&test_turn, &position), "d2d4");
    }

    #[test]
    pub fn export_lan_pawn_push_promotion() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_A7, FIELD_A8, Some(PieceType::Queen)));
        let position = Fen::import("8/P4k2/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        assert_eq!(Lan::export(&test_turn, &position), "a7a8q");
    }

    #[test]
    pub fn export_lan_pawn_capture() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_A2, FIELD_B3, None));
        let position = Fen::import("8/5k2/8/8/8/1r6/P7/4K3 w - - 0 1").unwrap();
        assert_eq!(Lan::export(&test_turn, &position), "a2xb3");
    }

    #[test]
    pub fn export_lan_pawn_capture_en_passant() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_A5, FIELD_B6, None));
        let position = Fen::import("4k3/8/8/Pp6/8/8/8/4K3 w - b6 0 1").unwrap();
        assert_eq!(Lan::export(&test_turn, &position), "a5xb6");
    }

    #[test]
    pub fn export_lan_pawn_capture_with_promotion() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_A7, FIELD_B8, Some(PieceType::Rook)));
        let position = Fen::import("1n2k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        assert_eq!(Lan::export(&test_turn, &position), "a7xb8r");
    }

    #[test]
    pub fn export_lan_castle_king() {
        let test_turn = Turn::Castle(CastleDirection::Kingside);
        let position = Fen::import("4k3/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();
        assert_eq!(Lan::export(&test_turn, &position), "O-O");
    }

    #[test]
    pub fn export_lan_castle_queen() {
        let test_turn = Turn::Castle(CastleDirection::Queenside);
        let position = Fen::import("4k3/8/8/8/8/8/8/R3K3 w Q - 0 1").unwrap();
        assert_eq!(Lan::export(&test_turn, &position), "O-O-O");
    }

    #[test]
    pub fn export_lan_piece_move() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_A1, FIELD_A8, None));
        let position = Fen::import("8/4k3/8/8/8/8/8/R3K3 w - - 0 1").unwrap();
        assert_eq!(Lan::export(&test_turn, &position), "a1a8");
    }

    #[test]
    pub fn export_lan_piece_move_capture() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_F2, FIELD_D4, None));
        let position = Fen::import("4k3/8/8/8/3r4/8/5B2/4K3 w - - 0 1").unwrap();
        assert_eq!(Lan::export(&test_turn, &position), "f2xd4");
    }

    #[test]
    pub fn export_lan_piece_move_capture_ambigious_white() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_F3, FIELD_E5, None));
        let position = Fen::import("4k3/8/8/4p3/8/3N1N2/8/4K3 w - - 0 1").unwrap();
        assert_eq!(Lan::export(&test_turn, &position), "f3xe5");
    }

    #[test]
    pub fn export_lan_piece_move_capture_ambigious_black() {
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_D6, FIELD_E4, None));
        let position = Fen::import("4k3/8/3n1n2/8/4P3/8/8/4K3 w - - 0 1").unwrap();
        assert_eq!(Lan::export(&test_turn, &position), "d6xe4");
    }
}
