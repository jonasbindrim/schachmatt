#[cfg(test)]
mod tests {
    use schachmatt::{Board::*, FEN, PieceType, SAN, Turn};

    #[test]
    pub fn export_san_pawn_push() {
        let test_turn = Turn::new(FIELD_D2, FIELD_D4, None);
        let fen_definition = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let test_position = FEN::import(fen_definition).unwrap();
        assert_eq!(SAN::export(test_turn, &test_position), "d4");
    }

    #[test]
    pub fn export_san_pawn_push_promotion() {
        let test_turn = Turn::new(FIELD_A7, FIELD_A8, Some(PieceType::Queen));
        let test_position = FEN::import("4k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        assert_eq!(SAN::export(test_turn, &test_position), "a8=Q+");
    }

    #[test]
    pub fn export_san_pawn_capture() {
        let test_turn = Turn::new(FIELD_A2, FIELD_B3, None);
        let test_position = FEN::import("4k3/8/8/8/8/1r6/P7/4K3 w - - 0 1").unwrap();
        assert_eq!(SAN::export(test_turn, &test_position), "axb3");
    }

    #[test]
    pub fn export_san_pawn_capture_en_passant() {
        let test_turn = Turn::new(FIELD_A5, FIELD_B6, None);
        let test_position = FEN::import("4k3/8/8/Pp6/8/8/8/4K3 w - b5 0 1").unwrap();
        assert_eq!(SAN::export(test_turn, &test_position), "axb6");
    }

    #[test]
    pub fn export_san_pawn_capture_with_promotion() {
        let test_turn = Turn::new(FIELD_A7, FIELD_B8, Some(PieceType::Rook));
        let test_position = FEN::import("1p2k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        assert_eq!(SAN::export(test_turn, &test_position), "axb8=R+");
    }

    #[test]
    pub fn export_san_castle_white_king() {
        let test_turn = Turn::new(FIELD_E1, FIELD_G1, None);
        let test_position = FEN::import("4k3/p7/8/8/8/8/7P/4K2R w K - 0 1").unwrap();
        assert_eq!(SAN::export(test_turn, &test_position), "O-O");
    }

    #[test]
    pub fn export_san_castle_black_queen() {
        let test_turn = Turn::new(FIELD_E8, FIELD_C8, None);
        let test_position = FEN::import("r3k3/8/8/8/8/8/8/R3K3 b q - 0 1").unwrap();
        assert_eq!(SAN::export(test_turn, &test_position), "O-O-O");
    }

    #[test]
    pub fn export_san_piece_move() {
        let test_turn = Turn::new(FIELD_A1, FIELD_A8, None);
        let test_position = FEN::import("4k3/8/8/8/8/8/8/R3K3 w - - 0 1").unwrap();
        assert_eq!(SAN::export(test_turn, &test_position), "Ra8+");
    }

    #[test]
    pub fn export_san_piece_move_capture() {
        let test_turn = Turn::new(FIELD_F2, FIELD_D4, None);
        let test_position = FEN::import("4k3/8/8/8/3r4/8/5B2/R3K3 w - - 0 1").unwrap();
        assert_eq!(SAN::export(test_turn, &test_position), "Bxd4");
    }

    #[test]
    pub fn export_san_piece_move_capture_ambigious_white() {
        let test_turn = Turn::new(FIELD_F3, FIELD_E5, None);
        let test_position = FEN::import("4k3/8/8/4p3/8/3N1N2/8/R3K3 w - - 0 1").unwrap();
        assert_eq!(SAN::export(test_turn, &test_position), "Nfxe5");
    }

    #[test]
    pub fn export_san_piece_move_capture_ambigious_black() {
        let test_turn = Turn::new(FIELD_D6, FIELD_E4, None);
        let test_position = FEN::import("4k3/8/3n1n2/8/4P3/8/8/R3K3 b - - 0 1").unwrap();
        assert_eq!(SAN::export(test_turn, &test_position), "Ndxe4");
    }

    #[test]
    pub fn export_san_piece_move_capture_double_ambigious_black() {
        let test_turn = Turn::new(FIELD_D6, FIELD_E4, None);
        let test_position = FEN::import("4k3/8/3n1n2/8/4P3/8/3n4/R5K b - - 0 1").unwrap();
        assert_eq!(SAN::export(test_turn, &test_position), "Nd6xe4");
    }
}
