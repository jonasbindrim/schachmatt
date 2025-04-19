#[cfg(test)]
mod game_result {
    mod to_string {
        use schachmatt::{GameResult, PlayerColor};

        #[test]
        fn test_to_string_draw() {
            let draw = GameResult::Draw;
            assert_eq!(GameResult::to_string(Some(&draw)), "1/2-1/2");
        }

        #[test]
        fn test_to_string_black() {
            let black = GameResult::Over(PlayerColor::Black);
            assert_eq!(GameResult::to_string(Some(&black)), "0-1");
        }

        #[test]
        fn test_to_string_white() {
            let white = GameResult::Over(PlayerColor::White);
            assert_eq!(GameResult::to_string(Some(&white)), "1-0");
        }

        #[test]
        fn test_to_string_none() {
            assert_eq!(GameResult::to_string(None), "*");
        }
    }

    mod from_string {
        use schachmatt::{GameResult, PlayerColor};

        #[test]
        fn test_from_string_draw() {
            let draw = GameResult::from_string("1/2-1/2");
            assert_eq!(draw, Some(GameResult::Draw));
        }

        #[test]
        fn test_from_string_black() {
            let black = GameResult::from_string("0-1");
            assert_eq!(black, Some(GameResult::Over(PlayerColor::Black)));
        }

        #[test]
        fn test_from_string_white() {
            let white = GameResult::from_string("1-0");
            assert_eq!(white, Some(GameResult::Over(PlayerColor::White)));
        }

        #[test]
        fn test_from_string_invalid() {
            let invalid = GameResult::from_string("invalid");
            assert_eq!(invalid, None);
        }
    }
}
