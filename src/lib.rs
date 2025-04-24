#![doc = include_str!("../README.md")]

extern crate pest;

#[macro_use]
extern crate pest_derive;

mod tests;

mod game;
pub use crate::game::Game;

mod position;
pub use crate::position::position_struct::Position;
pub use crate::position::util::errors::PositionError;

mod field;
pub use crate::field::Field;

mod turn;
pub use crate::turn::Turn;

mod piece;
pub use crate::piece::Piece;
pub use crate::piece::piece_type::PieceType;

mod game_result;
pub use crate::game_result::GameResult;

mod player_color;
pub use crate::player_color::PlayerColor;

mod ruleset;
pub use crate::ruleset::Ruleset;
pub use crate::ruleset::classic::CLASSIC_RULESET;

mod util;
pub use crate::util::board_descriptors as Board;
pub use crate::util::parser::forsyth_edwards_notation as FEN;
pub use crate::util::parser::long_algebraic_notation as LAN;
pub use crate::util::parser::portable_game_notation as PGN;
pub use crate::util::parser::standard_algebraic_notation as SAN;
