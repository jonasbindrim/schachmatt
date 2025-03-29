#[cfg(test)]
mod position_tests {
    use schachmatt::{Field, Position, Turn};

    #[test]
    fn execute_legal_move_test() {
        let mut default_position = Position::default();
        let possible_moves = default_position.get_possible_moves();
        let legal_move = possible_moves.first().unwrap();
        default_position.turn(legal_move).expect("Unexpected error");
    }

    #[test]
    fn execute_illegal_move_test() {
        let mut default_position = Position::default();
        let illegal_move = Turn::new(
            Field::new_from_string("a1").unwrap(),
            Field::new_from_string("b1").unwrap(),
            None,
        );

        default_position
            .turn(&illegal_move)
            .expect_err("Unexpected ok");
    }
}
