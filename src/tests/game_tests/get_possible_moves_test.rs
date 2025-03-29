#[cfg(test)]
mod tests {
    use crate::{
        Board::{self, *},
        FEN, Field, Turn,
        data_structures::piece::piece_type::PieceType,
        position::position_struct::{COLUMN_AMOUNT, ROW_AMOUNT},
    };

    /// Tests the possible moves of the king
    #[test]
    fn king_test() {
        let game = FEN::import("8/8/8/8/8/8/3K4/8 w - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        let turns = [
            Turn {
                from: FIELD_D2,
                to: FIELD_E1,
                promotion: None,
            },
            Turn {
                from: FIELD_D2,
                to: FIELD_E2,
                promotion: None,
            },
            Turn {
                from: FIELD_D2,
                to: FIELD_E3,
                promotion: None,
            },
            Turn {
                from: FIELD_D2,
                to: FIELD_D1,
                promotion: None,
            },
            Turn {
                from: FIELD_D2,
                to: FIELD_D3,
                promotion: None,
            },
            Turn {
                from: FIELD_D2,
                to: FIELD_C1,
                promotion: None,
            },
            Turn {
                from: FIELD_D2,
                to: FIELD_C2,
                promotion: None,
            },
            Turn {
                from: FIELD_D2,
                to: FIELD_C3,
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
        let mut column = Board::COLUMN_A;
        while column < COLUMN_AMOUNT as u8 {
            let test_turn = Turn {
                from: FIELD_D2,
                to: Field::new(column, Board::ROW_2).unwrap(),
                promotion: None,
            };
            if test_turn.from != test_turn.to {
                assert!(possible_moves.contains(&test_turn));
            }
            column += 1;
        }
        let mut row = Board::ROW_1;
        while row < ROW_AMOUNT as u8 {
            let test_turn = Turn {
                from: FIELD_D2,
                to: Field::new(Board::COLUMN_D, row).unwrap(),
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
                from: FIELD_D2,
                to: Field::new_from_usize(
                    (3 + lower_right_counter) as usize,
                    (1 + lower_right_counter) as usize,
                )
                .unwrap(),
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
                from: FIELD_D2,
                to: Field::new_from_usize(
                    (3 + lower_left_counter) as usize,
                    (1 - lower_left_counter) as usize,
                )
                .unwrap(),
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
        let mut column = Board::COLUMN_A;
        while column < COLUMN_AMOUNT as u8 {
            let test_turn = Turn {
                from: FIELD_D2,
                to: Field::new(column, Board::ROW_2).unwrap(),
                promotion: None,
            };
            if test_turn.from != test_turn.to {
                assert!(possible_moves.contains(&test_turn));
            }
            column += 1;
        }
        let mut row = Board::ROW_1;
        while row < ROW_AMOUNT as u8 {
            let test_turn = Turn {
                from: FIELD_D2,
                to: Field::new(Board::COLUMN_D, row).unwrap(),
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
                from: FIELD_C3,
                to: FIELD_A2,
                promotion: None,
            },
            Turn {
                from: FIELD_C3,
                to: FIELD_B1,
                promotion: None,
            },
            Turn {
                from: FIELD_C3,
                to: FIELD_A4,
                promotion: None,
            },
            Turn {
                from: FIELD_C3,
                to: FIELD_D1,
                promotion: None,
            },
            Turn {
                from: FIELD_C3,
                to: FIELD_B5,
                promotion: None,
            },
            Turn {
                from: FIELD_C3,
                to: FIELD_E2,
                promotion: None,
            },
            Turn {
                from: FIELD_C3,
                to: FIELD_D5,
                promotion: None,
            },
            Turn {
                from: FIELD_C3,
                to: FIELD_E4,
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
            from: FIELD_D3,
            to: FIELD_D4,
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
                from: FIELD_D2,
                to: FIELD_D3,
                promotion: None,
            },
            Turn {
                from: FIELD_D2,
                to: FIELD_D4,
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

        let mut lowleft_to_topright_counter = -1;
        while lowleft_to_topright_counter < 5 {
            let test_turn = Turn {
                from: FIELD_D2,
                to: Field::new_from_usize(
                    (3 + lowleft_to_topright_counter) as usize,
                    (1 + lowleft_to_topright_counter) as usize,
                )
                .unwrap(),
                promotion: None,
            };
            if test_turn.from != test_turn.to {
                assert!(possible_moves.contains(&test_turn));
            }
            lowleft_to_topright_counter += 1;
        }

        let mut lowright_to_topleft_counter = -3;
        while lowright_to_topleft_counter < 2 {
            let test_turn = Turn {
                from: FIELD_D2,
                to: Field::new_from_usize(
                    (3 + lowright_to_topleft_counter) as usize,
                    (1 - lowright_to_topleft_counter) as usize,
                )
                .unwrap(),
                promotion: None,
            };
            if test_turn.from != test_turn.to {
                assert!(possible_moves.contains(&test_turn));
            }
            lowright_to_topleft_counter += 1;
        }
    }

    /// Tests the possible moves of the black pawn
    #[test]
    fn pawn_black_test() {
        let game = FEN::import("8/8/8/8/8/3p4/8/8 b - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 1);
        let test_turn = Turn {
            from: FIELD_D3,
            to: FIELD_D2,
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
                from: FIELD_D7,
                to: FIELD_D6,
                promotion: None,
            },
            Turn {
                from: FIELD_D7,
                to: FIELD_D5,
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
            from: FIELD_D2,
            to: FIELD_C3,
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
            from: FIELD_C3,
            to: FIELD_D2,
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
            from: FIELD_A2,
            to: FIELD_E2,
            promotion: None,
        };
        let test_move = possible_moves.first().unwrap();
        assert!(*test_move == test_turn);
    }

    /// Tests if fully illegal moves are detected correctly
    #[test]
    fn fully_illegal_test() {
        let game = FEN::import("8/8/8/3P4/3P4/8/8/8 w - - 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        assert!(possible_moves.len() == 1);
        let test_turn = Turn {
            from: FIELD_D5,
            to: FIELD_D6,
            promotion: None,
        };
        let test_move = possible_moves.first().unwrap();
        assert!(*test_move == test_turn);
    }

    /// Tests if en-passant is possible
    #[test]
    fn pawn_enpassant_test() {
        let game = FEN::import("8/8/8/Pp6/8/8/8/8 w - b5 0 1").unwrap();
        let possible_moves = game.get_possible_moves();
        let test_turn = [
            Turn {
                from: FIELD_A5,
                to: FIELD_A6,
                promotion: None,
            },
            Turn {
                from: FIELD_A5,
                to: FIELD_B6,
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
                from: FIELD_A7,
                to: FIELD_A8,
                promotion: Some(PieceType::Queen),
            },
            Turn {
                from: FIELD_A7,
                to: FIELD_A8,
                promotion: Some(PieceType::Rook),
            },
            Turn {
                from: FIELD_A7,
                to: FIELD_A8,
                promotion: Some(PieceType::Bishop),
            },
            Turn {
                from: FIELD_A7,
                to: FIELD_A8,
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
                from: FIELD_A2,
                to: FIELD_A1,
                promotion: Some(PieceType::Queen),
            },
            Turn {
                from: FIELD_A2,
                to: FIELD_A1,
                promotion: Some(PieceType::Rook),
            },
            Turn {
                from: FIELD_A2,
                to: FIELD_A1,
                promotion: Some(PieceType::Bishop),
            },
            Turn {
                from: FIELD_A2,
                to: FIELD_A1,
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
