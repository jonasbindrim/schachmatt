#[cfg(test)]
mod forsyth_edwards_notation_import_tests {
    use schachmatt::{Fen, Position};

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_empty_string() {
        Fen::import("").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_block_amount() {
        Fen::import("a a a").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_amount_of_pieces_per_row_1() {
        Fen::import("9/8/8/8/8/8/8/8 w KQkq - 0 1").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_amount_of_pieces_per_row_2() {
        Fen::import("PPPPPPPPP/8/8/8/8/8/8/8 w KQkq - 0 1").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_color_at_turn() {
        Fen::import("8/8/8/8/8/8/8/8 c KQkq - 0 1").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_castling_sign() {
        Fen::import("8/8/8/8/8/8/8/8 w g - 0 1").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_en_passent_square() {
        Fen::import("8/8/8/8/8/8/8/8 w K t7 0 1").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_halfmove_clock() {
        Fen::import("8/8/8/8/8/8/8/8 w K - -1 1").expect("Returned error");
    }

    #[test]
    #[should_panic(expected = "Returned error")]
    pub fn import_test_invalid_fullmove_counter() {
        Fen::import("8/8/8/8/8/8/8/8 w K - 0 -1").expect("Returned error");
    }

    #[test]
    pub fn import_empty_field() {
        Fen::import("8/8/8/8/8/8/8/8 w KQkq - 0 1").unwrap();
    }

    #[test]
    pub fn default_chess_position_can_be_imported() {
        let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let import = Fen::import(fen_string).unwrap();

        let position = Position::default();
        assert_eq!(import, position);
    }

    #[test]
    pub fn position_test_1() {
        let fen_string = "3nr1k1/1b1q1N2/3B2p1/1p5p/3pP2P/6Q1/1P4P1/5RK1 b - - 0 1";
        let import = Fen::import(fen_string).unwrap();
        let export = Fen::export(&import);
        assert_eq!(fen_string, export);
    }

    #[test]
    pub fn position_test_2() {
        let fen_string = "2kr1b1r/ppqnnppp/2p1p3/3pP1N1/3P4/2PB1P2/PP4PP/1RBQ1RK1 b - - 0 1";
        let import = Fen::import(fen_string).unwrap();
        let export = Fen::export(&import);
        assert_eq!(fen_string, export);
    }

    #[test]
    pub fn position_test_3() {
        let fen_string = "8/3R4/1p1r1p1k/p1p5/P1P2bP1/3P4/5K2/8 w Kq c3 0 1";
        let import = Fen::import(fen_string).unwrap();
        let export = Fen::export(&import);
        assert_eq!(fen_string, export);
    }
}
