#![doc = include_str!("../README.md")]

extern crate pest;

#[macro_use]
extern crate pest_derive;

mod constants;
pub use crate::constants::columns as Columns;
pub use crate::constants::fields as Fields;
pub use crate::constants::rows as Rows;

mod chess;
pub use crate::chess::castling_rights::CastlingRights;
pub use crate::chess::field::Field;
pub use crate::chess::game::Game;
pub use crate::chess::game_result::GameResult;
pub use crate::chess::piece::Piece;
pub use crate::chess::piece_type::PieceType;
pub use crate::chess::player_color::PlayerColor;
pub use crate::chess::position::Position;
pub use crate::chess::position::PositionError;
pub use crate::chess::turn::Turn;

mod ruleset;
pub use crate::ruleset::Ruleset;
pub use crate::ruleset::classic::CLASSIC_RULESET;

mod util;
pub use crate::util::parser::forsyth_edwards_notation as FEN;
pub use crate::util::parser::long_algebraic_notation as LAN;
pub use crate::util::parser::portable_game_notation as PGN;
pub use crate::util::parser::standard_algebraic_notation as SAN;
