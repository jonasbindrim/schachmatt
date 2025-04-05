use pest::{
    Parser,
    iterators::{Pair, Pairs},
};

use crate::{Game, Position, SAN};

use super::error::PgnParserError;

#[derive(Parser)]
#[grammar = "util/pest_definitions/portable_game_notation.pest"]
struct PgnStruct;

/// Converts a string in PGN representation into a `Game`.
/// - `pgn_string` - The string containing pgn data
/// - `callback_option` - A callback function used for custom pgns
/// - `returns` - A game or an error
pub fn game_from_pgn(pgn_string: &str) -> Result<Game, PgnParserError> {
    let pgn_lines: Vec<&str> = pgn_string.lines().collect();
    let mut game = Game::new_empty();

    // Find the index of the line which seperates metadata and gamedata
    let part_seperator = match find_empty_line(&pgn_lines) {
        Some(index) => index,
        None => pgn_lines.len(),
    };

    parse_metadata_lines(&pgn_lines[0..part_seperator], &mut game)?;

    // Handle turn data
    game.push_position(Position::new());
    let mut turn_data = String::new();
    for line in &pgn_lines[part_seperator + 1..] {
        turn_data.push_str(line);
        turn_data.push(' ');
    }

    let Ok(pairs) = PgnStruct::parse(Rule::turn_data, &turn_data) else {
        return Err(PgnParserError::InvalidTurnData(turn_data));
    };

    let turn_data_pair = pairs.collect::<Vec<Pair<Rule>>>()[0].clone();
    for pair in turn_data_pair.into_inner() {
        if matches!(
            pair.as_rule(),
            Rule::single_move_entry | Rule::two_move_entry
        ) {
            handle_move_entry(pair.into_inner(), &mut game)?
        }
    }

    Ok(game)
}

/// Handles the parsing of a single move entry
fn handle_move_entry(pairs: Pairs<Rule>, game: &mut Game) -> Result<(), PgnParserError> {
    for pair in pairs {
        if pair.as_rule() == Rule::turn_move {
            let turn_rule = pair
                .into_inner()
                .find(|pair| pair.as_rule() == Rule::san_move)
                .unwrap();

            let Ok(turn) = SAN::import(turn_rule.as_str(), &mut game.get_current_state()) else {
                return Err(PgnParserError::IllegalTurn(turn_rule.as_str().to_string()));
            };
            if game.execute_turn(turn).is_err() {
                return Err(PgnParserError::IllegalTurn(turn_rule.as_str().to_string()));
            }
        }
    }

    Ok(())
}

/// Finds and returns the index of the first empty line.
/// The first empty line in a pgn is the seperator between the metadata and the turn data
/// - `lines` - The pgn data lines
/// - `returns` - The line index of the first empty line
fn find_empty_line(lines: &[&str]) -> Option<usize> {
    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            return Some(index);
        }
    }
    None
}

/// Converts each given line into a key-value pair and adds it as metadata to the game
/// - `lines` - The metadata lines of a pgn string
/// - `game` - The game to which the metadata is appended
fn parse_metadata_lines(lines: &[&str], game: &mut Game) -> Result<(), PgnParserError> {
    for line in lines {
        // Handle a single line
        let mut key = String::new();
        match PgnStruct::parse(Rule::metadata, line) {
            Ok(pairs) => {
                for rules in pairs.collect::<Vec<Pair<Rule>>>()[0].clone().into_inner() {
                    match rules.as_rule() {
                        Rule::metadata_key => key = rules.as_str().to_string(),
                        Rule::metadata_value => game.set_metadata(&key, rules.as_str()),
                        _ => return Err(PgnParserError::InvalidMetadataContent(line.to_string())),
                    }
                }
            }
            Err(_) => return Err(PgnParserError::InvalidMetadataContent(line.to_string())),
        }
    }
    Ok(())
}
