use std::collections::HashMap;

use crate::{Game, Position, Turn};

impl Game {
    /// Returns a reference to the current game state.
    /// - `returns` - A reference to the current game state.
    #[must_use]
    pub(crate) fn get_current_state_reference(&self) -> &Position {
        self.position_history.last().unwrap()
    }

    /// Returns the turn at the given index.
    /// - `returns` - The turn at the given index.
    #[must_use]
    pub(crate) fn get_turn_at_index(&self, index: usize) -> Option<&Turn> {
        self.turn_history.get(index)
    }

    /// Returns the whole map of metadata
    /// - `returns` - The whole metadata map
    pub(crate) fn get_metadata_map(&self) -> &HashMap<String, String> {
        &self.game_metadata
    }
}
