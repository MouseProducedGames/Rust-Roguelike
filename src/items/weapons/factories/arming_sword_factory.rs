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
pub struct ArmingSwordFactory(TemplateWeaponFactory);

impl ArmingSwordFactory {
    pub fn new() -> Self {
        Self {
            0: TemplateWeaponFactory::new(
                Name::new("Arming Sword"),
                Weapon::new(
                    WeaponGroup::Swords,
                    AttackValue::from(3),
                    DamageValue::from(3),
                    DefenceValue::from(3),
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
