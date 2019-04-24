/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use super::ArmourProcessor;
use crate::data_types::Name;
use crate::game::combat::{DefenceValue, ProtectionValue};
use crate::items::armours::Armour;

#[derive(Clone)]
pub struct LeatherArmourProcessor;

impl LeatherArmourProcessor {}

impl Default for LeatherArmourProcessor {
    fn default() -> Self {
        Self {}
    }
}

impl ArmourProcessor for LeatherArmourProcessor {
    fn process(&self, world: &mut World, item_entity: Entity) -> Entity {
        {
            let mut storage = world.write_storage::<Armour>();
            let armour = storage.get_mut(item_entity).unwrap();
            *armour.defence_value_mut() += DefenceValue::from(-1);
            *armour.protection_value_mut() += ProtectionValue::from(2);
        }

        {
            let mut storage = world.write_storage::<Name>();
            let name = storage.get_mut(item_entity).unwrap();
            name.insert_str(0, "Leather ");
        }

        item_entity
    }
}
