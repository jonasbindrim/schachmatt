#[cfg(test)]
mod standard_algebraic_notation_export_tests {
    use schachmatt::{CastleDirection, Fen, Fields::*, NormalTurn, PieceType, San, Turn};

    #[test]
    pub fn import_san_pawn_push() {
        let fen_definition = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut test_position = Fen::import(fen_definition).unwrap();
        let turn = San::import("d4", &mut test_position).unwrap();
        assert_eq!(
            turn,
            Turn::Normal(NormalTurn::new(FIELD_D2, FIELD_D4, None))
        );
    }

    #[test]
    pub fn import_san_push_ambiguous() {
        let fen_definition = "rnbqkb1r/pp3ppp/3ppn2/2p5/4PP2/1P6/PBPP2PP/RN1QKBNR w KQkq - 0 1";
        let mut test_position = Fen::import(fen_definition).unwrap();
        let turn = San::import("e5", &mut test_position).unwrap();
        assert_eq!(
            turn,
            Turn::Normal(NormalTurn::new(FIELD_E4, FIELD_E5, None))
        );
    }

    #[test]
    pub fn import_san_pawn_push_promotion() {
        let mut test_position = Fen::import("4k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        let turn = San::import("a8=Q+", &mut test_position).unwrap();
        assert_eq!(
            turn,
            Turn::Normal(NormalTurn::new(FIELD_A7, FIELD_A8, Some(PieceType::Queen)))
        );
    }

    #[test]
    pub fn import_san_pawn_capture() {
        let mut test_position = Fen::import("4k3/8/8/8/8/1r6/P7/4K3 w - - 0 1").unwrap();
        let turn = San::import("axb3", &mut test_position).unwrap();
        assert_eq!(
            turn,
            Turn::Normal(NormalTurn::new(FIELD_A2, FIELD_B3, None))
        );
    }

    #[test]
    pub fn import_san_pawn_capture_en_passant() {
        let mut test_position = Fen::import("4k3/8/8/Pp6/8/8/8/4K3 w - b5 0 1").unwrap();
        let turn = San::import("axb6", &mut test_position).unwrap();
        assert_eq!(
            turn,
            Turn::Normal(NormalTurn::new(FIELD_A5, FIELD_B6, None))
        );
    }

    #[test]
    pub fn import_san_pawn_capture_with_promotion() {
        let mut test_position = Fen::import("1p2k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        let turn = San::import("axb8=R+", &mut test_position).unwrap();
        assert_eq!(
            turn,
            Turn::Normal(NormalTurn::new(FIELD_A7, FIELD_B8, Some(PieceType::Rook)))
        );
    }

    #[test]
    pub fn import_san_castle_white_king() {
        let mut test_position = Fen::import("r3k3/p7/8/8/8/8/7P/4K2R w K - 0 1").unwrap();
        let turn = San::import("O-O", &mut test_position).unwrap();
        assert_eq!(turn, Turn::Castle(CastleDirection::Kingside));
    }

    #[test]
    pub fn import_san_castle_black_queen() {
        let mut test_position = Fen::import("r3k3/8/8/8/8/8/8/R3K3 b q - 0 1").unwrap();
        let turn = San::import("O-O-O", &mut test_position).unwrap();
        assert_eq!(turn, Turn::Castle(CastleDirection::Queenside));
    }

    #[test]
    pub fn import_san_piece_move() {
        let mut test_position = Fen::import("4k3/8/8/8/8/8/8/R3K3 w - - 0 1").unwrap();
        let turn = San::import("Ra8+", &mut test_position).unwrap();
        assert_eq!(
            turn,
            Turn::Normal(NormalTurn::new(FIELD_A1, FIELD_A8, None))
        );
    }

    #[test]
    pub fn import_san_piece_move_capture() {
        let mut test_position = Fen::import("4k3/8/8/8/3r4/8/5B2/R3K3 w - - 0 1").unwrap();
        let turn = San::import("Bxd4", &mut test_position).unwrap();
        assert_eq!(
            turn,
            Turn::Normal(NormalTurn::new(FIELD_F2, FIELD_D4, None))
        );
    }

    #[test]
    pub fn import_san_piece_move_capture_ambiguous_white() {
        let mut test_position = Fen::import("4k3/8/8/4p3/8/3N1N2/8/R3K3 w - - 0 1").unwrap();
        let turn = San::import("Nfxe5", &mut test_position).unwrap();
        assert_eq!(
            turn,
            Turn::Normal(NormalTurn::new(FIELD_F3, FIELD_E5, None))
        );
    }

    #[test]
    pub fn import_san_piece_move_capture_ambiguous_black() {
        let mut test_position = Fen::import("4k3/8/3n1n2/8/4P3/8/8/R3K3 b - - 0 1").unwrap();
        let turn = San::import("Ndxe4", &mut test_position).unwrap();
        assert_eq!(
            turn,
            Turn::Normal(NormalTurn::new(FIELD_D6, FIELD_E4, None))
        );
    }

    #[test]
    pub fn import_san_piece_move_capture_double_ambiguous_black() {
        let mut test_position = Fen::import("4k3/8/3n1n2/8/4P3/8/3n4/R5K b - - 0 1").unwrap();
        let turn = San::import("Nd6xe4", &mut test_position).unwrap();
        assert_eq!(
            turn,
            Turn::Normal(NormalTurn::new(FIELD_D6, FIELD_E4, None))
        );
    }

    #[test]
    pub fn import_san_piece_move_ambiguous_same_column() {
        let mut test_position = Fen::import("4k3/8/rr6/8/8/R7/8/R6K w - - 0 1").unwrap();
        let turn = San::import("R1a2", &mut test_position).unwrap();
        assert_eq!(
            turn,
            Turn::Normal(NormalTurn::new(FIELD_A1, FIELD_A2, None))
        );
    }
}
