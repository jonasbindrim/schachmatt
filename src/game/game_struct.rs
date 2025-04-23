use std::collections::HashMap;

use crate::{Position, Turn};

/// Represents a game of chess.
#[derive(Clone)]
pub struct Game {
    pub(super) game_metadata: HashMap<String, String>,
    pub(super) position_history: Vec<Position>,
    pub(super) turn_history: Vec<Turn>,
}
