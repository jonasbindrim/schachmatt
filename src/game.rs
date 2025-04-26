use std::collections::HashMap;

use crate::{
    CLASSIC_RULESET, FEN, GameResult, PlayerColor, Position, PositionError, Turn,
    ruleset::Ruleset,
    util::metadata::{METADATA_KEY_FEN, METADATA_KEY_RESULT},
};

/// Represents a game of chess.
#[derive(Clone)]
pub struct Game {
    pub(super) game_metadata: HashMap<String, String>,
    pub(super) position_history: Vec<Position>,
    pub(super) turn_history: Vec<Turn>,
    pub(super) ruleset: Ruleset,
}

impl Game {
    /// Creates a new `Game` with the given set of rules to run the game.
    /// To start a game with the classic rules use `Game::default()`.
    /// - `ruleset` - The set of rules used to run this game
    #[must_use]
    pub fn new(ruleset: &Ruleset) -> Game {
        Self::new_from_position(ruleset, ruleset.generate_initial_position())
    }

    /// Creates a new `Game` with the given set of rules to run the game and the given board setup.
    /// To start a game with the classic rules use `Game::default()`.
    /// - `ruleset` - The set of rules used to run this game
    /// - `starting_position` - Used as the starting position for this game. This overrides the position from the ruleset
    #[must_use]
    pub fn new_from_position(ruleset: &Ruleset, starting_position: Position) -> Game {
        let mut game = Game {
            game_metadata: HashMap::<String, String>::new(),
            position_history: Vec::<Position>::new(),
            turn_history: Vec::<Turn>::new(),
            ruleset: ruleset.clone(),
        };

        let position_fen = FEN::export(&starting_position);
        if position_fen != FEN::DEFAULT_BOARD_SETUP {
            game.set_metadata(METADATA_KEY_FEN, &position_fen);
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

    /// Returns all existing metadata keys
    pub fn get_metadata_keys(&self) -> Vec<String> {
        self.game_metadata.keys().cloned().collect()
    }

    /// Returns the whole map of metadata
    /// - `returns` - The whole metadata map
    pub fn get_metadata_map(&self) -> &HashMap<String, String> {
        &self.game_metadata
    }

    /// Returns the current position in the game.
    #[must_use]
    pub fn get_current_state(&self) -> &Position {
        self.position_history.last().unwrap()
    }

    /// Returns all possible turns in the current game state
    pub fn get_possible_turns(&self) -> Vec<Turn> {
        self.get_current_state().get_possible_turns(&self.ruleset)
    }

    /// Executes the given turn.
    /// Returns a result which indicates whether the given turn was legal.
    /// An illegal turn is not executed and an error is returned.
    /// - `turn` - The turn to play
    pub fn execute_turn(&mut self, turn: Turn) -> Result<(), PositionError> {
        let current_position = self.get_current_state();
        let updated_position = current_position.turn(&self.ruleset, &turn)?;

        self.position_history.push(updated_position);
        self.turn_history.push(turn);

        Ok(())
    }

    /// Returns the result of this game. A check is done whether a result is contained in the metadata.
    /// Only if no result is found in the metadata the actual position is checked.
    /// - `returns` - The result of this game. Is none if the game has not concluded.
    #[must_use]
    pub fn get_game_result(&self) -> Option<GameResult> {
        if let Some(result) = self.get_metadata(METADATA_KEY_RESULT) {
            let result = GameResult::from_string(&result);
            if result.is_some() {
                return result;
            }
        }

        self.get_current_state().game_over_check(&self.ruleset)
    }

    /// Returns the color of the player who has to move.
    /// - `returns` - The currently active player color
    #[must_use]
    pub fn get_color_at_turn(&self) -> PlayerColor {
        self.position_history.last().unwrap().get_active_color()
    }

    /// Returns all positions played in this game.
    /// Index 0 contains the starting position
    /// - `returns` - All positions played in this game.
    #[must_use]
    pub fn get_all_positions(&self) -> Vec<Position> {
        self.position_history.clone()
    }

    /// Returns the position after the given halfmove.
    /// - `halfmove` - 0 = Starting position. X = Position after halfmove x.
    /// - `returns` - The position after the given halfmove
    #[must_use]
    pub fn get_position_by_turn(&self, halfmove: u16) -> Option<Position> {
        Some(self.position_history.get(halfmove as usize)?.clone())
    }

    /// Returns all turns executed in this game.
    /// Index 0 contains the first turn played in the starting position
    /// - `returns` - All turns executed in this game.
    #[must_use]
    pub fn get_all_turns(&self) -> Vec<Turn> {
        self.turn_history.clone()
    }

    /// Returns the latest turn played in this game.
    /// - `returns` - The last turn played in this game
    #[must_use]
    pub fn get_last_turn(&self) -> Option<Turn> {
        self.turn_history.last().copied()
    }

    /// Sets the result of the game. This is used store results which can not be seen by the last position available.
    /// E.g. if a player resigns or the players agree to a draw. Game results set using this method will be considered
    /// more important than the result provided by the last position.
    pub fn set_game_result(&mut self, game_result: Option<GameResult>) {
        self.set_metadata(
            METADATA_KEY_RESULT,
            &GameResult::to_string(game_result.as_ref()),
        );
    }
}

impl Default for Game {
    /// Creates a new `Game` with the default chess board setup.
    /// - `returns` - A new `Game` with the default board setup
    #[must_use]
    fn default() -> Self {
        Game::new(&CLASSIC_RULESET)
    }
}
