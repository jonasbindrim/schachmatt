use crate::Position;

use super::Ruleset;

pub const CLASSIC_RULESET: Ruleset = Ruleset::new(generate_initial_position);

fn generate_initial_position() -> Position {
    Position::default()
}
