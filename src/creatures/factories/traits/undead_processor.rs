/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use crate::abilities::Undead;
use crate::creatures::factories::CreatureProcessor;

#[derive(Clone)]
pub struct UndeadProcessor;

impl UndeadProcessor {}

impl Default for UndeadProcessor {
    fn default() -> Self {
        Self {}
    }
}

impl CreatureProcessor for UndeadProcessor {
    fn process(&self, world: &mut World, creature_entity: Entity) -> Entity {
        {
            let mut undead_storage = world.write_storage::<Undead>();
            if undead_storage.get(creature_entity).is_none() {
                if let Err(e) = undead_storage.insert(creature_entity, Default::default()) {
                    panic!(e);
                }
            }
        }

        creature_entity
    }
}
