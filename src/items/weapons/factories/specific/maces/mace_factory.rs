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
use super::RoundMaceFactory;
use crate::items::weapons::factories::WeaponFactory;

#[derive(Clone)]
pub struct MaceFactory {
    weapon_factories: Arc<Vec<Arc<dyn WeaponFactory>>>,
}

impl MaceFactory {}

impl Default for MaceFactory {
    fn default() -> Self {
        Self {
            weapon_factories: Arc::new(vec![Arc::new(RoundMaceFactory::default())]),
        }
    }
}

impl WeaponFactory for MaceFactory {
    fn create(&self, world: &mut World) -> Entity {
        let index = thread_rng().gen_range(0, self.weapon_factories.len());
        self.weapon_factories[index].create(world)
    }
}
