use crate::{GameResult, PlayerColor, Position, Turn, FEN};
use super::Ruleset;

mod tests;
mod internal;

pub const CLASSIC_RULESET: Ruleset = Ruleset::new(
    generate_initial_position,
    get_possible_turns,
    execute_turn,
    game_over_check,
    is_in_check
);

fn generate_initial_position() -> Position {
    FEN::import(FEN::DEFAULT_BOARD_SETUP).unwrap()
}

fn get_possible_turns(position: &Position) -> Vec<Turn> {
    internal::get_possible_turns(position)
}

pub fn execute_turn(position: &Position, turn: &Turn) -> Position {
    internal::internal_turn(position, turn)
}

pub fn game_over_check(position: &Position) -> Option<GameResult> {
    internal::game_over_check(position)
}

pub fn is_in_check(position: &Position, player_color: PlayerColor) -> bool {
    internal::is_in_check(position, player_color)
}