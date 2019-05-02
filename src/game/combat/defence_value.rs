/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.

// Internal includes.
use crate::game::points::{BuildLevel, BuildPoints, CostsBuildPoints, HasBuildLevel};
use crate::game::GameValue;

#[derive(Copy, Clone, Default)]
pub struct DefenceMarker;

pub type DefenceValue = GameValue<DefenceMarker>;

impl CostsBuildPoints for DefenceValue {
    fn build_points_total(&self, world: &World) -> BuildPoints {
        BuildPoints::from(self.build_level_total(world))
    }
}

impl HasBuildLevel for DefenceValue {
    fn build_level_total(&self, _world: &World) -> BuildLevel {
        BuildLevel::from(i32::from(self) * 30)
    }
}
