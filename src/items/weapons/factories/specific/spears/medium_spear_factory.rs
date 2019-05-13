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
pub struct MediumSpearFactory(TemplateWeaponFactory);

impl MediumSpearFactory {}

impl Default for MediumSpearFactory {
    fn default() -> Self {
        Self {
            0: TemplateWeaponFactory::new(
                Name::new("Medium Spear"),
                Weapon::new(
                    WeaponGroup::Spears,
                    AttackValue::from(4),
                    DamageType::Piercing,
                    DamageValue::from(3),
                    DefenceValue::from(2),
                ),
            ),
        }
    }
}

impl WeaponFactory for MediumSpearFactory {
    fn create(&self, world: &mut World) -> Entity {
        self.0.create(world)
    }
}
