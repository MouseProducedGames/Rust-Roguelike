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
pub struct BastardSwordFactory(TemplateWeaponFactory);

impl BastardSwordFactory {}

impl Default for BastardSwordFactory {
    fn default() -> Self {
        Self {
            0: TemplateWeaponFactory::new(
                Name::new("Bastard Sword"),
                Weapon::new(
                    WeaponGroup::Swords,
                    AttackValue::from(3),
                    DamageType::Slashing,
                    DamageValue::from(5),
                    DefenceValue::from(2),
                ),
            ),
        }
    }
}

impl WeaponFactory for BastardSwordFactory {
    fn create(&self, world: &mut World) -> Entity {
        self.0.create(world)
    }
}
