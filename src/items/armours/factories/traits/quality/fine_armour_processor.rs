/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use crate::data_types::Name;
use crate::game::combat::DefenceValue;
use crate::items::armours::factories::{ArmourProcessor, ProcessArmourFactory};
use crate::items::armours::Armour;

#[allow(dead_code)]
pub type FineArmourFactory<TArmourFactory> =
    ProcessArmourFactory<TArmourFactory, FineArmourProcessor>;

#[derive(Clone)]
pub struct FineArmourProcessor;

impl FineArmourProcessor {}

impl Default for FineArmourProcessor {
    fn default() -> Self {
        Self {}
    }
}

impl ArmourProcessor for FineArmourProcessor {
    fn process(&self, world: &mut World, item_entity: Entity) -> Entity {
        {
            let mut storage = world.write_storage::<Armour>();
            let armour = storage.get_mut(item_entity).unwrap();
            *armour.defence_value_mut() += DefenceValue::new(1);
        }

        {
            let mut storage = world.write_storage::<Name>();
            let name = storage.get_mut(item_entity).unwrap();
            name.insert_str(0, "Fine ");
        }

        item_entity
    }
}
