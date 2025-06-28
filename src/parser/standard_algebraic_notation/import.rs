use crate::{
    CLASSIC_RULESET, Columns, Field,
    Fields::{FIELD_A1, FIELD_E1},
    Piece, PieceType, PlayerColor, Position, Rows, Ruleset, Turn,
};

use pest::{Parser, iterators::Pair};

use super::{San, SanParserError};

#[derive(Parser)]
#[grammar = "parser/standard_algebraic_notation/standard_algebraic_notation.pest"]
struct SanPestParser;

impl San {
    /// Converts a string in SAN representation to a `Turn` object.
    /// This function assumes the classical chess ruleset is used.
    /// - `raw` - The turn in san notation
    /// - `current_position` - The position the given turn was played in
    /// - `returns` - The resulting turn or an error
    pub fn import(raw: &str, current_position: &Position) -> Result<Turn, SanParserError> {
        Self::import_by_ruleset(raw, current_position, &CLASSIC_RULESET)
    }

    /// Converts a string in san notation into a `Turn` if the `Turn` is valid in the given ruleset.
    /// - `raw` - The turn in san notation
    /// - `current_position` - The position the given turn was played in
    /// - `ruleset` - The ruleset used in the game
    /// - `returns` - The resulting turn or an error
    pub fn import_by_ruleset(
        raw: &str,
        current_position: &Position,
        ruleset: &Ruleset,
    ) -> Result<Turn, SanParserError> {
        let Ok(mut parsed_data) = SanPestParser::parse(Rule::turn, raw) else {
            return Err(SanParserError::InvalidData(raw.to_string()));
        };

        let Some(turn_type) = parsed_data.next().unwrap().into_inner().next() else {
            return Err(SanParserError::InvalidData(raw.to_string()));
        };

        match turn_type.as_rule() {
            Rule::pawn_move => Self::import_pawn_movement(turn_type, current_position, ruleset),
            Rule::castling => Ok(Self::import_handle_castling(
                &turn_type,
                current_position,
                ruleset,
            )),
            Rule::piece_move_full => {
                Self::import_piece_move_full(turn_type, current_position, ruleset)
            }
            _ => Err(SanParserError::InvalidData(raw.to_string())),
        }
    }

    /// Converts the full piece move into a turn
    /// - `san_data` - The pest parsed san data
    /// - `position` - The current game position
    /// - `returns` - The resulting turn
    fn import_piece_move_full(
        san_data: Pair<Rule>,
        position: &Position,
        ruleset: &Ruleset,
    ) -> Result<Turn, SanParserError> {
        let possible_moves = ruleset.get_possible_turns(position);
        let raw_turn = san_data.as_str().to_string();

        let mut piece_type: Option<Piece> = None;
        let mut target_field: Option<Field> = None;
        let mut from_column: Option<u8> = None;
        let mut from_row: Option<u8> = None;

        for parts in san_data.into_inner() {
            match parts.as_rule() {
                Rule::piece_symbol => {
                    let piecetype_letter =
                        (parts.as_str().as_bytes()[0] as char).to_ascii_lowercase();

                    if let Some(piece) = PieceType::import_piecetype(piecetype_letter) {
                        piece_type = Some(Piece::new(piece, position.get_active_color()));
                    }
                }
                Rule::piece_move => {
                    let (target, column, row) = Self::import_piece_move(parts);
                    target_field = Some(target);
                    from_column = column;
                    from_row = row;
                }
                _ => return Err(SanParserError::InvalidData(raw_turn)),
            }
        }

        let piece_type = piece_type.unwrap();
        let target_field = target_field.unwrap();

        for turn in possible_moves {
            if target_field == turn.target
                && let Some(occupation) = position.get_field_occupation(&turn.current)
                && occupation == piece_type
            {
                // If a column is set in san notation, check whether column is correct
                if let Some(column_value) = from_column
                    && turn.current.get_column() != column_value
                {
                    continue;
                }

                // If a row is set in san notation, check whether row is correct
                if let Some(row_value) = from_row
                    && turn.current.get_row() != row_value
                {
                    continue;
                }
                return Ok(turn);
            }
        }
        Err(SanParserError::InvalidMove(raw_turn))
    }

    /// Converts a simple piece move
    /// - `san_data` - The pest parsed san data
    /// - `returns` - The target field
    fn import_piece_move(san_data: Pair<Rule>) -> (Field, Option<u8>, Option<u8>) {
        let mut from_column: Option<u8> = None;
        let mut from_row: Option<u8> = None;

        for part in san_data.into_inner() {
            match part.as_rule() {
                Rule::to_field => {
                    return (
                        Field::new_from_string(part.as_str()).unwrap(),
                        from_column,
                        from_row,
                    );
                }
                Rule::from_field => {
                    let data = part.as_str().as_bytes();
                    if data[0] >= b'a' && data[0] <= b'h' {
                        from_column = Some(data[0] - b'a');
                        if data.len() > 1 {
                            from_row = Some(data[1] - b'1');
                        }
                    } else {
                        from_row = Some(data[0] - b'1');
                    }
                }
                _ => unreachable!(),
            }
        }
        unreachable!();
    }

    /// Converts the san castling moves into turns
    /// - `san_data` - The pest parsed turn data
    /// - `position` - The position in which the turn was played
    /// - `returns` - The resulting `Turn`
    fn import_handle_castling(
        san_data: &Pair<Rule>,
        position: &Position,
        ruleset: &Ruleset,
    ) -> Turn {
        let possible_moves = ruleset.get_possible_turns(position);
        let player_color = position.get_active_color();

        // Initiate with row for white
        let mut target_field = FIELD_A1;
        let mut starting_field = FIELD_E1;

        // Change row if color is black
        if player_color == PlayerColor::Black {
            starting_field.set_row(Rows::ROW_8);
            target_field.set_row(Rows::ROW_8);
        }

        // Check if castle is king or queenside
        match san_data.as_str() {
            "O-O" | "0-0" => {
                target_field.set_column(Columns::COLUMN_G);
            }
            "O-O-O" | "0-0-0" => {
                target_field.set_column(Columns::COLUMN_C);
            }
            _ => unreachable!(),
        };

        possible_moves
            .into_iter()
            .find(|&turn| turn.target == target_field && turn.current == starting_field)
            .unwrap()
    }

    /// Convert the san pawn moves into turns
    fn import_pawn_movement(
        san_data: Pair<Rule>,
        position: &Position,
        ruleset: &Ruleset,
    ) -> Result<Turn, SanParserError> {
        let possible_moves = ruleset.get_possible_turns(position);
        let raw_turn = san_data.as_str().to_string();

        let mut target_field: Option<Field> = None;
        let mut promotion_piece: Option<PieceType> = None;
        let mut from_column: Option<u8> = None;
        let mut from_row: Option<u8> = None;

        // Create target field and promotion target
        for pawn_push in san_data.into_inner() {
            match pawn_push.as_rule() {
                Rule::to_field => target_field = Field::new_from_string(pawn_push.as_str()),
                Rule::promotion_piece => {
                    let letter = (pawn_push.as_str().as_bytes()[0] as char).to_ascii_lowercase();
                    let piece_type = PieceType::import_piecetype(letter).unwrap();
                    promotion_piece = Some(piece_type);
                }
                Rule::from_field => {
                    let data = pawn_push.as_str().as_bytes();
                    from_column = Some(data[0] - b'a');
                    if data.len() > 1 {
                        from_row = Some(data[1] - 1);
                    }
                }
                _ => unreachable!(),
            }
        }

        let target_field = target_field.unwrap();

        for turn in possible_moves {
            let from_occupation = position.get_field_occupation(&turn.current);
            let Some(moving_piece) = from_occupation else {
                return Err(SanParserError::InvalidMove(raw_turn));
            };
            if target_field == turn.target
                && promotion_piece == turn.promotion
                && moving_piece.get_type() == PieceType::Pawn
            {
                let Some(column) = from_column else {
                    return Ok(turn);
                };

                if let Some(row) = from_row {
                    if column == turn.current.get_column() && row == turn.current.get_row() {
                        return Ok(turn);
                    }
                    continue;
                }

                if column == turn.current.get_column() {
                    return Ok(turn);
                }
            }
        }
        Err(SanParserError::InvalidMove(raw_turn))
    }
}
