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
pub struct BattleAxeFactory(TemplateWeaponFactory);

impl BattleAxeFactory {
    pub fn new() -> Self {
        Self {
            0: TemplateWeaponFactory::new(
                Name::new("Battle Axe"),
                Weapon::new(
                    WeaponGroup::Swords,
                    AttackValue::from(1),
                    DamageValue::from(7),
                    DefenceValue::from(1),
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
