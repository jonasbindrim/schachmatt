use crate::{
    Position, Turn,
    chess::turn::{CastleDirection, NormalTurn},
};

use super::Lan;

impl Lan {
    /// Converts a `Turn` into its corresponding LAN representation.
    /// - `turn` - The turn to convert
    /// - `position` - The current position of the game, used to determine if a capture has occurred
    /// - `returns` - The LAN representation of the `Turn`-parameter
    #[must_use]
    pub fn export(turn: &Turn, position: &Position) -> String {
        match turn {
            Turn::Normal(normal_turn) => Self::export_normal(normal_turn, position),
            Turn::Castle(castle_direction) => Self::export_castling(castle_direction),
        }
    }

    fn export_normal(normal_turn: &NormalTurn, position: &Position) -> String {
        let mut output = String::new();

        // Add origin field
        output.push_str(&normal_turn.origin.to_string());

        // Add optional capture indicator
        if position.get_field_occupation(&normal_turn.target).is_some()
            || position.get_en_passant() == Some(normal_turn.target)
        {
            output.push('x');
        }

        // Add target field
        output.push_str(&normal_turn.target.to_string());

        // Add optional promotion piece
        if let Some(promotion) = normal_turn.promotion {
            output.push(promotion.export_piecetype_lowercase());
        }

        output
    }

    fn export_castling(castle_direction: &CastleDirection) -> String {
        match castle_direction {
            CastleDirection::Kingside => String::from("O-O"),
            CastleDirection::Queenside => String::from("O-O-O"),
        }
    }
}
