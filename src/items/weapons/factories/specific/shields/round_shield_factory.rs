/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use super::TemplateShieldFactory;
use crate::data_types::Name;
use crate::game::combat::{AttackValue, DamageValue, DefenceValue};
use crate::items::weapons::factories::WeaponFactory;
use crate::items::weapons::{Weapon, WeaponGroup};

#[derive(Clone)]
pub struct RoundShieldFactory(TemplateShieldFactory);

impl RoundShieldFactory {}

impl Default for RoundShieldFactory {
    fn default() -> Self {
        Self {
            0: TemplateShieldFactory::new(
                Name::new("Round Shield"),
                Weapon::new(
                    WeaponGroup::Shields,
                    AttackValue::from(2),
                    DamageValue::from(1),
                    DefenceValue::from(5),
                ),
            ),
        }
    }
}

impl WeaponFactory for RoundShieldFactory {
    fn create(&self, world: &mut World) -> Entity {
        self.0.create(world)
    }
}
