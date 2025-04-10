use crate::Turn;

/// Converts a `Turn` into its corresponding LAN representation.
/// - `turn` - The turn to convert
/// - `returns` - The LAN representation of the `Turn`-parameter
/// # Panics
/// This panic indicates an error in the library.
#[must_use]
pub fn from_turn(turn: &Turn) -> String {
    let turn_content: String = if let Some(promotion) = turn.promotion {
        format!(
            "{}{}{}",
            turn.current,
            turn.target,
            promotion.export_piecetype_lowercase()
        )
    } else {
        format!("{}{}", turn.current, turn.target)
    };

    turn_content
}
