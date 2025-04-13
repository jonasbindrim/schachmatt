#[cfg(test)]
mod test_get_game_result {
    use schachmatt::Game;

    #[test]
    fn test_metadata_result_is_used_if_available() {
        let mut game = Game::default();
        assert_eq!(game.get_current_state().game_over_check(), Option::None);
        game.set_metadata("Result", "1-0");
        assert_eq!(
            game.get_game_result(),
            Some(schachmatt::GameResult::Over(schachmatt::PlayerColor::White))
        );
    }

    #[test]
    fn test_position_check_is_used_if_metadata_is_empty() {
        let game = Game::default();
        assert_eq!(game.get_current_state().game_over_check(), Option::None);
        assert_eq!(game.get_game_result(), Option::None);
    }
}
