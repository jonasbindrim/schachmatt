use crate::Turn;

use super::Lan;

impl Lan {
    /// Converts a `Turn` into its corresponding LAN representation.
    /// - `turn` - The turn to convert
    /// - `returns` - The LAN representation of the `Turn`-parameter
    #[must_use]
    pub fn export(turn: &Turn) -> String {
        if let Some(promotion) = turn.promotion {
            format!(
                "{}{}{}",
                turn.current,
                turn.target,
                promotion.export_piecetype_lowercase()
            )
        } else {
            format!("{}{}", turn.current, turn.target)
        }
    }
}
