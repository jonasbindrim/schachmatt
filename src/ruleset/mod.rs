use crate::Position;

pub(crate) mod classic;

/// Contains function which describe the rules of a chess game
#[derive(Clone)]
pub struct Ruleset {
    generate_initial_position: fn() -> Position,
}

impl Ruleset {
    pub const fn new(generate_initial_position: fn() -> Position) -> Self {
        Ruleset {
            generate_initial_position,
        }
    }

    pub fn generate_initial_position(&self) -> Position {
        (self.generate_initial_position)()
    }
}
