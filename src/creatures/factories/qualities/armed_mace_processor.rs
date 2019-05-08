/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use crate::bodies::Body;
use crate::creatures::factories::LeveledCreatureProcessor;
use crate::game::points::BuildLevel;
use crate::items::weapons::factories::specific::maces::LeveledMaceGenerator;
use crate::items::weapons::factories::WeaponGenerator;

#[derive(Clone)]
pub struct ArmedMaceProcessor;

impl ArmedMaceProcessor {}

impl Default for ArmedMaceProcessor {
    fn default() -> Self {
        Self {}
    }
}

impl LeveledCreatureProcessor for ArmedMaceProcessor {
    fn process(&self, world: &mut World, creature_entity: Entity, level: BuildLevel) -> Entity {
        {
            let weapon_entity = LeveledMaceGenerator::default().create(world, level);

            let mut body_storage = world.write_storage::<Body>();
            let body = body_storage.get_mut(creature_entity).unwrap();
            let mut body_data = body.get();
            let body_slot = body_data.get_mut("Right Palm").unwrap();
            body_slot.hold_item(weapon_entity);
        }

        creature_entity
    }
}
