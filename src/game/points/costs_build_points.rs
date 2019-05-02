/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.

// Internal includes.
use super::{BuildPoints, HasBuildLevel};

pub trait CostsBuildPoints {
    fn build_points_total(&self, world: &World) -> BuildPoints;
}

impl CostsBuildPoints for HasBuildLevel {
    fn build_points_total(&self, world: &World) -> BuildPoints {
        BuildPoints::from(self.build_level_total(world))
    }
}
