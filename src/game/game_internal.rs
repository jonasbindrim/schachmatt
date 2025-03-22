use std::collections::HashMap;

use crate::{util::error::error_messages, Game, Position, Turn};

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
        match self.position_history.last() {
            Some(state) => state,
            None => panic!("{}", error_messages::INTERNAL_ERROR_MESSAGE),
        }
    }

    /// Returns a reference to the game state at the given index.
    /// - `returns` - A reference to the game state at the given index.
    #[must_use]
    pub(crate) fn get_state_at_index_reference(&self, index: usize) -> &Position {
        match self.position_history.get(index) {
            Some(state) => state,
            None => panic!("{}", error_messages::INTERNAL_ERROR_MESSAGE),
        }
    }

    /// Returns a reference to the turn at the given index.
    /// - `returns` - A reference to the turn at the given index.
    #[must_use]
    pub(crate) fn get_turn_at_index(&self, index: usize) -> Turn {
        match self.turn_history.get(index) {
            Some(turn) => *turn,
            None => panic!("{}", error_messages::INTERNAL_ERROR_MESSAGE),
        }
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
