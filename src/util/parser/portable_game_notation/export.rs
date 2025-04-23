use std::collections::HashMap;

use crate::{
    Game, GameResult, PlayerColor, SAN,
    util::metadata::{
        METADATA_KEY_BLACK, METADATA_KEY_DATE, METADATA_KEY_EVENT, METADATA_KEY_RESULT,
        METADATA_KEY_ROUND, METADATA_KEY_SITE, METADATA_KEY_WHITE,
    },
};

static REQUIRED_TAGS: [&str; 6] = [
    METADATA_KEY_BLACK,
    METADATA_KEY_WHITE,
    METADATA_KEY_DATE,
    METADATA_KEY_ROUND,
    METADATA_KEY_EVENT,
    METADATA_KEY_SITE,
];

/// Converts a `Game` into its PGN representation.
/// - `game` - The game which is converted
/// - `returns` - A string representing the game in pgn form
#[must_use]
pub fn game_to_pgn(game: &Game) -> String {
    let mut metadata = game.get_metadata_map().clone();
    let game_result = game.get_game_result();

    add_seven_tag_roster(&mut metadata, game_result);

    let metadata = format_metadata(&metadata);
    let turndata = format_turndata(game);
    format!("{}\n{}", metadata, turndata)
}

/// Formats the turn data of the given game into pgn format
/// - `game` - The game containing the turns that will be added
/// - `returns` - The formatted turn data output
fn format_turndata(game: &Game) -> String {
    let mut result = String::new();
    let mut first_fullmove_indicator: bool = true;

    let position_history = game.get_all_positions();

    for (position_index, position) in position_history.iter().enumerate() {
        // Add game result. Only done once in the last position
        if position_index == position_history.len() - 1 {
            let game_result = GameResult::to_string(position.game_over_check().as_ref());
            result.push_str(&game_result);
            break;
        }

        // Print fullmove counter
        match position.get_active_color() {
            PlayerColor::Black => {
                if first_fullmove_indicator {
                    result.push_str(&position.get_fullmove_counter().to_string());
                    result.push_str(".. ");
                    first_fullmove_indicator = false;
                }
            }
            PlayerColor::White => {
                result.push_str(&position.get_fullmove_counter().to_string());
                result.push_str(". ");
                first_fullmove_indicator = false;
            }
        }

        // Add turn san data
        let turn = game.get_turn_at_index(position_index).unwrap();
        result.push_str(&SAN::export(turn, position));
        result.push(' ');
    }

    result
}

/// Formats the metadata of the given game into pgn format
/// - `game` - The game containing the metadata that gets appended
/// - `returns` - The formatted metadata output
fn format_metadata(metadata: &HashMap<String, String>) -> String {
    metadata
        .iter()
        .map(|(key, value)| format!("[{} \"{}\"]\n", key, value))
        .collect::<Vec<String>>()
        .join("")
}

/// Adds the required metadata entries of the game if not set already
fn add_seven_tag_roster(
    metadata_map: &mut HashMap<String, String>,
    game_result: Option<GameResult>,
) {
    for tag in REQUIRED_TAGS {
        if metadata_map.get(tag).is_none() {
            metadata_map.insert(tag.to_string(), "".to_string());
        }
    }

    if metadata_map.get(METADATA_KEY_RESULT).is_some() {
        return;
    }

    let result = GameResult::to_string(game_result.as_ref());
    metadata_map.insert(METADATA_KEY_RESULT.to_string(), result);
}
