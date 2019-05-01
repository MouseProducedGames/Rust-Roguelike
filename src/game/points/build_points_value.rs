/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};

// Standard includes.
use std::fmt;

// Internal includes.
use crate::game::GameValue;

#[derive(Copy, Clone, Default)]
pub struct BuildPointsMarker;

pub type BuildPointsValue = GameValue<BuildPointsMarker>;

impl Component for BuildPointsValue {
    type Storage = VecStorage<Self>;
}

impl fmt::Display for BuildPointsValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", i32::from(self))
    }
}
