#[cfg(test)]
mod tests {
    use crate::{Board::*, FEN, SAN, Turn, data_structures::piece::piece_type::PieceType};

    #[test]
    pub fn import_san_pawn_push() {
        let mut test_position =
            FEN::import("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        if let Ok(turn) = SAN::import("d4", &mut test_position) {
            assert_eq!(turn, Turn::new(FIELD_D2, FIELD_D4, None));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_san_push_ambigious() {
        let mut test_position =
            FEN::import("rnbqkb1r/pp3ppp/3ppn2/2p5/4PP2/1P6/PBPP2PP/RN1QKBNR w KQkq - 0 1")
                .unwrap();
        if let Ok(turn) = SAN::import("e5", &mut test_position) {
            assert_eq!(turn, Turn::new(FIELD_E4, FIELD_E5, None));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_san_pawn_push_promotion() {
        let mut test_position = FEN::import("4k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        if let Ok(turn) = SAN::import("a8=Q+", &mut test_position) {
            assert_eq!(turn, Turn::new(FIELD_A7, FIELD_A8, Some(PieceType::Queen)));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_san_pawn_capture() {
        let mut test_position = FEN::import("4k3/8/8/8/8/1r6/P7/4K3 w - - 0 1").unwrap();
        if let Ok(turn) = SAN::import("axb3", &mut test_position) {
            assert_eq!(turn, Turn::new(FIELD_A2, FIELD_B3, None));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_san_pawn_capture_en_passant() {
        let mut test_position = FEN::import("4k3/8/8/Pp6/8/8/8/4K3 w - b5 0 1").unwrap();
        if let Ok(turn) = SAN::import("axb6", &mut test_position) {
            assert_eq!(turn, Turn::new(FIELD_A5, FIELD_B6, None));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_san_pawn_capture_with_promotion() {
        let mut test_position = FEN::import("1p2k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        if let Ok(turn) = SAN::import("axb8=R+", &mut test_position) {
            assert_eq!(turn, Turn::new(FIELD_A7, FIELD_B8, Some(PieceType::Rook)));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_san_castle_white_king() {
        let mut test_position = FEN::import("r3k3/p7/8/8/8/8/7P/4K2R w K - 0 1").unwrap();
        if let Ok(turn) = SAN::import("O-O", &mut test_position) {
            assert_eq!(turn, Turn::new(FIELD_E1, FIELD_G1, None));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_san_castle_black_queen() {
        let mut test_position = FEN::import("r3k3/8/8/8/8/8/8/R3K3 b q - 0 1").unwrap();
        if let Ok(turn) = SAN::import("O-O-O", &mut test_position) {
            assert_eq!(turn, Turn::new(FIELD_E8, FIELD_C8, None));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_san_piece_move() {
        let mut test_position = FEN::import("4k3/8/8/8/8/8/8/R3K3 w - - 0 1").unwrap();
        if let Ok(turn) = SAN::import("Ra8+", &mut test_position) {
            assert_eq!(turn, Turn::new(FIELD_A1, FIELD_A8, None));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_san_piece_move_capture() {
        let mut test_position = FEN::import("4k3/8/8/8/3r4/8/5B2/R3K3 w - - 0 1").unwrap();
        if let Ok(turn) = SAN::import("Bxd4", &mut test_position) {
            assert_eq!(turn, Turn::new(FIELD_F2, FIELD_D4, None));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_san_piece_move_capture_ambigious_white() {
        let mut test_position = FEN::import("4k3/8/8/4p3/8/3N1N2/8/R3K3 w - - 0 1").unwrap();
        if let Ok(turn) = SAN::import("Nfxe5", &mut test_position) {
            assert_eq!(turn, Turn::new(FIELD_F3, FIELD_E5, None));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_san_piece_move_capture_ambigious_black() {
        let mut test_position = FEN::import("4k3/8/3n1n2/8/4P3/8/8/R3K3 b - - 0 1").unwrap();
        if let Ok(turn) = SAN::import("Ndxe4", &mut test_position) {
            assert_eq!(turn, Turn::new(FIELD_D6, FIELD_E4, None));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_san_piece_move_capture_double_ambigious_black() {
        let mut test_position = FEN::import("4k3/8/3n1n2/8/4P3/8/3n4/R5K b - - 0 1").unwrap();
        if let Ok(turn) = SAN::import("Nd6xe4", &mut test_position) {
            assert_eq!(turn, Turn::new(FIELD_D6, FIELD_E4, None));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn export_san_pawn_push() {
        let test_turn = Turn::new(FIELD_D2, FIELD_D4, None);

        let test_position =
            FEN::import("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        let test_data = String::from("d4");
        assert_eq!(SAN::export(test_turn, &test_position), test_data);
    }

    #[test]
    pub fn export_san_pawn_push_promotion() {
        let test_turn = Turn::new(FIELD_A7, FIELD_A8, Some(PieceType::Queen));

        let test_position = FEN::import("4k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();

        let test_data = String::from("a8=Q+");
        assert_eq!(SAN::export(test_turn, &test_position), test_data);
    }

    #[test]
    pub fn export_san_pawn_capture() {
        let test_turn = Turn::new(FIELD_A2, FIELD_B3, None);

        let test_position = FEN::import("4k3/8/8/8/8/1r6/P7/4K3 w - - 0 1").unwrap();

        let test_data = String::from("axb3");
        assert_eq!(SAN::export(test_turn, &test_position), test_data);
    }

    #[test]
    pub fn export_san_pawn_capture_en_passant() {
        let test_turn = Turn::new(FIELD_A5, FIELD_B6, None);

        let test_position = FEN::import("4k3/8/8/Pp6/8/8/8/4K3 w - b5 0 1").unwrap();

        let test_data = String::from("axb6");
        assert_eq!(SAN::export(test_turn, &test_position), test_data);
    }

    #[test]
    pub fn export_san_pawn_capture_with_promotion() {
        let test_turn = Turn::new(FIELD_A7, FIELD_B8, Some(PieceType::Rook));

        let test_position = FEN::import("1p2k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();

        let test_data = String::from("axb8=R+");
        assert_eq!(SAN::export(test_turn, &test_position), test_data);
    }

    #[test]
    pub fn export_san_castle_white_king() {
        let test_turn = Turn::new(FIELD_E1, FIELD_G1, None);

        let test_position = FEN::import("4k3/p7/8/8/8/8/7P/4K2R w K - 0 1").unwrap();

        let test_data = String::from("O-O");
        assert_eq!(SAN::export(test_turn, &test_position), test_data);
    }

    #[test]
    pub fn export_san_castle_black_queen() {
        let test_turn = Turn::new(FIELD_E8, FIELD_C8, None);

        let test_position = FEN::import("r3k3/8/8/8/8/8/8/R3K3 b q - 0 1").unwrap();

        let test_data = String::from("O-O-O");
        assert_eq!(SAN::export(test_turn, &test_position), test_data);
    }

    #[test]
    pub fn export_san_piece_move() {
        let test_turn = Turn::new(FIELD_A1, FIELD_A8, None);

        let test_position = FEN::import("4k3/8/8/8/8/8/8/R3K3 w - - 0 1").unwrap();

        let test_data = String::from("Ra8+");
        assert_eq!(SAN::export(test_turn, &test_position), test_data);
    }

    #[test]
    pub fn export_san_piece_move_capture() {
        let test_turn = Turn::new(FIELD_F2, FIELD_D4, None);

        let test_position = FEN::import("4k3/8/8/8/3r4/8/5B2/R3K3 w - - 0 1").unwrap();

        let test_data = String::from("Bxd4");
        assert_eq!(SAN::export(test_turn, &test_position), test_data);
    }

    #[test]
    pub fn export_san_piece_move_capture_ambigious_white() {
        let test_turn = Turn::new(FIELD_F3, FIELD_E5, None);

        let test_position = FEN::import("4k3/8/8/4p3/8/3N1N2/8/R3K3 w - - 0 1").unwrap();

        let test_data = String::from("Nfxe5");
        assert_eq!(SAN::export(test_turn, &test_position), test_data);
    }

    #[test]
    pub fn export_san_piece_move_capture_ambigious_black() {
        let test_turn = Turn::new(FIELD_D6, FIELD_E4, None);

        let test_position = FEN::import("4k3/8/3n1n2/8/4P3/8/8/R3K3 b - - 0 1").unwrap();

        let test_data = String::from("Ndxe4");
        assert_eq!(SAN::export(test_turn, &test_position), test_data);
    }

    #[test]
    pub fn export_san_piece_move_capture_double_ambigious_black() {
        let test_turn = Turn::new(FIELD_D6, FIELD_E4, None);

        let test_position = FEN::import("4k3/8/3n1n2/8/4P3/8/3n4/R5K b - - 0 1").unwrap();

        let test_data = String::from("Nd6xe4");
        assert_eq!(SAN::export(test_turn, &test_position), test_data);
    }
}
