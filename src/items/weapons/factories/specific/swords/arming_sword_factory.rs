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
pub struct ArmingSwordFactory(TemplateWeaponFactory);

impl ArmingSwordFactory {}

impl Default for ArmingSwordFactory {
    fn default() -> Self {
        Self {
            0: TemplateWeaponFactory::new(
                Name::new("Arming Sword"),
                Weapon::new(
                    WeaponGroup::Swords,
                    AttackValue::new(3),
                    DamageType::Slashing,
                    DamageValue::new(3),
                    DefenceValue::new(3),
                ),
            ),
        }
    }
}

impl WeaponFactory for ArmingSwordFactory {
    fn create(&self, world: &mut World) -> Entity {
        self.0.create(world)
    }
}
