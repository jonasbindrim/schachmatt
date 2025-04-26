use crate::{GameResult, Position, Turn, FEN};
use super::Ruleset;

mod internal;

pub const CLASSIC_RULESET: Ruleset = Ruleset::new(
    generate_initial_position,
    get_possible_turns,
    execute_turn,
    game_over_check,
);

fn generate_initial_position() -> Position {
    FEN::import(FEN::DEFAULT_BOARD_SETUP).unwrap()
}

fn get_possible_turns(position: &Position) -> Vec<Turn> {
    todo!()
}

pub fn execute_turn(position: &Position, turn: &Turn) -> Position {
    todo!()
}

pub fn game_over_check(position: &Position) -> Option<GameResult> {
    todo!()
}