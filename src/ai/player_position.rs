use crate::rrl_math::Position;
/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// internal includes
use std::default::Default;

// #[derive(Default)]
pub struct PlayerPosition(pub Position);

impl Default for PlayerPosition {
    fn default() -> Self {
        Self(Position::new(0, 0))
    }
}
