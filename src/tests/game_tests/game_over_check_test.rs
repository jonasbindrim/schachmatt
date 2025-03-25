#[cfg(test)]
mod tests {

    use crate::{FEN, Field, GameResult, PlayerColor, Position, Turn};

    /// Tests whether the conversion from a move into the algebraic chess notation works as idented
    #[test]
    fn test_game_over_check_1() {
        let game = Position::new();
        let result = game.game_over_check();
        assert!(result.is_none());
    }

    #[test]
    fn test_game_over_check_2() {
        let game = FEN::import("8/8/8/8/8/8/q7/Kq6 w - - 0 1").unwrap();
        let result = game.game_over_check();
        assert!(matches!(
            result.unwrap(),
            GameResult::Over(PlayerColor::Black)
        ));
    }

    #[test]
    fn test_game_over_check_3() {
        let game = FEN::import("8/8/8/8/8/8/8/Kq6 w - - 0 1").unwrap();
        let result = game.game_over_check();
        assert!(result.is_none());
    }

    #[test]
    fn test_40_move_counter() {
        let mut game = FEN::import("7k/8/8/8/8/8/8/K7 w - - 49 1").unwrap();
        let turn = Turn {
            from: Field { column: 0, row: 0 },
            to: Field { column: 1, row: 0 },
            promotion: None,
        };
        game.turn(&turn).unwrap();
        assert!(matches!(game.game_over_check().unwrap(), GameResult::Draw));
        assert_eq!(FEN::export(&game), "7k/8/8/8/8/8/8/1K6 b - - 50 1");
    }

    #[test]
    fn test_insufficient_material_1() {
        let game = FEN::import("8/k7/8/8/8/8/8/KP6 w - - 10 1").unwrap();
        assert!(game.game_over_check().is_none());
    }

    #[test]
    fn test_insufficient_material_2() {
        let game = FEN::import("8/k7/8/8/8/8/8/K7 w - - 10 1").unwrap();
        assert!(matches!(game.game_over_check().unwrap(), GameResult::Draw));
    }

    #[test]
    fn test_insufficient_material_3() {
        let game = FEN::import("8/k7/8/8/8/8/8/KB6 w - - 10 1").unwrap();
        assert!(matches!(game.game_over_check().unwrap(), GameResult::Draw));
    }

    #[test]
    fn test_insufficient_material_4() {
        let game = FEN::import("8/k7/8/8/8/8/8/KBB5 w - - 10 1").unwrap();
        assert!(game.game_over_check().is_none());
    }

    #[test]
    fn test_insufficient_material_5() {
        let game = FEN::import("8/k7/8/8/8/8/8/KN6 w - - 10 1").unwrap();
        assert!(matches!(game.game_over_check().unwrap(), GameResult::Draw));
    }

    #[test]
    fn test_insufficient_material_6() {
        let game = FEN::import("8/k7/8/8/8/8/8/KNN5 w - - 10 1").unwrap();
        assert!(game.game_over_check().is_none());
    }

    #[test]
    fn test_insufficient_material_7() {
        let game = FEN::import("8/kn6/8/8/8/8/8/KN6 w - - 10 1").unwrap();
        assert!(game.game_over_check().is_none());
    }

    #[test]
    fn test_insufficient_material_8() {
        let game = FEN::import("8/kb6/8/8/8/8/8/KN6 w - - 10 1").unwrap();
        assert!(game.game_over_check().is_none());
    }
}
