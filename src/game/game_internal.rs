use std::collections::HashMap;

use crate::{Game, Position, Turn, util::error::error_messages};

impl Game {
    /// Creates a new game without any position or data.
    /// - `returns` - A new empty game
    #[must_use]
    pub(crate) fn new_empty() -> Game {
        Game {
            game_metadata: HashMap::<String, String>::new(),
            position_history: Vec::<Position>::new(),
            turn_history: Vec::<Turn>::new(),
        }
    }

    /// Returns a reference to the current game state.
    /// - `returns` - A reference to the current game state.
    #[must_use]
    pub(crate) fn get_current_state_reference(&self) -> &Position {
        let Some(state) = self.position_history.last() else {
            panic!("{}", error_messages::INTERNAL_ERROR_MESSAGE);
        };

        state
    }

    /// Returns a reference to the game state at the given index.
    /// - `returns` - A reference to the game state at the given index.
    #[must_use]
    pub(crate) fn get_state_at_index_reference(&self, index: usize) -> &Position {
        let Some(state) = self.position_history.get(index) else {
            panic!("{}", error_messages::INTERNAL_ERROR_MESSAGE);
        };

        state
    }

    /// Returns the turn at the given index.
    /// - `returns` - The turn at the given index.
    #[must_use]
    pub(crate) fn get_turn_at_index(&self, index: usize) -> Turn {
        let Some(turn) = self.turn_history.get(index) else {
            panic!("{}", error_messages::INTERNAL_ERROR_MESSAGE);
        };

        *turn
    }

    /// Adds the given position to the game
    /// - `position` - The position which is added to the position history
    pub(crate) fn push_position(&mut self, position: Position) {
        self.position_history.push(position);
    }

    /// Returns the whole map of metadata
    /// - `returns` - The whole metadata map
    pub(crate) fn get_metadata_map(&self) -> &HashMap<String, String> {
        &self.game_metadata
    }
}
