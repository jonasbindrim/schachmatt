#[cfg(test)]
mod tests {
    use schachmatt::{
        CLASSIC_RULESET,
        Columns::{self, COLUMN_AMOUNT},
        Fen, Field,
        Fields::*,
        NormalTurn, PieceType,
        Rows::{self, ROW_AMOUNT},
        Turn,
    };

    /// Tests the possible moves of the king
    #[test]
    fn king_test() {
        let position = Fen::import("8/8/8/8/8/8/3K4/8 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        let turns = [
            Turn::Normal(NormalTurn::new(FIELD_D2, FIELD_E1, None)),
            Turn::Normal(NormalTurn::new(FIELD_D2, FIELD_E2, None)),
            Turn::Normal(NormalTurn::new(FIELD_D2, FIELD_E3, None)),
            Turn::Normal(NormalTurn::new(FIELD_D2, FIELD_D1, None)),
            Turn::Normal(NormalTurn::new(FIELD_D2, FIELD_D3, None)),
            Turn::Normal(NormalTurn::new(FIELD_D2, FIELD_C1, None)),
            Turn::Normal(NormalTurn::new(FIELD_D2, FIELD_C2, None)),
            Turn::Normal(NormalTurn::new(FIELD_D2, FIELD_C3, None)),
        ];
        assert!(possible_moves.len() == turns.len());
        for item in turns {
            assert!(possible_moves.contains(&item));
        }
    }

    /// Tests the possible horizontal moves of the queen
    #[test]
    fn queen_horizontal_test() {
        let position = Fen::import("8/8/8/8/8/8/3Q4/8 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 23);
        // Test horizontally
        let mut column = Columns::COLUMN_A;
        while column < COLUMN_AMOUNT as u8 {
            let test_turn =
                NormalTurn::new(FIELD_D2, Field::new(column, Rows::ROW_2).unwrap(), None);
            let should_contain = column != Columns::COLUMN_D;
            assert_eq!(
                possible_moves.contains(&Turn::Normal(test_turn)),
                should_contain
            );
            column += 1;
        }
    }

    /// Tests the possible vertical moves of the queen
    #[test]
    fn queen_vertical_test() {
        let position = Fen::import("8/8/8/8/8/8/3Q4/8 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 23);

        let mut row = Rows::ROW_1;
        while row < ROW_AMOUNT as u8 {
            let test_turn =
                NormalTurn::new(FIELD_D2, Field::new(Columns::COLUMN_D, row).unwrap(), None);
            let should_contain = row != Rows::ROW_2;
            assert_eq!(
                possible_moves.contains(&Turn::Normal(test_turn)),
                should_contain
            );
            row += 1;
        }
    }

    /// Tests the possible diagonal moves of the queen
    #[test]
    fn queen_diagonal_bottom_left_to_top_right_test() {
        let position = Fen::import("8/8/8/8/8/8/3Q4/8 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 23);

        let mut lower_right_counter: i8 = -1;
        while lower_right_counter < 5 {
            let (target_column, target_row) = (
                (3 + lower_right_counter) as u8,
                (1 + lower_right_counter) as u8,
            );
            let test_turn = NormalTurn::new(
                FIELD_D2,
                Field::new(target_column, target_row).unwrap(),
                None,
            );
            let should_contain = target_row != Rows::ROW_2 || target_column != Columns::COLUMN_D;
            assert_eq!(
                possible_moves.contains(&Turn::Normal(test_turn)),
                should_contain
            );
            lower_right_counter += 1;
        }
    }

    /// Tests the possible diagonal moves of the queen
    #[test]
    fn queen_diagonal_top_left_to_bottom_right_test() {
        let position = Fen::import("8/8/8/8/8/8/3Q4/8 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 23);

        let mut lower_left_counter: i8 = -3;
        while lower_left_counter < 2 {
            let (target_column, target_row) = (
                (3 + lower_left_counter) as u8,
                (1 - lower_left_counter) as u8,
            );
            let test_turn = NormalTurn::new(
                FIELD_D2,
                Field::new(target_column, target_row).unwrap(),
                None,
            );
            let should_contain = target_row != Rows::ROW_2 || target_column != Columns::COLUMN_D;
            assert_eq!(
                possible_moves.contains(&Turn::Normal(test_turn)),
                should_contain
            );
            lower_left_counter += 1;
        }
    }

    /// Test the possible vertical moves of the rook
    #[test]
    fn rook_vertical_test() {
        let position = Fen::import("8/8/8/8/8/8/3R4/8 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 14);
        let mut column = Columns::COLUMN_A;
        while column < COLUMN_AMOUNT as u8 {
            let test_turn =
                NormalTurn::new(FIELD_D2, Field::new(column, Rows::ROW_2).unwrap(), None);
            let should_contain = column != Columns::COLUMN_D;
            assert_eq!(
                possible_moves.contains(&Turn::Normal(test_turn)),
                should_contain
            );
            column += 1;
        }
    }

    /// Tests the possible horizontal moves of the rook
    #[test]
    fn rook_horizontal_test() {
        let position = Fen::import("8/8/8/8/8/8/3R4/8 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 14);
        let mut row = Rows::ROW_1;
        while row < ROW_AMOUNT as u8 {
            let test_turn =
                NormalTurn::new(FIELD_D2, Field::new(Columns::COLUMN_D, row).unwrap(), None);
            let should_contain = row != Rows::ROW_2;
            assert_eq!(
                possible_moves.contains(&Turn::Normal(test_turn)),
                should_contain
            );
            row += 1;
        }
    }

    /// Tests the possible moves of the bishop
    #[test]
    fn bishop_bottom_left_to_top_right_test() {
        let position = Fen::import("8/8/8/8/8/8/3B4/8 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 9);

        let mut lowleft_to_topright_counter: i8 = -1;
        while lowleft_to_topright_counter < 5 {
            let (target_column, target_row) = (
                (3 + lowleft_to_topright_counter) as u8,
                (1 + lowleft_to_topright_counter) as u8,
            );
            let test_turn = NormalTurn::new(
                FIELD_D2,
                Field::new(target_column, target_row).unwrap(),
                None,
            );
            let should_contain = target_row != Rows::ROW_2 || target_column != Columns::COLUMN_D;
            assert_eq!(
                possible_moves.contains(&Turn::Normal(test_turn)),
                should_contain
            );
            lowleft_to_topright_counter += 1;
        }
    }

    /// Tests the possible moves of the bishop
    #[test]
    fn bishop_top_right_to_bottom_left_test() {
        let position = Fen::import("8/8/8/8/8/8/3B4/8 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 9);

        let mut lowright_to_topleft_counter: i8 = -3;
        while lowright_to_topleft_counter < 2 {
            let (target_column, target_row) = (
                (3 + lowright_to_topleft_counter) as u8,
                (1 - lowright_to_topleft_counter) as u8,
            );
            let test_turn = NormalTurn::new(
                FIELD_D2,
                Field::new(target_column, target_row).unwrap(),
                None,
            );
            let should_contain = target_row != Rows::ROW_2 || target_column != Columns::COLUMN_D;
            assert_eq!(
                possible_moves.contains(&Turn::Normal(test_turn)),
                should_contain
            );
            lowright_to_topleft_counter += 1;
        }
    }

    /// Tests the possible moves of the knight
    #[test]
    fn knight_test() {
        let position = Fen::import("8/8/8/8/8/2N5/8/8 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        let turns = [
            Turn::Normal(NormalTurn::new(FIELD_C3, FIELD_A2, None)),
            Turn::Normal(NormalTurn::new(FIELD_C3, FIELD_B1, None)),
            Turn::Normal(NormalTurn::new(FIELD_C3, FIELD_A4, None)),
            Turn::Normal(NormalTurn::new(FIELD_C3, FIELD_D1, None)),
            Turn::Normal(NormalTurn::new(FIELD_C3, FIELD_B5, None)),
            Turn::Normal(NormalTurn::new(FIELD_C3, FIELD_E2, None)),
            Turn::Normal(NormalTurn::new(FIELD_C3, FIELD_D5, None)),
            Turn::Normal(NormalTurn::new(FIELD_C3, FIELD_E4, None)),
        ];
        assert!(possible_moves.len() == turns.len());
        for item in turns {
            assert!(possible_moves.contains(&item));
        }
    }

    /// Tests the possible moves of the white pawns
    #[test]
    fn pawn_white_test() {
        let position = Fen::import("8/8/8/8/8/3P4/8/8 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 1);
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_D3, FIELD_D4, None));
        assert!(possible_moves.contains(&test_turn));
    }

    /// Tests the possible moves of the white pawns
    #[test]
    fn pawn_white_double_advancement_test() {
        let position = Fen::import("8/8/8/8/8/8/3P4/8 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        let test_turn = [
            Turn::Normal(NormalTurn::new(FIELD_D2, FIELD_D3, None)),
            Turn::Normal(NormalTurn::new(FIELD_D2, FIELD_D4, None)),
        ];
        assert!(possible_moves.len() == test_turn.len());
        assert!(possible_moves.contains(&test_turn[0]));
        assert!(possible_moves.contains(&test_turn[1]));
    }

    /// Tests the possible moves of the black pawn
    #[test]
    fn pawn_black_test() {
        let position = Fen::import("8/8/8/8/8/3p4/8/8 b - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 1);
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_D3, FIELD_D2, None));
        assert!(possible_moves.contains(&test_turn));
    }

    /// Tests the possible moves of the black pawn
    #[test]
    fn pawn_black_double_advancement_test() {
        let position = Fen::import("8/3p4/8/8/8/8/8/8 b - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        let test_turn = [
            Turn::Normal(NormalTurn::new(FIELD_D7, FIELD_D6, None)),
            Turn::Normal(NormalTurn::new(FIELD_D7, FIELD_D5, None)),
        ];
        assert!(possible_moves.len() == test_turn.len());
        assert!(possible_moves.contains(&test_turn[0]));
        assert!(possible_moves.contains(&test_turn[1]));
    }

    /// Tests the possible moves of the white pawns
    #[test]
    fn pawn_white_takes_test() {
        let position = Fen::import("8/8/8/8/8/2pp4/3P4/8 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 1);
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_D2, FIELD_C3, None));
        assert!(possible_moves.contains(&test_turn));
    }

    /// Tests the possible moves of the black pawn
    #[test]
    fn pawn_black_takes_test() {
        let position = Fen::import("8/8/8/8/8/2p5/2PP4/8 b - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 1);
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_C3, FIELD_D2, None));
        assert!(possible_moves.contains(&test_turn));
    }

    /// Tests if temporary illegal moves are detected correctly
    #[test]
    fn temporary_illegal_test() {
        let position = Fen::import("8/8/8/8/8/3rrr2/R7/3PKP2 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 1);
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_A2, FIELD_E2, None));
        let test_move = possible_moves.first().unwrap();
        assert!(*test_move == test_turn);
    }

    /// Tests if fully illegal moves are detected correctly
    #[test]
    fn fully_illegal_test() {
        let position = Fen::import("8/8/8/3P4/3P4/8/8/8 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 1);
        let test_turn = Turn::Normal(NormalTurn::new(FIELD_D5, FIELD_D6, None));
        let test_move = possible_moves.first().unwrap();
        assert!(*test_move == test_turn);
    }

    /// Tests if en-passant is possible
    #[test]
    fn pawn_enpassant_test() {
        let position = Fen::import("8/8/8/Pp6/8/8/8/8 w - b5 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        let test_turn = [
            Turn::Normal(NormalTurn::new(FIELD_A5, FIELD_A6, None)),
            Turn::Normal(NormalTurn::new(FIELD_A5, FIELD_B6, None)),
        ];
        assert!(possible_moves.len() == test_turn.len());
        assert!(possible_moves.contains(&test_turn[0]));
        assert!(possible_moves.contains(&test_turn[1]));
    }

    #[test]
    fn promotion_test_1() {
        let position = Fen::import("8/P7/8/8/8/8/8/8 w - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        let test_turn = [
            Turn::Normal(NormalTurn::new(FIELD_A7, FIELD_A8, Some(PieceType::Queen))),
            Turn::Normal(NormalTurn::new(FIELD_A7, FIELD_A8, Some(PieceType::Rook))),
            Turn::Normal(NormalTurn::new(FIELD_A7, FIELD_A8, Some(PieceType::Bishop))),
            Turn::Normal(NormalTurn::new(FIELD_A7, FIELD_A8, Some(PieceType::Knight))),
        ];
        assert!(possible_moves.len() == test_turn.len());
        for turns in test_turn {
            assert!(possible_moves.contains(&turns));
        }
    }

    #[test]
    fn promotion_test_2() {
        let position = Fen::import("8/8/8/8/8/8/p7/8 b - - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        let test_turn = [
            Turn::Normal(NormalTurn::new(FIELD_A2, FIELD_A1, Some(PieceType::Queen))),
            Turn::Normal(NormalTurn::new(FIELD_A2, FIELD_A1, Some(PieceType::Rook))),
            Turn::Normal(NormalTurn::new(FIELD_A2, FIELD_A1, Some(PieceType::Bishop))),
            Turn::Normal(NormalTurn::new(FIELD_A2, FIELD_A1, Some(PieceType::Knight))),
        ];
        assert!(possible_moves.len() == test_turn.len());
        for turns in test_turn {
            assert!(possible_moves.contains(&turns));
        }
    }

    #[test]
    fn castling_test_1() {
        let position = Fen::import("8/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 15);
    }

    #[test]
    fn castling_test_2() {
        let position = Fen::import("8/8/8/8/8/8/8/R3K3 w Q - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 16);
    }

    #[test]
    fn castling_test_3() {
        let position = Fen::import("4k2r/8/8/8/8/8/8/8 b k - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 15);
    }

    #[test]
    fn castling_test_4() {
        let position = Fen::import("r3k3/8/8/8/8/8/8/8 b q - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 16);
    }

    #[test]
    fn castling_test_5() {
        let position = Fen::import("r3k3/8/8/8/8/8/8/2R5 b q - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 15);
    }

    #[test]
    fn castling_test_6() {
        let position = Fen::import("2r5/8/8/8/8/8/8/R3K3 w Q - 0 1").unwrap();
        let possible_moves = CLASSIC_RULESET.get_possible_turns(&position);
        assert!(possible_moves.len() == 15);
    }
}
