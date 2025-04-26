use crate::{Position, Turn};

pub(crate) mod classic;

/// Contains function which describe the rules of a chess game
#[derive(Clone)]
pub struct Ruleset {
    generate_initial_position: fn() -> Position,
    get_possible_moves: fn(&Position) -> Vec<Turn>,
}

impl Ruleset {
    pub const fn new(
        generate_initial_position: fn() -> Position,
        get_possible_moves: fn(&Position) -> Vec<Turn>,
    ) -> Self {
        Ruleset {
            generate_initial_position,
            get_possible_moves
        }
    }

    pub fn generate_initial_position(&self) -> Position {
        (self.generate_initial_position)()
    }

    pub fn get_possible_moves(&self, position: &Position) -> Vec<Turn> {
        (self.get_possible_moves)(position)
    }
}
