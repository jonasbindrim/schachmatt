#[cfg(test)]
mod tests {

    use crate::{Board::*, FEN, Turn, data_structures::piece::piece_type::PieceType};

    #[test]
    fn en_passant_test1() {
        let mut game = FEN::import("8/1p6/8/2P5/8/8/8/8 b - - 0 1").unwrap();
        game.turn(&Turn::new(FIELD_B7, FIELD_B5, None)).unwrap();
        assert!(FEN::export(&game) == "8/8/8/1pP5/8/8/8/8 w - b5 0 2");
    }

    #[test]
    fn en_passant_test2() {
        let mut game = FEN::import("8/8/8/1pP5/8/8/8/8 w - b5 0 1").unwrap();
        game.turn(&Turn::new(FIELD_C5, FIELD_B6, None)).unwrap();
        assert!(FEN::export(&game) == "8/8/1P6/8/8/8/8/8 b - - 0 1");
    }

    #[test]
    fn promotion_test_1() {
        let mut game = FEN::import("8/P7/8/8/8/8/8/8 w - - 0 1").unwrap();
        game.turn(&Turn::new(FIELD_A7, FIELD_A8, Some(PieceType::Queen)))
            .unwrap();
        assert!(FEN::export(&game) == "Q7/8/8/8/8/8/8/8 b - - 0 1");
    }

    #[test]
    fn promotion_test_2() {
        let mut game = FEN::import("8/P7/8/8/8/8/8/8 w - - 0 1").unwrap();
        game.turn(&Turn::new(FIELD_A7, FIELD_A8, Some(PieceType::Bishop)))
            .unwrap();
        assert!(FEN::export(&game) == "B7/8/8/8/8/8/8/8 b - - 0 1");
    }

    #[test]
    fn castling_test_1() {
        let mut game = FEN::import("8/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();
        game.turn(&Turn::new(FIELD_E1, FIELD_G1, None)).unwrap();
        assert!(FEN::export(&game) == "8/8/8/8/8/8/8/5RK1 b - - 1 1");
    }

    #[test]
    fn castling_test_2() {
        let mut game = FEN::import("8/8/8/8/8/8/8/R3K3 w Q - 0 1").unwrap();
        game.turn(&Turn::new(FIELD_E1, FIELD_C1, None)).unwrap();
        assert!(FEN::export(&game) == "8/8/8/8/8/8/8/2KR4 b - - 1 1");
    }

    #[test]
    fn castling_test_3() {
        let mut game = FEN::import("4k2r/8/8/8/8/8/8/8 b k - 0 1").unwrap();
        game.turn(&Turn::new(FIELD_E8, FIELD_G8, None)).unwrap();
        assert!(FEN::export(&game) == "5rk1/8/8/8/8/8/8/8 w - - 1 2");
    }

    #[test]
    fn castling_test_4() {
        let mut game = FEN::import("r3k3/8/8/8/8/8/8/8 b q - 0 1").unwrap();
        game.turn(&Turn::new(FIELD_E8, FIELD_C8, None)).unwrap();
        assert!(FEN::export(&game) == "2kr4/8/8/8/8/8/8/8 w - - 1 2");
    }
}
