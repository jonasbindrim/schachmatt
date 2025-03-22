use crate::{Game, GameResult, PlayerColor, SAN};

/// Converts a `Game` into its PGN representation.
/// - `game` - The game which is converted
/// - `returns` - A string representing the game in pgn form
#[must_use]
pub fn game_to_pgn(game: &mut Game) -> String {
    let mut result = String::new();

    add_metadata(&mut result, game);
    result.push('\n');
    add_turn_data(&mut result, game);

    result
}

/// Adds the turn data of a game to a string
/// - `result` - The `turn_data` is appended to this string
/// - `game` - The game containing the turns that will be added
fn add_turn_data(result: &mut String, game: &mut Game) {
    let mut first_fullmove_indicator: bool = true;

    for position_index in 0..game.position_history.len() {
        let position = game.get_state_at_index_reference(position_index);

        // Add game result. Only done once in the last position
        if position_index == game.position_history.len() - 1 {
            let game_result: &str;
            match position.game_over_check() {
                GameResult::Draw => game_result = "1/2-1/2",
                GameResult::None => game_result = "*",
                GameResult::Over(player_color) => match player_color {
                    PlayerColor::Black => game_result = "0-1",
                    PlayerColor::White => game_result = "1-0",
                },
            }
            game.set_metadata("Result", game_result);
            result.push_str(game_result);
            break;
        }

        // Print fullmove counter
        match position.active_color {
            PlayerColor::Black => {
                if first_fullmove_indicator {
                    result.push_str(&position.fullmove_counter.to_string());
                    result.push_str(".. ");
                    first_fullmove_indicator = false;
                }
            }
            PlayerColor::White => {
                result.push_str(&position.fullmove_counter.to_string());
                result.push_str(". ");
                first_fullmove_indicator = false;
            }
        }

        // Add turn san data
        let turn = game.get_turn_at_index(position_index);
        result.push_str(&SAN::export(turn, position));
        result.push(' ');
    }
}

/// Adds the metadata of a game to a string
/// - `result` - The metadata is appended to this string
/// - `game` - The game containing the metadata that gets appended
fn add_metadata(result: &mut String, game: &Game) {
    let metadata_map = game.get_metadata_map();
    for key in metadata_map.keys() {
        result.push('[');

        result.push_str(key);
        result.push_str(" \"");
        result.push_str(metadata_map.get(key).unwrap());

        result.push_str("\"]\n");
    }
}
