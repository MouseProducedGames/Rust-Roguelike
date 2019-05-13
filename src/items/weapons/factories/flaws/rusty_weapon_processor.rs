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
use crate::game::combat::DamageValue;
use crate::items::weapons::factories::{ProcessWeaponFactory, WeaponProcessor};
use crate::items::weapons::Weapon;

#[allow(dead_code)]
pub type RustyWeaponFactory<TWeaponFactory> =
    ProcessWeaponFactory<TWeaponFactory, RustyWeaponProcessor>;

#[derive(Clone)]
pub struct RustyWeaponProcessor;

impl RustyWeaponProcessor {}

impl Default for RustyWeaponProcessor {
    fn default() -> Self {
        Self {}
    }
}

impl WeaponProcessor for RustyWeaponProcessor {
    fn process(&self, world: &mut World, item_entity: Entity) -> Entity {
        {
            let mut storage = world.write_storage::<Weapon>();
            let weapon = storage.get_mut(item_entity).unwrap();
            *weapon.damage_value_mut() += DamageValue::new(-2);
        }

        {
            let mut storage = world.write_storage::<Name>();
            let name = storage.get_mut(item_entity).unwrap();
            name.insert_str(0, "Rusty ");
        }

        item_entity
    }
}
