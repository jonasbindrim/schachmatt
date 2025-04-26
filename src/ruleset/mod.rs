use crate::{GameResult, Position, Turn};

pub(crate) mod classic;

/// Contains function which describe the rules of a chess game
#[derive(Clone)]
pub struct Ruleset {
    generate_initial_position: fn() -> Position,
    get_possible_turns: fn(&Position) -> Vec<Turn>,
    execute_turn: fn(&Position, &Turn) -> Position,
    game_over_check: fn(&Position) -> Option<GameResult>,
}

impl Ruleset {
    pub const fn new(
        generate_initial_position: fn() -> Position,
        get_possible_turns: fn(&Position) -> Vec<Turn>,
        execute_turn: fn(&Position, &Turn) -> Position,
        game_over_check: fn(&Position) -> Option<GameResult>,
    ) -> Self {
        Ruleset {
            generate_initial_position,
            get_possible_turns,
            execute_turn,
            game_over_check,
        }
    }

    pub fn generate_initial_position(&self) -> Position {
        (self.generate_initial_position)()
    }

    pub fn get_possible_turns(&self, position: &Position) -> Vec<Turn> {
        (self.get_possible_turns)(position)
    }

    pub fn execute_turn(&self, position: &Position, turn: &Turn) -> Position {
        (self.execute_turn)(position, turn)
    }

    pub fn game_over_check(&self, position: &Position) -> Option<GameResult> {
        (self.game_over_check)(position)
    }
}
