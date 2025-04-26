use crate::{Position, Turn};

use super::Ruleset;

pub const CLASSIC_RULESET: Ruleset = Ruleset::new(
    generate_initial_position,
    get_possible_moves
);

fn generate_initial_position() -> Position {
    Position::default()
}

fn get_possible_moves(position: &Position) -> Vec<Turn> {
    todo!()
}