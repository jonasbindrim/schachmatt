#[cfg(test)]
mod game_metadata_tests {
    use schachmatt::{FEN, Game};

    #[test]
    fn metadata_is_empty_initially() {
        let game = Game::default();
        assert_eq!(game.get_metadata_keys().len(), 0);
    }

    #[test]
    fn metadata_for_costum_position_contains_fen() {
        let custom_position = FEN::import("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1").unwrap();
        let game = Game::new(custom_position);
        let metadata_keys = game.get_metadata_keys();
        assert_eq!(metadata_keys.len(), 1);
        assert!(metadata_keys.contains(&String::from("Fen")));
        assert_eq!(
            game.get_metadata("Fen").unwrap(),
            "rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn metadata_is_added_correctly() {
        let mut game = Game::default();
        assert_eq!(game.get_metadata_keys().len(), 0);
        game.set_metadata("Testkey", "Testvalue");
        let metadata_keys = game.get_metadata_keys();
        assert_eq!(metadata_keys.len(), 1);
        assert!(metadata_keys.contains(&String::from("Testkey")));
        assert_eq!(game.get_metadata("Testkey").unwrap(), "Testvalue");
    }

    #[test]
    fn metadata_is_overridden_correctly() {
        let custom_position = FEN::import("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1").unwrap();
        let mut game = Game::new(custom_position);
        assert_eq!(
            game.get_metadata("Fen").unwrap(),
            "rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1"
        );
        game.set_metadata("Fen", "Testvalue");
        assert_eq!(game.get_metadata("Fen").unwrap(), "Testvalue");
    }
}
