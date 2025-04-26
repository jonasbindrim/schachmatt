#[cfg(test)]
mod position_turn {
    use schachmatt::{Field, Game, Turn};

    #[test]
    fn execute_legal_move_test() {
        let mut default_game = Game::default();
        let possible_moves = default_game.get_possible_turns();
        let legal_move = possible_moves.first().unwrap();
        default_game.execute_turn(*legal_move).expect("Unexpected error");
    }

    #[test]
    fn execute_illegal_move_test() {
        let mut default_game = Game::default();
        let illegal_move = Turn::new(
            Field::new_from_string("a1").unwrap(),
            Field::new_from_string("b1").unwrap(),
            None,
        );

        default_game
            .execute_turn(illegal_move)
            .expect_err("Unexpected ok");
    }
}
