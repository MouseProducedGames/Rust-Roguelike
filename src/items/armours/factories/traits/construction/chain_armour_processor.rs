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
use crate::game::combat::{DefenceValue, ProtectionValue};
use crate::items::armours::factories::{ArmourProcessor, ProcessArmourFactory};
use crate::items::armours::Armour;

#[allow(dead_code)]
pub type ChainArmourFactory<TArmourFactory> =
    ProcessArmourFactory<TArmourFactory, ChainArmourProcessor>;

#[derive(Clone)]
pub struct ChainArmourProcessor;

impl ChainArmourProcessor {}

impl Default for ChainArmourProcessor {
    fn default() -> Self {
        Self {}
    }
}

impl ArmourProcessor for ChainArmourProcessor {
    fn process(&self, world: &mut World, item_entity: Entity) -> Entity {
        {
            let mut storage = world.write_storage::<Armour>();
            let armour = storage.get_mut(item_entity).unwrap();
            *armour.defence_value_mut() += DefenceValue::new(-2);
            *armour.protection_value_mut() += ProtectionValue::new(8);
        }

        {
            let mut storage = world.write_storage::<Name>();
            let name = storage.get_mut(item_entity).unwrap();
            name.insert_str(0, "Chain ");
        }

        item_entity
    }
}
