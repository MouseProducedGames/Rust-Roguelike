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
use crate::items::weapons::factories::{ProcessWeaponFactory, WeaponProcessor};
use crate::items::weapons::Weapon;

#[allow(dead_code)]
pub type MediumShieldFactory<TWeaponFactory> =
    ProcessWeaponFactory<TWeaponFactory, MediumShieldProcessor>;

#[derive(Clone)]
pub struct MediumShieldProcessor;

impl MediumShieldProcessor {}

impl Default for MediumShieldProcessor {
    fn default() -> Self {
        Self {}
    }
}

impl WeaponProcessor for MediumShieldProcessor {
    fn process(&self, world: &mut World, item_entity: Entity) -> Entity {
        {
            let mut storage = world.write_storage::<Name>();
            let name = storage.get_mut(item_entity).unwrap();
            name.insert_str(0, "Medium ");
        }

        item_entity
    }
}
