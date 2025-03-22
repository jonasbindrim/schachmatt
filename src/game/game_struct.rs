use std::collections::HashMap;

use crate::{Position, Turn};

/// Represents a game of chess.
pub struct Game {
    pub(crate) game_metadata: HashMap<String, String>,
    pub(crate) position_history: Vec<Position>,
    pub(crate) turn_history: Vec<Turn>,
}
