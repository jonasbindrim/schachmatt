use crate::{
    Board, Field, Piece, PlayerColor, Position, Turn,
    data_structures::piece::{piece_move_iterator::PieceMoveIterator, piece_type::PieceType},
    util::castle_data::{
        CASTLE_BK_BLOCKED, CASTLE_BK_CHECKED, CASTLE_BQ_BLOCKED, CASTLE_BQ_CHECKED,
        CASTLE_WK_BLOCKED, CASTLE_WK_CHECKED, CASTLE_WQ_BLOCKED, CASTLE_WQ_CHECKED,
    },
};

use super::util::move_legality::MoveLegality;

impl Position {
    /// Takes a turn which is a promotion turn and returns a vector of each possible resulting promotion turn.
    /// - `turn` - The promotion turn
    /// - `turns` - the vector of turns to wich turn should be pushed
    pub(crate) fn push_turn(turn: Turn) -> Vec<Turn> {
        let mut promotion_turns = Vec::<Turn>::with_capacity(4);
        let mut base_turn = turn;

        base_turn.promotion = Option::Some(PieceType::Rook);
        promotion_turns.push(base_turn);

        base_turn.promotion = Option::Some(PieceType::Queen);
        promotion_turns.push(base_turn);

        base_turn.promotion = Option::Some(PieceType::Bishop);
        promotion_turns.push(base_turn);

        base_turn.promotion = Option::Some(PieceType::Knight);
        promotion_turns.push(base_turn);

        promotion_turns
    }

    /// Executes the given turn. This method does not check whether a turn is legal.
    /// - `action` - The turn which should be played
    /// # Panics
    /// This panic indicates an error in the library.
    pub(crate) fn internal_turn(&mut self, action: &Turn) {
        let from_field = self.get_field_occupation(action.from);
        let Some(moving_piece) = from_field else {
            todo!() // TODO: Do something if illegal move is played
        };

        let to_field = self.get_field_occupation(action.to);

        // Increase move counter if no piece has been taken and no pawn has been moved
        if moving_piece.get_type() == PieceType::Pawn || to_field.is_some() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        // Move the piece
        self.board_position[action.to.row as usize][action.to.column as usize] = Some(moving_piece);
        self.board_position[action.from.row as usize][action.from.column as usize] = None;

        if PieceType::King == moving_piece.get_type() {
            match moving_piece.get_color() {
                PlayerColor::Black => {
                    if action.from.row == Board::ROW_8 && action.from.column == Board::COLUMN_E {
                        if action.to.row == Board::ROW_8 && action.to.column == Board::COLUMN_C {
                            self.board_position[Board::ROW_8 as usize][Board::COLUMN_D as usize] =
                                self.board_position[Board::ROW_8 as usize]
                                    [Board::COLUMN_A as usize];
                            self.board_position[Board::ROW_8 as usize][Board::COLUMN_A as usize] =
                                None;
                        } else if action.to.row == Board::ROW_8
                            && action.to.column == Board::COLUMN_G
                        {
                            self.board_position[Board::ROW_8 as usize][Board::COLUMN_F as usize] =
                                self.board_position[Board::ROW_8 as usize]
                                    [Board::COLUMN_H as usize];
                            self.board_position[Board::ROW_8 as usize][Board::COLUMN_H as usize] =
                                None;
                        }
                    }
                    self.castling_black.kingside = false;
                    self.castling_black.queenside = false;
                }
                PlayerColor::White => {
                    if action.from.row == Board::ROW_1 && action.from.column == Board::COLUMN_E {
                        if action.to.row == Board::ROW_1 && action.to.column == Board::COLUMN_C {
                            self.board_position[Board::ROW_1 as usize][Board::COLUMN_D as usize] =
                                self.board_position[Board::ROW_1 as usize]
                                    [Board::COLUMN_A as usize];
                            self.board_position[Board::ROW_1 as usize][Board::COLUMN_A as usize] =
                                None;
                        } else if action.to.row == Board::ROW_1
                            && action.to.column == Board::COLUMN_G
                        {
                            self.board_position[Board::ROW_1 as usize][Board::COLUMN_F as usize] =
                                self.board_position[Board::ROW_1 as usize]
                                    [Board::COLUMN_H as usize];
                            self.board_position[Board::ROW_1 as usize][Board::COLUMN_H as usize] =
                                None;
                        }
                    }
                    self.castling_white.kingside = false;
                    self.castling_white.queenside = false;
                }
            }
        }

        // Remove castling rights if the rook moves
        if PieceType::Rook == moving_piece.get_type() {
            match moving_piece.get_color() {
                PlayerColor::Black => {
                    if action.from.row == Board::ROW_8 && action.from.column == Board::COLUMN_A {
                        self.castling_black.queenside = false;
                    } else if action.from.row == Board::ROW_8
                        && action.from.column == Board::COLUMN_H
                    {
                        self.castling_black.kingside = false;
                    }
                }
                PlayerColor::White => {
                    if action.from.row == Board::ROW_1 && action.from.column == Board::COLUMN_A {
                        self.castling_white.queenside = false;
                    } else if action.from.row == Board::ROW_1
                        && action.from.column == Board::COLUMN_H
                    {
                        self.castling_white.kingside = false;
                    }
                }
            }
        }

        // Promote if possible
        if PieceType::Pawn == moving_piece.get_type()
            && (action.to.row == Board::ROW_1 || action.to.row == Board::ROW_8)
        {
            self.board_position[action.to.row as usize][action.to.column as usize] =
                Some(Piece::new(action.promotion.unwrap(), self.active_color));
        }

        // Remove piece taken with en passant
        if let Some(field) = self.en_passant {
            if action.to.column == field.column && action.from.row == field.row {
                self.board_position[field.row as usize][field.column as usize] = None;
                self.halfmove_clock = 0;
            }
        }

        // Empty en-passant field
        self.en_passant = None;

        // Check if a pawn has moved two field for possible en passant
        if PieceType::Pawn == moving_piece.get_type()
            && action.from.row.abs_diff(action.to.row) == 2
        {
            self.en_passant = Some(action.to);
        }

        // Raise fullmove counter
        if self.active_color == PlayerColor::Black {
            self.fullmove_counter += 1;
        }

        // Change color at turn
        self.active_color = self.active_color.reverse();
    }

    /// Checks if the move which is specified by the two fields is a legal move
    /// - `turn` - The turn which is checked
    /// - `player_color` - The player that performs the turn
    /// - `returns` - Returns whether the turn is legal
    pub(crate) fn is_legal_move(&self, turn: Turn, check_for_check: bool) -> MoveLegality {
        let moving_piece = match self.get_field_occupation(turn.from) {
            Some(piece) => piece,
            None => return MoveLegality::FullyIllegal,
        };
        let active_color = moving_piece.get_color();

        // Check if move is capture and whether it captures an enemy piece
        let is_capture = match self.get_field_occupation(turn.to) {
            Some(piece) => {
                if piece.get_color() == active_color {
                    return MoveLegality::FullyIllegal;
                }

                true
            }
            None => false,
        };

        let legality_state = match moving_piece.get_type() {
            PieceType::Pawn => self.is_pawn_move_legal(turn, active_color, is_capture),
            PieceType::King => self.is_king_move_legal(turn, active_color),
            _ => MoveLegality::Legal,
        };

        if matches!(
            legality_state,
            MoveLegality::FullyIllegal | MoveLegality::TemporarelyIllegal
        ) {
            return legality_state;
        }

        // Play move and check if a king is checked
        if check_for_check {
            let mut resulting_position: Position = (*self).clone();
            resulting_position.internal_turn(&turn);
            if resulting_position.is_in_check(active_color) {
                return MoveLegality::TemporarelyIllegal;
            }
        }

        if is_capture {
            return MoveLegality::LastLegal;
        }
        MoveLegality::Legal
    }

    /// Checks whether a given king move is legal.
    /// - `turn` - The turn played by the king
    /// - `active_color` - The currently active color
    /// - `returns` - Whether or not the move is legal
    fn is_king_move_legal(&self, turn: Turn, active_color: PlayerColor) -> MoveLegality {
        let step = turn.to.column as i8 - turn.from.column as i8;
        if step == 2 {
            if turn.from.row == Board::ROW_8
                && active_color == PlayerColor::Black
                && self.castling_black.kingside
            {
                if self.is_castle_illegal(&CASTLE_BK_BLOCKED, CASTLE_BK_CHECKED, active_color) {
                    return MoveLegality::FullyIllegal;
                }
            } else if turn.from.row == Board::ROW_1
                && active_color == PlayerColor::White
                && self.castling_white.kingside
            {
                if self.is_castle_illegal(&CASTLE_WK_BLOCKED, CASTLE_WK_CHECKED, active_color) {
                    return MoveLegality::FullyIllegal;
                }
            } else {
                return MoveLegality::FullyIllegal;
            }
        } else if step == -2 {
            if turn.from.row == Board::ROW_8
                && active_color == PlayerColor::Black
                && self.castling_black.queenside
            {
                if self.is_castle_illegal(&CASTLE_BQ_BLOCKED, CASTLE_BQ_CHECKED, active_color) {
                    return MoveLegality::FullyIllegal;
                }
            } else if turn.from.row == Board::ROW_1
                && active_color == PlayerColor::White
                && self.castling_white.queenside
            {
                if self.is_castle_illegal(&CASTLE_WQ_BLOCKED, CASTLE_WQ_CHECKED, active_color) {
                    return MoveLegality::FullyIllegal;
                }
            } else {
                return MoveLegality::FullyIllegal;
            }
        }
        MoveLegality::Legal
    }

    /// Checks whether a given pawn move is legal or illegal.
    /// - `turn` - The turn which gets checked
    /// - `active_color` - The color playing the move
    /// - `is_capture` - Whether or not the move is a capture move
    /// - `returns` - Whether the move is deemed to be legal or not
    fn is_pawn_move_legal(
        &self,
        turn: Turn,
        active_color: PlayerColor,
        is_capture: bool,
    ) -> MoveLegality {
        // Forward moves
        if turn.from.column == turn.to.column {
            if is_capture {
                return MoveLegality::FullyIllegal;
            }

            if match active_color {
                PlayerColor::Black => {
                    turn.to.row.abs_diff(turn.from.row) == 2 && turn.from.row != Board::ROW_7
                }
                PlayerColor::White => {
                    turn.from.row.abs_diff(turn.to.row) == 2 && turn.from.row != Board::ROW_2
                }
            } {
                return MoveLegality::FullyIllegal;
            }
        }

        // Check if a pawn can capture diagonally
        if turn.from.column != turn.to.column && self.get_field_occupation(turn.to).is_none() {
            let Some(field) = self.en_passant else {
                return MoveLegality::FullyIllegal;
            };

            if turn.to.column != field.column || turn.from.row != field.row {
                return MoveLegality::FullyIllegal;
            }
        }
        MoveLegality::Legal
    }

    /// Runs the check if castling is allowed in the current position
    /// - `blocked_fields` - An array of fields which must not be blocked
    /// - `checked_fields` - An array of fields which must not be under attack
    /// - `player_color` - The player to check castling rights for
    /// - `returns` - Whether the player is allowed to castle
    pub(crate) fn is_castle_illegal(
        &self,
        blocked_fields: &[Field],
        checked_fields: [Field; 3],
        player_color: PlayerColor,
    ) -> bool {
        self.castling_fields_blocked(blocked_fields)
            || self.fields_under_attack(player_color, checked_fields)
    }

    /// Checks and returns if one of the given fields is blocked by another piece
    /// - `fields` - The fields to check for blockades
    /// - `returns` - Whether one of the fields is blocked
    pub(crate) fn castling_fields_blocked(&self, fields: &[Field]) -> bool {
        for field in fields {
            if self.get_field_occupation(*field).is_some() {
                return true;
            }
        }
        false
    }

    /// Returns if the given player is currently checked
    /// - `player_color` - The player to check for being checked
    /// - `returns` - Whether the given player is currently checked
    pub(crate) fn is_in_check(&self, player_color: PlayerColor) -> bool {
        for (row, column) in BOARD_FIELDS {
            let occupation = self.board_position[row][column];
            if let Some(piece) = occupation {
                if piece.get_color() != player_color {
                    let mut piece_iterator = PieceMoveIterator::new(
                        piece.movement_modifiers(),
                        Field::new_from_usize(column, row).unwrap(),
                    );

                    loop {
                        while let Some(turn) = piece_iterator.current() {
                            // Handling of the next loops
                            match self.is_legal_move(turn, false) {
                                MoveLegality::Legal => {
                                    if self.get_field_occupation(turn.to).is_none() {
                                        continue;
                                    }
                                    break;
                                }
                                MoveLegality::LastLegal => {
                                    if let Some(target_piece) = self.get_field_occupation(turn.to) {
                                        if PieceType::King == target_piece.get_type() {
                                            return true;
                                        }
                                    }
                                    break;
                                }
                                MoveLegality::TemporarelyIllegal => continue,
                                MoveLegality::FullyIllegal => break,
                            }
                        }
                        if !piece_iterator.step() {
                            break;
                        }
                    }
                }
            }
        }
        false
    }

    /// Checks whether the given fields are under attack by the enemie color
    /// - `player_color` - The color at turn
    /// - `fields` - The fields to check for being attacked
    /// - `returns` - Whether one of the fields is being attacked
    pub(crate) fn fields_under_attack(
        &self,
        player_color: PlayerColor,
        fields: [Field; 3],
    ) -> bool {
        for (row, column) in BOARD_FIELDS {
            let occupation = self.board_position[row][column];

            if let Some(piece) = occupation {
                if piece.get_color() != player_color {
                    let mut piece_iterator = PieceMoveIterator::new(
                        piece.movement_modifiers(),
                        Field::new_from_usize(column, row).unwrap(),
                    );

                    loop {
                        while let Some(turn) = piece_iterator.current() {
                            // Dont check castling options
                            if PieceType::King == piece.get_type()
                                && turn.from.column.abs_diff(turn.to.column) == 2
                            {
                                continue;
                            }

                            // Handling of the next loops
                            match self.is_legal_move(turn, false) {
                                MoveLegality::Legal => {
                                    let compare_field = Field::new_from_usize(column, row).unwrap();
                                    if fields.contains(&compare_field) {
                                        return true;
                                    }
                                    continue;
                                }
                                MoveLegality::LastLegal => {
                                    let compare_field = Field::new_from_usize(column, row).unwrap();
                                    if fields.contains(&compare_field) {
                                        return true;
                                    }
                                    break;
                                }
                                MoveLegality::TemporarelyIllegal => continue,
                                MoveLegality::FullyIllegal => break,
                            }
                        }
                        if !piece_iterator.step() {
                            break;
                        }
                    }
                }
            }
        }
        false
    }

    /// Returns whether enough material is on the board to checkmate
    /// - `returns` - Whether enough material is on the board to checkmate
    pub(crate) fn is_sufficient_material(&self) -> bool {
        let mut white_bishop: bool = false;
        let mut white_knight: bool = false;
        let mut black_bishop: bool = false;
        let mut black_knight: bool = false;

        let material_handling = |piece: Piece| -> Option<bool> {
            match piece.get_type() {
                PieceType::Pawn | PieceType::Rook | PieceType::Queen => return Some(true),
                PieceType::Bishop => match piece.get_color() {
                    PlayerColor::Black => {
                        if black_bishop || black_knight || white_knight {
                            return Some(true);
                        }
                        black_bishop = true;
                    }
                    PlayerColor::White => {
                        if white_bishop || white_knight || black_knight {
                            return Some(true);
                        }
                        white_bishop = true;
                    }
                },
                PieceType::Knight => {
                    if black_bishop || black_knight || white_bishop || white_knight {
                        return Some(true);
                    }
                    match piece.get_color() {
                        PlayerColor::Black => black_knight = true,
                        PlayerColor::White => white_knight = true,
                    }
                }
                _ => {}
            }
            None
        };

        self.iterate_board_by_piece(material_handling)
    }

    /// Returns the field occupation at the given position of `self.board_position`
    /// - `field` - The field of the piece to return
    /// - `returns` - The occupation of the given field
    #[inline]
    pub(crate) fn get_field_occupation(&self, field: Field) -> Option<Piece> {
        self.board_position[field.row as usize][field.column as usize]
    }

    /// This function iterates over every field of the position and calls the given function for each field
    /// - `func` - The function which is called for every field of the board
    /// - `returns` - The value returned by the given func
    pub(crate) fn iterate_board_by_piece<ClosureType>(&self, mut func: ClosureType) -> bool
    where
        ClosureType: FnMut(Piece) -> Option<bool>,
    {
        for row in self.board_position {
            for piece in row.into_iter().flatten() {
                if let Some(value) = func(piece) {
                    return value;
                }
            }
        }
        false
    }
}

pub(crate) static BOARD_FIELDS: [(usize, usize); 64] = [
    (Board::ROW_1 as usize, Board::COLUMN_A as usize),
    (Board::ROW_1 as usize, Board::COLUMN_B as usize),
    (Board::ROW_1 as usize, Board::COLUMN_C as usize),
    (Board::ROW_1 as usize, Board::COLUMN_D as usize),
    (Board::ROW_1 as usize, Board::COLUMN_E as usize),
    (Board::ROW_1 as usize, Board::COLUMN_F as usize),
    (Board::ROW_1 as usize, Board::COLUMN_G as usize),
    (Board::ROW_1 as usize, Board::COLUMN_H as usize),
    (Board::ROW_2 as usize, Board::COLUMN_A as usize),
    (Board::ROW_2 as usize, Board::COLUMN_B as usize),
    (Board::ROW_2 as usize, Board::COLUMN_C as usize),
    (Board::ROW_2 as usize, Board::COLUMN_D as usize),
    (Board::ROW_2 as usize, Board::COLUMN_E as usize),
    (Board::ROW_2 as usize, Board::COLUMN_F as usize),
    (Board::ROW_2 as usize, Board::COLUMN_G as usize),
    (Board::ROW_2 as usize, Board::COLUMN_H as usize),
    (Board::ROW_3 as usize, Board::COLUMN_A as usize),
    (Board::ROW_3 as usize, Board::COLUMN_B as usize),
    (Board::ROW_3 as usize, Board::COLUMN_C as usize),
    (Board::ROW_3 as usize, Board::COLUMN_D as usize),
    (Board::ROW_3 as usize, Board::COLUMN_E as usize),
    (Board::ROW_3 as usize, Board::COLUMN_F as usize),
    (Board::ROW_3 as usize, Board::COLUMN_G as usize),
    (Board::ROW_3 as usize, Board::COLUMN_H as usize),
    (Board::ROW_4 as usize, Board::COLUMN_A as usize),
    (Board::ROW_4 as usize, Board::COLUMN_B as usize),
    (Board::ROW_4 as usize, Board::COLUMN_C as usize),
    (Board::ROW_4 as usize, Board::COLUMN_D as usize),
    (Board::ROW_4 as usize, Board::COLUMN_E as usize),
    (Board::ROW_4 as usize, Board::COLUMN_F as usize),
    (Board::ROW_4 as usize, Board::COLUMN_G as usize),
    (Board::ROW_4 as usize, Board::COLUMN_H as usize),
    (Board::ROW_5 as usize, Board::COLUMN_A as usize),
    (Board::ROW_5 as usize, Board::COLUMN_B as usize),
    (Board::ROW_5 as usize, Board::COLUMN_C as usize),
    (Board::ROW_5 as usize, Board::COLUMN_D as usize),
    (Board::ROW_5 as usize, Board::COLUMN_E as usize),
    (Board::ROW_5 as usize, Board::COLUMN_F as usize),
    (Board::ROW_5 as usize, Board::COLUMN_G as usize),
    (Board::ROW_5 as usize, Board::COLUMN_H as usize),
    (Board::ROW_6 as usize, Board::COLUMN_A as usize),
    (Board::ROW_6 as usize, Board::COLUMN_B as usize),
    (Board::ROW_6 as usize, Board::COLUMN_C as usize),
    (Board::ROW_6 as usize, Board::COLUMN_D as usize),
    (Board::ROW_6 as usize, Board::COLUMN_E as usize),
    (Board::ROW_6 as usize, Board::COLUMN_F as usize),
    (Board::ROW_6 as usize, Board::COLUMN_G as usize),
    (Board::ROW_6 as usize, Board::COLUMN_H as usize),
    (Board::ROW_7 as usize, Board::COLUMN_A as usize),
    (Board::ROW_7 as usize, Board::COLUMN_B as usize),
    (Board::ROW_7 as usize, Board::COLUMN_C as usize),
    (Board::ROW_7 as usize, Board::COLUMN_D as usize),
    (Board::ROW_7 as usize, Board::COLUMN_E as usize),
    (Board::ROW_7 as usize, Board::COLUMN_F as usize),
    (Board::ROW_7 as usize, Board::COLUMN_G as usize),
    (Board::ROW_7 as usize, Board::COLUMN_H as usize),
    (Board::ROW_8 as usize, Board::COLUMN_A as usize),
    (Board::ROW_8 as usize, Board::COLUMN_B as usize),
    (Board::ROW_8 as usize, Board::COLUMN_C as usize),
    (Board::ROW_8 as usize, Board::COLUMN_D as usize),
    (Board::ROW_8 as usize, Board::COLUMN_E as usize),
    (Board::ROW_8 as usize, Board::COLUMN_F as usize),
    (Board::ROW_8 as usize, Board::COLUMN_G as usize),
    (Board::ROW_8 as usize, Board::COLUMN_H as usize),
];
