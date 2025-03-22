#[cfg(test)]
mod tests {
    use crate::{data_structures::piece::piece_type::PieceType, Field, Turn, FEN};

    /// Tests the possible moves of the king
    #[test]
    fn king_test() {
        let game = FEN::import("8/8/8/8/8/8/3K4/8 w - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        let turns = [
            Turn {
                from: Field { column: 3, row: 1 },
                to: Field { column: 4, row: 0 },
                promotion: None,
            },
            Turn {
                from: Field { column: 3, row: 1 },
                to: Field { column: 4, row: 1 },
                promotion: None,
            },
            Turn {
                from: Field { column: 3, row: 1 },
                to: Field { column: 4, row: 2 },
                promotion: None,
            },
            Turn {
                from: Field { column: 3, row: 1 },
                to: Field { column: 3, row: 0 },
                promotion: None,
            },
            Turn {
                from: Field { column: 3, row: 1 },
                to: Field { column: 3, row: 2 },
                promotion: None,
            },
            Turn {
                from: Field { column: 3, row: 1 },
                to: Field { column: 2, row: 0 },
                promotion: None,
            },
            Turn {
                from: Field { column: 3, row: 1 },
                to: Field { column: 2, row: 1 },
                promotion: None,
            },
            Turn {
                from: Field { column: 3, row: 1 },
                to: Field { column: 2, row: 2 },
                promotion: None,
            },
        ];
        assert!(possible_moves.len() == turns.len());
        for item in turns {
            assert!(possible_moves.contains(&item));
        }
    }

    /// Tests the possible moves of the queen
    #[test]
    fn queen_test() {
        let game = FEN::import("8/8/8/8/8/8/3Q4/8 w - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 23);
        // Test horizontally
        let mut column = 0;
        while column < 8 {
            let test_turn = Turn {
                from: Field { column: 3, row: 1 },
                to: Field { column, row: 1 },
                promotion: None,
            };
            if test_turn.from != test_turn.to {
                assert!(possible_moves.contains(&test_turn));
            }
            column += 1;
        }
        let mut row = 0;
        while row < 8 {
            let test_turn = Turn {
                from: Field { column: 3, row: 1 },
                to: Field { column: 3, row },
                promotion: None,
            };
            if test_turn.from != test_turn.to {
                assert!(possible_moves.contains(&test_turn));
            }
            row += 1;
        }
        let mut lower_right_counter = -1;
        while lower_right_counter < 5 {
            let test_turn = Turn {
                from: Field { column: 3, row: 1 },
                to: Field {
                    column: u8::try_from(3 + lower_right_counter).unwrap(),
                    row: u8::try_from(1 + lower_right_counter).unwrap(),
                },
                promotion: None,
            };
            if test_turn.from != test_turn.to {
                assert!(possible_moves.contains(&test_turn));
            }
            lower_right_counter += 1;
        }

        let mut lower_left_counter = -3;
        while lower_left_counter < 2 {
            let test_turn = Turn {
                from: Field { column: 3, row: 1 },
                to: Field {
                    column: u8::try_from(3 + lower_left_counter).unwrap(),
                    row: u8::try_from(1 - lower_left_counter).unwrap(),
                },
                promotion: None,
            };
            if test_turn.from != test_turn.to {
                assert!(possible_moves.contains(&test_turn));
            }
            lower_left_counter += 1;
        }
    }

    /// Tests the possible moves of the rook
    #[test]
    fn rook_test() {
        let game = FEN::import("8/8/8/8/8/8/3R4/8 w - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 14);
        // Test horizontally
        let mut column = 0;
        while column < 8 {
            let test_turn = Turn {
                from: Field { column: 3, row: 1 },
                to: Field { column, row: 1 },
                promotion: None,
            };
            if test_turn.from != test_turn.to {
                assert!(possible_moves.contains(&test_turn));
            }
            column += 1;
        }
        let mut row = 0;
        while row < 8 {
            let test_turn = Turn {
                from: Field { column: 3, row: 1 },
                to: Field { column: 3, row },
                promotion: None,
            };
            if test_turn.from != test_turn.to {
                assert!(possible_moves.contains(&test_turn));
            }
            row += 1;
        }
    }

    /// Tests the possible moves of the knight
    #[test]
    fn knight_test() {
        let game = FEN::import("8/8/8/8/8/2N5/8/8 w - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        let turns = [
            Turn {
                from: Field { column: 2, row: 2 },
                to: Field { column: 0, row: 1 },
                promotion: None,
            },
            Turn {
                from: Field { column: 2, row: 2 },
                to: Field { column: 1, row: 0 },
                promotion: None,
            },
            Turn {
                from: Field { column: 2, row: 2 },
                to: Field { column: 0, row: 3 },
                promotion: None,
            },
            Turn {
                from: Field { column: 2, row: 2 },
                to: Field { column: 3, row: 0 },
                promotion: None,
            },
            Turn {
                from: Field { column: 2, row: 2 },
                to: Field { column: 1, row: 4 },
                promotion: None,
            },
            Turn {
                from: Field { column: 2, row: 2 },
                to: Field { column: 4, row: 1 },
                promotion: None,
            },
            Turn {
                from: Field { column: 2, row: 2 },
                to: Field { column: 3, row: 4 },
                promotion: None,
            },
            Turn {
                from: Field { column: 2, row: 2 },
                to: Field { column: 4, row: 3 },
                promotion: None,
            },
        ];
        assert!(possible_moves.len() == turns.len());
        for item in turns {
            assert!(possible_moves.contains(&item));
        }
    }

    /// Tests the possible moves of the white pawns
    #[test]
    fn pawn_white_test() {
        let game = FEN::import("8/8/8/8/8/3P4/8/8 w - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 1);
        let test_turn = Turn {
            from: Field { column: 3, row: 2 },
            to: Field { column: 3, row: 3 },
            promotion: None,
        };
        assert!(possible_moves.contains(&test_turn));
    }

    /// Tests the possible moves of the white pawns
    #[test]
    fn pawn_white_double_advancement_test() {
        let game = FEN::import("8/8/8/8/8/8/3P4/8 w - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        let test_turn = [
            Turn {
                from: Field { column: 3, row: 1 },
                to: Field { column: 3, row: 2 },
                promotion: None,
            },
            Turn {
                from: Field { column: 3, row: 1 },
                to: Field { column: 3, row: 3 },
                promotion: None,
            },
        ];
        assert!(possible_moves.len() == test_turn.len());
        assert!(possible_moves.contains(&test_turn[0]));
        assert!(possible_moves.contains(&test_turn[1]));
    }

    /// Tests the possible moves of the bishop
    #[test]
    fn bishop_test() {
        let game = FEN::import("8/8/8/8/8/8/3B4/8 w - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 9);

        let mut lower_right_counter = -1;
        while lower_right_counter < 5 {
            let test_turn = Turn {
                from: Field { column: 3, row: 1 },
                to: Field {
                    column: u8::try_from(3 + lower_right_counter).unwrap(),
                    row: u8::try_from(1 + lower_right_counter).unwrap(),
                },
                promotion: None,
            };
            if test_turn.from != test_turn.to {
                assert!(possible_moves.contains(&test_turn));
            }
            lower_right_counter += 1;
        }

        let mut lower_left_counter = -3;
        while lower_left_counter < 2 {
            let test_turn = Turn {
                from: Field { column: 3, row: 1 },
                to: Field {
                    column: u8::try_from(3 + lower_left_counter).unwrap(),
                    row: u8::try_from(1 - lower_left_counter).unwrap(),
                },
                promotion: None,
            };
            if test_turn.from != test_turn.to {
                assert!(possible_moves.contains(&test_turn));
            }
            lower_left_counter += 1;
        }
    }

    /// Tests the possible moves of the black pawn
    #[test]
    fn pawn_black_test() {
        let game = FEN::import("8/8/8/8/8/3p4/8/8 b - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 1);
        let test_turn = Turn {
            from: Field { column: 3, row: 2 },
            to: Field { column: 3, row: 1 },
            promotion: None,
        };
        assert!(possible_moves.contains(&test_turn));
    }

    /// Tests the possible moves of the black pawn
    #[test]
    fn pawn_black_double_advancement_test() {
        let game = FEN::import("8/3p4/8/8/8/8/8/8 b - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        let test_turn = [
            Turn {
                from: Field { column: 3, row: 6 },
                to: Field { column: 3, row: 5 },
                promotion: None,
            },
            Turn {
                from: Field { column: 3, row: 6 },
                to: Field { column: 3, row: 4 },
                promotion: None,
            },
        ];
        assert!(possible_moves.len() == test_turn.len());
        assert!(possible_moves.contains(&test_turn[0]));
        assert!(possible_moves.contains(&test_turn[1]));
    }

    /// Tests the possible moves of the white pawns
    #[test]
    fn pawn_white_takes_test() {
        let game = FEN::import("8/8/8/8/8/2pp4/3P4/8 w - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 1);
        let test_turn = Turn {
            from: Field { column: 3, row: 1 },
            to: Field { column: 2, row: 2 },
            promotion: None,
        };
        assert!(possible_moves.contains(&test_turn));
    }

    /// Tests the possible moves of the black pawn
    #[test]
    fn pawn_black_takes_test() {
        let game = FEN::import("8/8/8/8/8/2p5/2PP4/8 b - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 1);
        let test_turn = Turn {
            from: Field { column: 2, row: 2 },
            to: Field { column: 3, row: 1 },
            promotion: None,
        };
        assert!(possible_moves.contains(&test_turn));
    }

    /// Tests if temporary illegal moves are detected correctly
    #[test]
    fn temporary_illegal_test() {
        let game = FEN::import("8/8/8/8/8/3rrr2/R7/3PKP2 w - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 1);
        let test_turn = Turn {
            from: Field { column: 0, row: 1 },
            to: Field { column: 4, row: 1 },
            promotion: None,
        };
        let test_move = possible_moves.get(0).unwrap();
        assert!(*test_move == test_turn);
    }

    /// Tests if fully illegal moves are detected correctly
    #[test]
    fn fully_illegal_test() {
        let game = FEN::import("8/8/8/3P4/3P4/8/8/8 w - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 1);
        let test_turn = Turn {
            from: Field { column: 3, row: 4 },
            to: Field { column: 3, row: 5 },
            promotion: None,
        };
        let test_move = possible_moves.get(0).unwrap();
        assert!(*test_move == test_turn);
    }

    /// Tests if en-passant is possible
    #[test]
    fn pawn_enpassant_test() {
        let game = FEN::import("8/8/8/Pp6/8/8/8/8 w - b5 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        let test_turn = [
            Turn {
                from: Field { column: 0, row: 4 },
                to: Field { column: 0, row: 5 },
                promotion: None,
            },
            Turn {
                from: Field { column: 0, row: 4 },
                to: Field { column: 1, row: 5 },
                promotion: None,
            },
        ];
        assert!(possible_moves.len() == test_turn.len());
        assert!(possible_moves.contains(&test_turn[0]));
        assert!(possible_moves.contains(&test_turn[1]));
    }

    #[test]
    fn promotion_test_1() {
        let game = FEN::import("8/P7/8/8/8/8/8/8 w - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        let test_turn = [
            Turn {
                from: Field { column: 0, row: 6 },
                to: Field { column: 0, row: 7 },
                promotion: Some(PieceType::Queen),
            },
            Turn {
                from: Field { column: 0, row: 6 },
                to: Field { column: 0, row: 7 },
                promotion: Some(PieceType::Rook),
            },
            Turn {
                from: Field { column: 0, row: 6 },
                to: Field { column: 0, row: 7 },
                promotion: Some(PieceType::Bishop),
            },
            Turn {
                from: Field { column: 0, row: 6 },
                to: Field { column: 0, row: 7 },
                promotion: Some(PieceType::Knight),
            },
        ];
        assert!(possible_moves.len() == test_turn.len());
        for turns in test_turn {
            assert!(possible_moves.contains(&turns));
        }
    }

    #[test]
    fn promotion_test_2() {
        let game = FEN::import("8/8/8/8/8/8/p7/8 b - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        let test_turn = [
            Turn {
                from: Field { column: 0, row: 1 },
                to: Field { column: 0, row: 0 },
                promotion: Some(PieceType::Queen),
            },
            Turn {
                from: Field { column: 0, row: 1 },
                to: Field { column: 0, row: 0 },
                promotion: Some(PieceType::Rook),
            },
            Turn {
                from: Field { column: 0, row: 1 },
                to: Field { column: 0, row: 0 },
                promotion: Some(PieceType::Bishop),
            },
            Turn {
                from: Field { column: 0, row: 1 },
                to: Field { column: 0, row: 0 },
                promotion: Some(PieceType::Knight),
            },
        ];
        assert!(possible_moves.len() == test_turn.len());
        for turns in test_turn {
            assert!(possible_moves.contains(&turns));
        }
    }

    #[test]
    fn castling_test_1() {
        let game = FEN::import("8/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 15);
    }

    #[test]
    fn castling_test_2() {
        let game = FEN::import("8/8/8/8/8/8/8/R3K3 w Q - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 16);
    }

    #[test]
    fn castling_test_3() {
        let game = FEN::import("4k2r/8/8/8/8/8/8/8 b k - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 15);
    }

    #[test]
    fn castling_test_4() {
        let game = FEN::import("r3k3/8/8/8/8/8/8/8 b q - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 16);
    }

    #[test]
    fn castling_test_5() {
        let game = FEN::import("r3k3/8/8/8/8/8/8/2R5 b q - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 15);
    }

    #[test]
    fn castling_test_6() {
        let game = FEN::import("2r5/8/8/8/8/8/8/R3K3 w Q - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 15);
    }
}
