#[cfg(test)]
mod tests {
    use crate::{Board, Field, LAN, Turn, data_structures::piece::piece_type::PieceType};

    #[test]
    pub fn import_lan_pawn_push() {
        let test_data = String::from("d2d4");
        if let Some(turn) = LAN::import(&test_data) {
            assert_eq!(
                turn,
                Turn {
                    from: Field {
                        column: Board::COLUMN_D,
                        row: Board::ROW_2
                    },
                    to: Field {
                        column: Board::COLUMN_D,
                        row: Board::ROW_4
                    },
                    promotion: None
                }
            );
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_lan_push_ambigious() {
        let test_data = String::from("e4e5");
        if let Some(turn) = LAN::import(&test_data) {
            assert_eq!(
                turn,
                Turn {
                    from: Field {
                        column: Board::COLUMN_E,
                        row: Board::ROW_4
                    },
                    to: Field {
                        column: Board::COLUMN_E,
                        row: Board::ROW_5
                    },
                    promotion: None
                }
            );
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_lan_pawn_push_promotion() {
        let test_data = String::from("a7a8q");
        if let Some(turn) = LAN::import(&test_data) {
            assert_eq!(
                turn,
                Turn {
                    from: Field {
                        column: Board::COLUMN_A,
                        row: Board::ROW_7
                    },
                    to: Field {
                        column: Board::COLUMN_A,
                        row: Board::ROW_8
                    },
                    promotion: Some(PieceType::Queen)
                }
            );
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_lan_pawn_capture() {
        let test_data = String::from("a2xb3");
        if let Some(turn) = LAN::import(&test_data) {
            assert_eq!(
                turn,
                Turn {
                    from: Field {
                        column: Board::COLUMN_A,
                        row: Board::ROW_2
                    },
                    to: Field {
                        column: Board::COLUMN_B,
                        row: Board::ROW_3
                    },
                    promotion: None
                }
            );
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_lan_pawn_capture_en_passant() {
        let test_data = String::from("a5xb6");
        if let Some(turn) = LAN::import(&test_data) {
            assert_eq!(
                turn,
                Turn {
                    from: Field {
                        column: Board::COLUMN_A,
                        row: Board::ROW_5
                    },
                    to: Field {
                        column: Board::COLUMN_B,
                        row: Board::ROW_6
                    },
                    promotion: None
                }
            );
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_lan_pawn_capture_with_promotion() {
        let test_data = String::from("a7xb8r");
        if let Some(turn) = LAN::import(&test_data) {
            assert_eq!(
                turn,
                Turn {
                    from: Field {
                        column: Board::COLUMN_A,
                        row: Board::ROW_7
                    },
                    to: Field {
                        column: Board::COLUMN_B,
                        row: Board::ROW_8
                    },
                    promotion: Some(PieceType::Rook)
                }
            );
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_lan_castle_white_king() {
        let test_data = String::from("e1g1");
        if let Some(turn) = LAN::import(&test_data) {
            assert_eq!(
                turn,
                Turn {
                    from: Field {
                        column: Board::COLUMN_E,
                        row: Board::ROW_1
                    },
                    to: Field {
                        column: Board::COLUMN_G,
                        row: Board::ROW_1
                    },
                    promotion: None
                }
            );
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_lan_castle_black_queen() {
        let test_data = String::from("e8c8");
        if let Some(turn) = LAN::import(&test_data) {
            assert_eq!(
                turn,
                Turn {
                    from: Field {
                        column: Board::COLUMN_E,
                        row: Board::ROW_8
                    },
                    to: Field {
                        column: Board::COLUMN_C,
                        row: Board::ROW_8
                    },
                    promotion: None
                }
            );
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_lan_piece_move() {
        let test_data = String::from("Ra1a8");
        if let Some(turn) = LAN::import(&test_data) {
            assert_eq!(
                turn,
                Turn {
                    from: Field {
                        column: Board::COLUMN_A,
                        row: Board::ROW_1
                    },
                    to: Field {
                        column: Board::COLUMN_A,
                        row: Board::ROW_8
                    },
                    promotion: None
                }
            );
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_lan_piece_move_capture() {
        let test_data = String::from("Bf2xd4");
        if let Some(turn) = LAN::import(&test_data) {
            assert_eq!(
                turn,
                Turn {
                    from: Field {
                        column: Board::COLUMN_F,
                        row: Board::ROW_2
                    },
                    to: Field {
                        column: Board::COLUMN_D,
                        row: Board::ROW_4
                    },
                    promotion: None
                }
            );
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_lan_piece_move_capture_ambigious_white() {
        let test_data = String::from("f3xe5");
        if let Some(turn) = LAN::import(&test_data) {
            assert_eq!(
                turn,
                Turn {
                    from: Field {
                        column: Board::COLUMN_F,
                        row: Board::ROW_3
                    },
                    to: Field {
                        column: Board::COLUMN_E,
                        row: Board::ROW_5
                    },
                    promotion: None
                }
            );
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_lan_piece_move_capture_ambigious_black() {
        let test_data = String::from("Nd6e4");
        if let Some(turn) = LAN::import(&test_data) {
            assert_eq!(
                turn,
                Turn {
                    from: Field {
                        column: Board::COLUMN_D,
                        row: Board::ROW_6
                    },
                    to: Field {
                        column: Board::COLUMN_E,
                        row: Board::ROW_4
                    },
                    promotion: None
                }
            );
        } else {
            panic!();
        }
    }

    #[test]
    pub fn import_lan_piece_move_capture_double_ambigious_black() {
        let test_data = String::from("Nd6xe4");
        if let Some(turn) = LAN::import(&test_data) {
            assert_eq!(
                turn,
                Turn {
                    from: Field {
                        column: Board::COLUMN_D,
                        row: Board::ROW_6
                    },
                    to: Field {
                        column: Board::COLUMN_E,
                        row: Board::ROW_4
                    },
                    promotion: None
                }
            );
        } else {
            panic!();
        }
    }

    #[test]
    pub fn export_lan_pawn_push() {
        let test_turn = Turn {
            from: Field {
                column: Board::COLUMN_D,
                row: Board::ROW_2,
            },
            to: Field {
                column: Board::COLUMN_D,
                row: Board::ROW_4,
            },
            promotion: None,
        };

        let test_data = String::from("d2d4");
        assert_eq!(LAN::export(&test_turn), test_data);
    }

    #[test]
    pub fn export_lan_pawn_push_promotion() {
        let test_turn = Turn {
            from: Field {
                column: Board::COLUMN_A,
                row: Board::ROW_7,
            },
            to: Field {
                column: Board::COLUMN_A,
                row: Board::ROW_8,
            },
            promotion: Some(PieceType::Queen),
        };

        let test_data = String::from("a7a8q");
        assert_eq!(LAN::export(&test_turn), test_data);
    }

    #[test]
    pub fn export_lan_pawn_capture() {
        let test_turn = Turn {
            from: Field {
                column: Board::COLUMN_A,
                row: Board::ROW_2,
            },
            to: Field {
                column: Board::COLUMN_B,
                row: Board::ROW_3,
            },
            promotion: None,
        };

        let test_data = String::from("a2b3");
        assert_eq!(LAN::export(&test_turn), test_data);
    }

    #[test]
    pub fn export_lan_pawn_capture_en_paslant() {
        let test_turn = Turn {
            from: Field {
                column: Board::COLUMN_A,
                row: Board::ROW_5,
            },
            to: Field {
                column: Board::COLUMN_B,
                row: Board::ROW_6,
            },
            promotion: None,
        };

        let test_data = String::from("a5b6");
        assert_eq!(LAN::export(&test_turn), test_data);
    }

    #[test]
    pub fn export_lan_pawn_capture_with_promotion() {
        let test_turn = Turn {
            from: Field {
                column: Board::COLUMN_A,
                row: Board::ROW_7,
            },
            to: Field {
                column: Board::COLUMN_B,
                row: Board::ROW_8,
            },
            promotion: Some(PieceType::Rook),
        };

        let test_data = String::from("a7b8r");
        assert_eq!(LAN::export(&test_turn), test_data);
    }

    #[test]
    pub fn export_lan_castle_white_king() {
        let test_turn = Turn {
            from: Field {
                column: Board::COLUMN_E,
                row: Board::ROW_1,
            },
            to: Field {
                column: Board::COLUMN_G,
                row: Board::ROW_1,
            },
            promotion: None,
        };

        let test_data = String::from("e1g1");
        assert_eq!(LAN::export(&test_turn), test_data);
    }

    #[test]
    pub fn export_lan_castle_black_queen() {
        let test_turn = Turn {
            from: Field {
                column: Board::COLUMN_E,
                row: Board::ROW_8,
            },
            to: Field {
                column: Board::COLUMN_C,
                row: Board::ROW_8,
            },
            promotion: None,
        };

        let test_data = String::from("e8c8");
        assert_eq!(LAN::export(&test_turn), test_data);
    }

    #[test]
    pub fn export_lan_piece_move() {
        let test_turn = Turn {
            from: Field {
                column: Board::COLUMN_A,
                row: Board::ROW_1,
            },
            to: Field {
                column: Board::COLUMN_A,
                row: Board::ROW_8,
            },
            promotion: None,
        };

        let test_data = String::from("a1a8");
        assert_eq!(LAN::export(&test_turn), test_data);
    }

    #[test]
    pub fn export_lan_piece_move_capture() {
        let test_turn = Turn {
            from: Field {
                column: Board::COLUMN_F,
                row: Board::ROW_2,
            },
            to: Field {
                column: Board::COLUMN_D,
                row: Board::ROW_4,
            },
            promotion: None,
        };

        let test_data = String::from("f2d4");
        assert_eq!(LAN::export(&test_turn), test_data);
    }

    #[test]
    pub fn export_lan_piece_move_capture_ambigious_white() {
        let test_turn = Turn {
            from: Field {
                column: Board::COLUMN_F,
                row: Board::ROW_3,
            },
            to: Field {
                column: Board::COLUMN_E,
                row: Board::ROW_5,
            },
            promotion: None,
        };

        let test_data = String::from("f3e5");
        assert_eq!(LAN::export(&test_turn), test_data);
    }

    #[test]
    pub fn export_lan_piece_move_capture_ambigious_black() {
        let test_turn = Turn {
            from: Field {
                column: Board::COLUMN_D,
                row: Board::ROW_6,
            },
            to: Field {
                column: Board::COLUMN_E,
                row: Board::ROW_4,
            },
            promotion: None,
        };

        let test_data = String::from("d6e4");
        assert_eq!(LAN::export(&test_turn), test_data);
    }

    #[test]
    pub fn export_lan_piece_move_capture_double_ambigious_black() {
        let test_turn = Turn {
            from: Field {
                column: Board::COLUMN_D,
                row: Board::ROW_6,
            },
            to: Field {
                column: Board::COLUMN_E,
                row: Board::ROW_4,
            },
            promotion: None,
        };

        let test_data = String::from("d6e4");
        assert_eq!(LAN::export(&test_turn), test_data);
    }
}
