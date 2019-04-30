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
pub struct RapierFactory(TemplateWeaponFactory);

impl RapierFactory {}

impl Default for RapierFactory {
    fn default() -> Self {
        Self {
            0: TemplateWeaponFactory::new(
                Name::new("Rapier"),
                Weapon::new(
                    WeaponGroup::Swords,
                    AttackValue::from(4),
                    DamageType::Piercing,
                    DamageValue::from(1),
                    DefenceValue::from(3),
                ),
            ),
        }
    }
}

impl WeaponFactory for RapierFactory {
    fn create(&self, world: &mut World) -> Entity {
        self.0.create(world)
    }
}
