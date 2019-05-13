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
pub struct BattleAxeFactory(TemplateWeaponFactory);

impl BattleAxeFactory {}

impl Default for BattleAxeFactory {
    fn default() -> Self {
        Self {
            0: TemplateWeaponFactory::new(
                Name::new("Battle Axe"),
                Weapon::new(
                    WeaponGroup::Axes,
                    AttackValue::from(1),
                    DamageType::Slashing,
                    DamageValue::from(6),
                    DefenceValue::from(2),
                ),
            ),
        }
    }
}

impl WeaponFactory for BattleAxeFactory {
    fn create(&self, world: &mut World) -> Entity {
        self.0.create(world)
    }
}
