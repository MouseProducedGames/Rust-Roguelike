/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};

// Standard includes.

// Internal includes.
use crate::game::GameValue;

#[derive(Copy, Clone, Default)]
pub struct BuildPointsMarker;

pub type BuildPointsValue = GameValue<BuildPointsMarker>;

impl Component for BuildPointsValue {
    type Storage = VecStorage<Self>;
}
