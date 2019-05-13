/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::fmt;

// Internal includes.
use crate::game::points::{BuildLevel, BuildPoints, CostsBuildPoints, HasBuildLevel};
use crate::game::GameValue;

#[derive(Copy, Clone, Default)]
pub struct SkillMarker;

pub type SkillValue = GameValue<SkillMarker>;

impl CostsBuildPoints for SkillValue {
    fn build_points_total(&self, world: &World) -> BuildPoints {
        BuildPoints::from(self.build_level_total(world))
    }
}

impl fmt::Display for SkillValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.raw())
    }
}

impl HasBuildLevel for SkillValue {
    fn build_level_total(&self, _world: &World) -> BuildLevel {
        BuildLevel::new((self.raw() * 10) + 30)
    }
}
