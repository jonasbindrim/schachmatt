#[cfg(test)]
mod test_game_over_check {
    use schachmatt::{CLASSIC_RULESET, Fen, GameResult, PlayerColor, Position};

    #[test]
    fn test_game_over_check_1() {
        let position = Position::default();
        let result = CLASSIC_RULESET.game_over_check(&position);
        assert!(result.is_none());
    }

    #[test]
    fn test_game_over_check_2() {
        let position = Fen::import("8/8/8/8/8/8/q7/Kq6 w - - 0 1").unwrap();
        let result = CLASSIC_RULESET.game_over_check(&position);
        assert!(matches!(
            result.unwrap(),
            GameResult::Decisive(PlayerColor::Black)
        ));
    }

    #[test]
    fn test_game_over_check_3() {
        let position = Fen::import("8/8/8/8/8/8/8/Kq6 w - - 0 1").unwrap();
        let result = CLASSIC_RULESET.game_over_check(&position);
        assert!(result.is_none());
    }

    #[test]
    fn test_40_move_counter() {
        let position = Fen::import("7k/8/8/8/8/8/8/1K6 b - - 50 1").unwrap();
        assert!(matches!(
            CLASSIC_RULESET.game_over_check(&position).unwrap(),
            GameResult::Draw
        ));
    }

    #[test]
    fn test_insufficient_material_1() {
        let position = Fen::import("8/k7/8/8/8/8/8/KP6 w - - 10 1").unwrap();
        assert!(CLASSIC_RULESET.game_over_check(&position).is_none());
    }

    #[test]
    fn test_insufficient_material_2() {
        let position = Fen::import("8/k7/8/8/8/8/8/K7 w - - 10 1").unwrap();
        assert!(matches!(
            CLASSIC_RULESET.game_over_check(&position).unwrap(),
            GameResult::Draw
        ));
    }

    #[test]
    fn test_insufficient_material_3() {
        let position = Fen::import("8/k7/8/8/8/8/8/KB6 w - - 10 1").unwrap();
        assert!(matches!(
            CLASSIC_RULESET.game_over_check(&position).unwrap(),
            GameResult::Draw
        ));
    }

    #[test]
    fn test_insufficient_material_4() {
        let position = Fen::import("8/k7/8/8/8/8/8/KBB5 w - - 10 1").unwrap();
        assert!(CLASSIC_RULESET.game_over_check(&position).is_none());
    }

    #[test]
    fn test_insufficient_material_5() {
        let position = Fen::import("8/k7/8/8/8/8/8/KN6 w - - 10 1").unwrap();
        assert!(CLASSIC_RULESET.game_over_check(&position).unwrap() == GameResult::Draw);
    }

    #[test]
    fn test_insufficient_material_6() {
        let position = Fen::import("8/k7/8/8/8/8/8/KNN5 w - - 10 1").unwrap();
        assert!(CLASSIC_RULESET.game_over_check(&position).is_none());
    }

    #[test]
    fn test_insufficient_material_7() {
        let position = Fen::import("8/kn6/8/8/8/8/8/KN6 w - - 10 1").unwrap();
        assert!(CLASSIC_RULESET.game_over_check(&position).is_none());
    }

    #[test]
    fn test_insufficient_material_8() {
        let position = Fen::import("8/kb6/8/8/8/8/8/KN6 w - - 10 1").unwrap();
        assert!(CLASSIC_RULESET.game_over_check(&position).is_none());
    }
}
