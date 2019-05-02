/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.

// Internal includes.
use super::BuildLevel;

pub trait HasBuildLevel {
    fn build_level_total(&self, world: &World) -> BuildLevel;
}
