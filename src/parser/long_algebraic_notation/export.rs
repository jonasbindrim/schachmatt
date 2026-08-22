use crate::{
    Turn,
    chess::turn::{CastleDirection, NormalTurn},
};

use super::Lan;

impl Lan {
    /// Converts a `Turn` into its corresponding LAN representation.
    /// - `turn` - The turn to convert
    /// - `returns` - The LAN representation of the `Turn`-parameter
    #[must_use]
    pub fn export(turn: &Turn) -> String {
        match turn {
            Turn::Normal(normal_turn) => Self::export_normal(normal_turn),
            Turn::Castle(castle_direction) => Self::export_castling(castle_direction),
        }
    }

    fn export_normal(normal_turn: &NormalTurn) -> String {
        if let Some(promotion) = normal_turn.promotion {
            format!(
                "{}{}{}",
                normal_turn.origin,
                normal_turn.target,
                promotion.export_piecetype_lowercase()
            )
        } else {
            format!("{}{}", normal_turn.origin, normal_turn.target)
        }
    }

    fn export_castling(castle_direction: &CastleDirection) -> String {
        match castle_direction {
            CastleDirection::Kingside => String::from("O-O"),
            CastleDirection::Queenside => String::from("O-O-O"),
        }
    }
}
