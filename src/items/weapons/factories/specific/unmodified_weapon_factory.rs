/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use rand::{thread_rng, Rng};
use specs::{Entity, World};

// Standard includes.
use std::default::Default;
use std::sync::Arc;

// Internal includes.
use super::axes::BattleAxeFactory;
use super::maces::RoundMaceFactory;
use super::spears::SpearFactory;
use super::swords::SwordFactory;
use crate::items::weapons::factories::WeaponFactory;

#[derive(Clone)]
pub struct UnmodifiedWeaponFactory {
    weapon_factories: Arc<Vec<Arc<dyn WeaponFactory>>>,
}

impl UnmodifiedWeaponFactory {}

impl Default for UnmodifiedWeaponFactory {
    fn default() -> Self {
        Self {
            weapon_factories: Arc::new(vec![
                Arc::new(BattleAxeFactory::default()),
                Arc::new(RoundMaceFactory::default()),
                Arc::new(SpearFactory::default()),
                Arc::new(SwordFactory::default()),
            ]),
        }
    }
}

impl WeaponFactory for UnmodifiedWeaponFactory {
    fn create(&self, world: &mut World) -> Entity {
        let index = thread_rng().gen_range(0, self.weapon_factories.len());
        self.weapon_factories[index].create(world)
    }
}
