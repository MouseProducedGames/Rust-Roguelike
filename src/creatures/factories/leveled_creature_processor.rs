/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.

// Internal includes.
use crate::game::points::BuildLevel;

pub trait LeveledCreatureProcessor {
    fn process(&self, world: &mut World, creature_entity: Entity, level: BuildLevel) -> Entity;
}
