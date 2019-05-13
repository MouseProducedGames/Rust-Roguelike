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
pub struct RoundMaceFactory(TemplateWeaponFactory);

impl RoundMaceFactory {}

impl Default for RoundMaceFactory {
    fn default() -> Self {
        Self {
            0: TemplateWeaponFactory::new(
                Name::new("Mace"),
                Weapon::new(
                    WeaponGroup::Maces,
                    AttackValue::new(1),
                    DamageType::Crushing,
                    DamageValue::new(7),
                    DefenceValue::new(1),
                ),
            ),
        }
    }
}

impl WeaponFactory for RoundMaceFactory {
    fn create(&self, world: &mut World) -> Entity {
        self.0.create(world)
    }
}
