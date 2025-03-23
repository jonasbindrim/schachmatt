use std::collections::HashMap;

use crate::{FEN, Game, GameResult, PlayerColor, Position, Turn, util::error::error_messages};

impl Game {
    /// Creates a new `Game` with the default chess board setup.
    /// To start a game in the classic chess board setup use `Game::default()`.
    /// - `starting_position` - The `Position` the `Game` should start from
    /// - `returns` - A new `Game` with the given board setup
    #[must_use]
    pub fn new(starting_position: Position) -> Game {
        let mut game = Game {
            game_metadata: HashMap::<String, String>::new(),
            position_history: Vec::<Position>::new(),
            turn_history: Vec::<Turn>::new(),
        };

        let position_fen = FEN::export(&starting_position);
        if position_fen != FEN::DEFAULT_BOARD_SETUP {
            game.set_metadata("FEN", &position_fen);
        }
        game.position_history.push(starting_position);

        game
    }

    /// Returns the metadata value for the corresponding key if the key exists.
    /// - `key` - The key identifing the metadata
    /// - `returns` - The metadata associated with the given key
    #[must_use]
    pub fn get_metadata(&self, key: &str) -> Option<String> {
        self.game_metadata.get(key).cloned()
    }

    /// Sets the metadata for this game.
    /// The specific metadata is identified by given key and will be
    /// written to the metadata section of a pgn if converted into pgn.
    /// If a value is already stored for this key, it is overwritten.
    /// - `key` - The key under which the value should be stored
    /// - `value` - The value which will be stored as metadata
    pub fn set_metadata(&mut self, key: &str, value: &str) {
        self.game_metadata
            .insert(key.to_string(), value.to_string());
    }

    /// Returns a copy of the current game state.
    /// - `returns` - A copy of the current game state.
    /// # Panics
    /// Panics when the `Game` has no current state.
    /// This panic indicates an error in the library.
    #[must_use]
    pub fn get_current_state(&self) -> Position {
        let Some(position) = self.position_history.last() else {
            panic!("{}", error_messages::INTERNAL_ERROR_MESSAGE)
        };

        position.clone()
    }

    /// Executes the given turn.
    /// - `turn` - The turn to play
    pub fn execute_turn(&mut self, turn: Turn) {
        let mut current_position = self.get_current_state();
        current_position.turn(&turn);
        self.position_history.push(current_position);
        self.turn_history.push(turn);
    }

    /// Returns the result of this game.
    /// - `returns` - The result of this game. Is none if the game has not concluded.
    #[must_use]
    pub fn get_game_result(&mut self) -> Option<GameResult> {
        self.get_current_state_reference().game_over_check()
    }

    /// Returns the color of the player who has to move.
    /// - `returns` - The currently active player color
    #[must_use]
    pub fn get_color_at_turn(&self) -> PlayerColor {
        self.get_current_state_reference().active_color
    }

    /// Returns the latest turn played in this game.
    /// - `returns` - The last turn played in this game
    #[must_use]
    pub fn get_last_turn(&self) -> Option<Turn> {
        self.turn_history.last().copied()
    }
}

impl Default for Game {
    /// Creates a new `Game` with the default chess board setup.
    /// - `returns` - A new `Game` with the default board setup
    #[must_use]
    fn default() -> Self {
        let mut game = Game {
            game_metadata: HashMap::<String, String>::new(),
            position_history: Vec::<Position>::new(),
            turn_history: Vec::<Turn>::new(),
        };

        game.position_history.push(Position::new());

        game
    }
}
