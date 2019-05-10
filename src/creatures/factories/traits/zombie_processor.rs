/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use crate::creatures::factories::traits::UndeadProcessor;
use crate::creatures::factories::CreatureProcessor;
use crate::stats::CreatureStats;

#[derive(Clone)]
pub struct ZombieProcessor(UndeadProcessor);

impl ZombieProcessor {
    fn apply_undead(&self, world: &mut World, creature_entity: Entity) -> Entity {
        self.0.process(world, creature_entity)
    }
}

impl Default for ZombieProcessor {
    fn default() -> Self {
        Self {
            0: UndeadProcessor::default(),
        }
    }
}

impl CreatureProcessor for ZombieProcessor {
    fn process(&self, world: &mut World, creature_entity: Entity) -> Entity {
        let creature_entity = self.apply_undead(world, creature_entity);

        *world
            .write_storage::<CreatureStats>()
            .get_mut(creature_entity)
            .unwrap() += CreatureStats::new(8, -4, -4, 4, -4, 4);

        creature_entity
    }
}
