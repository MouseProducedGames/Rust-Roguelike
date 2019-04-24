/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.

// Internal includes.
use super::{TemplateWeaponFactory, WeaponFactory};
use crate::data_types::Name;
use crate::game::combat::{AttackValue, DamageValue, DefenceValue};
use crate::items::weapons::{Weapon, WeaponGroup};

#[derive(Clone)]
pub struct PalmFactory(TemplateWeaponFactory);

impl PalmFactory {
    pub fn new() -> Self {
        Self {
            0: TemplateWeaponFactory::new(
                Name::new("Palm"),
                Weapon::new(
                    WeaponGroup::Unarmed,
                    AttackValue::from(0),
                    DamageValue::from(0),
                    DefenceValue::from(0),
                ),
            ),
        }
    }
}

impl WeaponFactory for PalmFactory {
    fn create(&self, world: &mut World) -> Entity {
        self.0.create(world)
    }
}
