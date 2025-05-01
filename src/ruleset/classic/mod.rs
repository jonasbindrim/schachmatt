use super::Ruleset;
use crate::{DEFAULT_BOARD_SETUP, Fen, GameResult, PlayerColor, Position, Turn};

mod internal;
mod tests;
mod util;

/// The classic ruleset for chess
/// This ruleset is used by default and implements the standard rules of chess.
pub const CLASSIC_RULESET: Ruleset = Ruleset::new(
    generate_initial_position,
    get_possible_turns,
    execute_turn,
    game_over_check,
    is_in_check,
);

fn generate_initial_position() -> Position {
    Fen::import(DEFAULT_BOARD_SETUP).unwrap()
}

fn get_possible_turns(position: &Position) -> Vec<Turn> {
    internal::get_possible_turns(position)
}

fn execute_turn(position: &Position, turn: &Turn) -> Position {
    internal::internal_turn(position, turn)
}

fn game_over_check(position: &Position) -> Option<GameResult> {
    internal::game_over_check(position)
}

fn is_in_check(position: &Position, player_color: PlayerColor) -> bool {
    internal::is_in_check(position, player_color)
}
