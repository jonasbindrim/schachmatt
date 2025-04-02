#[cfg(test)]
mod forsyth_edwards_notation_import_tests {
    use schachmatt::{FEN, Position};

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_empty_string() {
        FEN::import("").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_block_amount() {
        FEN::import("a a a").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_amount_of_pieces_per_row_1() {
        FEN::import("9/8/8/8/8/8/8/8 w KQkq - 0 1").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_amount_of_pieces_per_row_2() {
        FEN::import("PPPPPPPPP/8/8/8/8/8/8/8 w KQkq - 0 1").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_color_at_turn() {
        FEN::import("8/8/8/8/8/8/8/8 c KQkq - 0 1").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_castling_sign() {
        FEN::import("8/8/8/8/8/8/8/8 w g - 0 1").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_en_passent_square() {
        FEN::import("8/8/8/8/8/8/8/8 w K t7 0 1").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_fullmove_counter() {
        FEN::import("8/8/8/8/8/8/8/8 w K - -1 1").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_halfmove_clock() {
        FEN::import("8/8/8/8/8/8/8/8 w K - 0 -1").expect("Returned error");
    }

    #[test]
    pub fn import_empty_field() {
        FEN::import("8/8/8/8/8/8/8/8 w KQkq - 0 1").unwrap();
    }

    #[test]
    pub fn default_chess_position_can_be_imported() {
        let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let import = FEN::import(fen_string).unwrap();

        let position = Position::default();
        assert_eq!(import, position);
    }

    #[test]
    pub fn position_test_1() {
        let fen_string = "3nr1k1/1b1q1N2/3B2p1/1p5p/3pP2P/6Q1/1P4P1/5RK1 b - - 0 1";
        let import = FEN::import(fen_string).unwrap();
        let export = FEN::export(&import);
        assert_eq!(fen_string, export);
    }

    #[test]
    pub fn position_test_2() {
        let fen_string = "2kr1b1r/ppqnnppp/2p1p3/3pP1N1/3P4/2PB1P2/PP4PP/1RBQ1RK1 b - - 0 1";
        let import = FEN::import(fen_string).unwrap();
        let export = FEN::export(&import);
        assert_eq!(fen_string, export);
    }

    #[test]
    pub fn position_test_3() {
        let fen_string = "8/3R4/1p1r1p1k/p1p5/P1P2bP1/3P4/5K2/8 w Kq c3 0 1";
        let import = FEN::import(fen_string).unwrap();
        let export = FEN::export(&import);
        assert_eq!(fen_string, export);
    }
}
