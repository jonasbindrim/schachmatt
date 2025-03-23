#![doc = include_str!("../README.md")]

extern crate pest;

#[macro_use]
extern crate pest_derive;

mod tests;

mod game;
pub use crate::game::game_struct::Game;

mod position;
pub use crate::position::position_struct::Position;

mod data_structures;
pub use crate::data_structures::field::Field;
pub use crate::data_structures::game_result::GameResult;
pub use crate::data_structures::piece::Piece;
pub use crate::data_structures::piece::piece_type::PieceType;
pub use crate::data_structures::player_color::PlayerColor;
pub use crate::data_structures::turn::Turn;

mod util;
pub use crate::util::error::parser_error::ParserError;
pub use crate::util::parser::forsyth_edwards_notation as FEN;
pub use crate::util::parser::long_algebraic_notation as LAN;
pub use crate::util::parser::portable_game_notation as PGN;
pub use crate::util::parser::standard_algebraic_notation as SAN;
