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
use crate::game::combat::{AttackValue, DefenceValue};
use crate::items::weapons::factories::{ProcessWeaponFactory, WeaponProcessor};
use crate::items::weapons::Weapon;

#[allow(dead_code)]
pub type SmallShieldFactory<TWeaponFactory> =
    ProcessWeaponFactory<TWeaponFactory, SmallShieldProcessor>;

#[derive(Clone)]
pub struct SmallShieldProcessor;

impl SmallShieldProcessor {}

impl Default for SmallShieldProcessor {
    fn default() -> Self {
        Self {}
    }
}

impl WeaponProcessor for SmallShieldProcessor {
    fn process(&self, world: &mut World, item_entity: Entity) -> Entity {
        {
            let mut storage = world.write_storage::<Weapon>();
            let weapon = storage.get_mut(item_entity).unwrap();
            *weapon.attack_value_mut() += AttackValue::new(1);
            *weapon.defence_value_mut() += DefenceValue::new(-1);
        }

        {
            let mut storage = world.write_storage::<Name>();
            let name = storage.get_mut(item_entity).unwrap();
            name.insert_str(0, "Small ");
        }

        item_entity
    }
}
