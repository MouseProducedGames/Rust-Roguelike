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
pub struct LongSwordFactory(TemplateWeaponFactory);

impl LongSwordFactory {}

impl Default for LongSwordFactory {
    fn default() -> Self {
        Self {
            0: TemplateWeaponFactory::new(
                Name::new("Long Sword"),
                Weapon::new(
                    WeaponGroup::Swords,
                    AttackValue::new(2),
                    DamageType::Slashing,
                    DamageValue::new(7),
                    DefenceValue::new(2),
                ),
            ),
        }
    }
}

impl WeaponFactory for LongSwordFactory {
    fn create(&self, world: &mut World) -> Entity {
        self.0.create(world)
    }
}
