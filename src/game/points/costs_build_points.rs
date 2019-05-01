/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.

// Internal includes.
use super::BuildPointsValue;

pub trait CostsBuildPoints {
    fn build_points_total(&self, world: &World) -> BuildPointsValue;
}
