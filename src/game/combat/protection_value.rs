/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.

// Internal includes.
use crate::game::points::{BuildLevel, HasBuildLevel};
use crate::game::{GameValue, GameValueFixed};

#[derive(Copy, Clone, Default)]
pub struct ProtectionMarker;

pub type ProtectionValue = GameValue<ProtectionMarker>;

impl HasBuildLevel for ProtectionValue {
    fn build_level_total(&self, _world: &World) -> BuildLevel {
        BuildLevel::new(self.raw() - GameValueFixed::from_int(3))
    }
}
