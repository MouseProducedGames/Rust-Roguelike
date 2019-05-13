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
use crate::game::combat::{AttackValue, DamageType, DamageValue, DefenceValue};
use crate::items::weapons::factories::{TemplateWeaponFactory, WeaponFactory};
use crate::items::weapons::{Weapon, WeaponGroup};

#[derive(Clone)]
pub struct ShortSpearFactory(TemplateWeaponFactory);

impl ShortSpearFactory {}

impl Default for ShortSpearFactory {
    fn default() -> Self {
        Self {
            0: TemplateWeaponFactory::new(
                Name::new("Short Spear"),
                Weapon::new(
                    WeaponGroup::Spears,
                    AttackValue::from(3),
                    DamageType::Piercing,
                    DamageValue::from(3),
                    DefenceValue::from(3),
                ),
            ),
        }
    }
}

impl WeaponFactory for ShortSpearFactory {
    fn create(&self, world: &mut World) -> Entity {
        self.0.create(world)
    }
}
