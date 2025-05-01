use crate::{GameResult, PlayerColor, Position, Turn};

pub(crate) mod classic;

/// Contains functions which describe the rules of a chess game
/// Can be used to alter the rules of the game
/// (e.g. for variants like Chess960)
/// Each `Ruleset` has to implement specific functions.
#[derive(Clone)]
pub struct Ruleset {
    generate_initial_position: fn() -> Position,
    get_possible_turns: fn(&Position) -> Vec<Turn>,
    execute_turn: fn(&Position, &Turn) -> Position,
    game_over_check: fn(&Position) -> Option<GameResult>,
    is_in_check: fn(&Position, PlayerColor) -> bool,
}

impl Ruleset {
    pub const fn new(
        generate_initial_position: fn() -> Position,
        get_possible_turns: fn(&Position) -> Vec<Turn>,
        execute_turn: fn(&Position, &Turn) -> Position,
        game_over_check: fn(&Position) -> Option<GameResult>,
        is_in_check: fn(&Position, PlayerColor) -> bool,
    ) -> Self {
        Ruleset {
            generate_initial_position,
            get_possible_turns,
            execute_turn,
            game_over_check,
            is_in_check,
        }
    }

    /// Implementations of this `Ruleset` function must place the initial position of the pieces on the board
    /// and return the initial position.
    pub fn generate_initial_position(&self) -> Position {
        (self.generate_initial_position)()
    }

    /// Implementations of this `Ruleset` function must return a vector of all possible turns
    pub fn get_possible_turns(&self, position: &Position) -> Vec<Turn> {
        (self.get_possible_turns)(position)
    }

    /// Implementations of this `Ruleset` function must execute a turn and return the new position.
    /// For any given position, each `Turn` which is returned by the `get_possible_turns` function
    /// for the same position must be executable by this function.
    pub fn execute_turn(&self, position: &Position, turn: &Turn) -> Position {
        (self.execute_turn)(position, turn)
    }

    /// Implementations of this `Ruleset` function must check if the game is over
    /// and return the result of the game.
    pub fn game_over_check(&self, position: &Position) -> Option<GameResult> {
        (self.game_over_check)(position)
    }

    /// Implementations of this `Ruleset` function must check if the given player is in check
    pub fn is_in_check(&self, position: &Position, player_color: PlayerColor) -> bool {
        (self.is_in_check)(position, player_color)
    }
}
