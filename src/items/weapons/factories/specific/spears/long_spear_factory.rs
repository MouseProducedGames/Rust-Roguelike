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
pub struct LongSpearFactory(TemplateWeaponFactory);

impl LongSpearFactory {}

impl Default for LongSpearFactory {
    fn default() -> Self {
        Self {
            0: TemplateWeaponFactory::new(
                Name::new("Long Spear"),
                Weapon::new(
                    WeaponGroup::Spears,
                    AttackValue::from(5),
                    DamageType::Piercing,
                    DamageValue::from(3),
                    DefenceValue::from(1),
                ),
            ),
        }
    }
}

impl WeaponFactory for LongSpearFactory {
    fn create(&self, world: &mut World) -> Entity {
        self.0.create(world)
    }
}
