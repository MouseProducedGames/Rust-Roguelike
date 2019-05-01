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
use crate::game::combat::ProtectionValue;
use crate::game::points::{BuildPointsValue, CostsBuildPoints};
use crate::items::armours::factories::{ArmourProcessor, ProcessArmourFactory};
use crate::items::armours::Armour;

#[allow(dead_code)]
pub type RustyArmourFactory<TArmourFactory> =
    ProcessArmourFactory<TArmourFactory, RustyArmourProcessor>;

#[derive(Clone)]
pub struct RustyArmourProcessor;

impl RustyArmourProcessor {}

impl Default for RustyArmourProcessor {
    fn default() -> Self {
        Self {}
    }
}

impl ArmourProcessor for RustyArmourProcessor {
    fn process(&self, world: &mut World, item_entity: Entity) -> Entity {
        {
            let mut storage = world.write_storage::<Armour>();
            let armour = storage.get_mut(item_entity).unwrap();
            *armour.protection_value_mut() += ProtectionValue::from(-2);
        }

        {
            let mut storage = world.write_storage::<BuildPointsValue>();
            let build_points_value = storage.get_mut(item_entity).unwrap();
            *build_points_value += ProtectionValue::from(-2).build_points_total(world);
        }

        {
            let mut storage = world.write_storage::<Name>();
            let name = storage.get_mut(item_entity).unwrap();
            name.insert_str(0, "Rusty ");
        }

        item_entity
    }
}
