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
use crate::game::combat::{DamageType, DamageValue};
use crate::items::weapons::factories::{ProcessWeaponFactory, WeaponProcessor};
use crate::items::weapons::Weapon;

#[allow(dead_code)]
pub type DamagingWeaponFactory<TWeaponFactory> =
    ProcessWeaponFactory<TWeaponFactory, DamagingWeaponProcessor>;

#[derive(Clone)]
pub struct DamagingWeaponProcessor;

impl DamagingWeaponProcessor {}

impl Default for DamagingWeaponProcessor {
    fn default() -> Self {
        Self {}
    }
}

impl WeaponProcessor for DamagingWeaponProcessor {
    fn process(&self, world: &mut World, item_entity: Entity) -> Entity {
        let damage_type;
        {
            let mut storage = world.write_storage::<Weapon>();
            let weapon = storage.get_mut(item_entity).unwrap();
            *weapon.damage_value_mut() += DamageValue::from(2);
            damage_type = weapon.damage_type();
        }

        {
            let mut storage = world.write_storage::<Name>();
            let name = storage.get_mut(item_entity).unwrap();
            name.push_str(match damage_type {
                DamageType::Blunt => " of Hardness",
                DamageType::Crushing => " of Impact",
                DamageType::Piercing => " of Acuity",
                DamageType::Slashing => " of Sharpness",
            });
        }

        item_entity
    }
}
