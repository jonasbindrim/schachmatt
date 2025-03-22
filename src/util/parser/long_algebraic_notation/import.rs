use crate::{Field, Turn, data_structures::piece::piece_type::PieceType};

use pest::{Parser, iterators::Pair};

#[derive(Parser)]
#[grammar = "util/pest_definitions/long_algebraic_notation.pest"]
struct LanStruct;

/// Converts a string in LAN into a `Turn` if possible.
/// - `raw` - The raw string in long algebraic notation
/// - `color_at_turn` - The color who played the given turn
/// - `returns` - A turn representing the raw data or none
/// # Panics
/// This panic indicates an error in the library.
#[must_use]
pub fn from_string(raw: &str) -> Option<Turn> {
    let Ok(mut parsed_data) = LanStruct::parse(Rule::turn, raw) else {
        return None;
    };

    if let Some(turn_type) = parsed_data.next().unwrap().into_inner().next() {
        match turn_type.as_rule() {
            Rule::from_to_turn => {
                let (from, to) = handle_from_to_turn_rule(turn_type);
                return Some(Turn {
                    from,
                    to,
                    promotion: None,
                });
            }
            Rule::piece_descriptor_turn => {
                let (from, to) = handle_piece_descriptor_turn_rule(turn_type);
                return Some(Turn {
                    from,
                    to,
                    promotion: None,
                });
            }
            Rule::promotion_turn => {
                return Some(handle_promotion_turn_rule(turn_type));
            }
            _ => unreachable!(),
        }
    }
    unreachable!()
}

/// Converts the `promotion-turn`-Rule into a `Turn`-object
/// - `promotion_turn` - The matching Rule
/// - `returns` - A `Turn`-object
fn handle_promotion_turn_rule(promotion_turn: Pair<Rule>) -> Turn {
    let mut from_field: Option<Field> = None;
    let mut to_field: Option<Field> = None;

    for promotion_turn_descriptor in promotion_turn.into_inner() {
        match promotion_turn_descriptor.as_rule() {
            Rule::from_to_turn => {
                let (temp_from_field, temp_to_field) =
                    handle_from_to_turn_rule(promotion_turn_descriptor);
                from_field = Some(temp_from_field);
                to_field = Some(temp_to_field);
            }
            Rule::piece_descriptor_turn => {
                let (temp_from_field, temp_to_field) =
                    handle_piece_descriptor_turn_rule(promotion_turn_descriptor);
                from_field = Some(temp_from_field);
                to_field = Some(temp_to_field);
            }
            Rule::promotion_piece => {
                let piece_representation = promotion_turn_descriptor.as_str().as_bytes()[0];
                let piece_type = PieceType::import_piecetype(piece_representation as char).unwrap();

                return Turn {
                    from: from_field.unwrap(),
                    to: to_field.unwrap(),
                    promotion: Some(piece_type),
                };
            }
            _ => unreachable!(),
        }
    }
    unreachable!()
}

/// Converts the `piece-descriptor-turn`-Rule into two fields
/// - `piece_descriptor_turn` - The matching Rule
/// - `returns` - The two fields (`from_field`, `to_field`)
fn handle_piece_descriptor_turn_rule(piece_descriptor_turn: Pair<Rule>) -> (Field, Field) {
    for piece_descriptor in piece_descriptor_turn.into_inner() {
        match piece_descriptor.as_rule() {
            Rule::from_to_turn => {
                return handle_from_to_turn_rule(piece_descriptor);
            }
            Rule::piece_symbol => {}
            _ => unreachable!(),
        }
    }
    unreachable!()
}

/// Converts the `from-to-turn`-Rule into two fields
/// - `from_to_turn` - The matching Rule
/// - `returns` - The two fields (`from_field`, `to_field`)
fn handle_from_to_turn_rule(from_to_turn: Pair<Rule>) -> (Field, Field) {
    let mut from_field: Option<Field> = None;

    for field_descriptor in from_to_turn.into_inner() {
        match field_descriptor.as_rule() {
            Rule::field_descriptor => match from_field {
                Some(from) => {
                    return (from, Field::from_string(field_descriptor.as_str()).unwrap());
                }
                None => {
                    from_field = Some(Field::from_string(field_descriptor.as_str()).unwrap());
                }
            },
            _ => unreachable!(),
        }
    }
    unreachable!()
}
