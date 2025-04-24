use std::collections::HashMap;

use crate::{Position, Turn, ruleset::Ruleset};

/// Represents a game of chess.
#[derive(Clone)]
pub struct Game {
    pub(super) game_metadata: HashMap<String, String>,
    pub(super) position_history: Vec<Position>,
    pub(super) turn_history: Vec<Turn>,
    pub(super) ruleset: Ruleset,
}
