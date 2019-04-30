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
use super::{LongSpearFactory, MediumSpearFactory, ShortSpearFactory};
use crate::items::weapons::factories::WeaponFactory;

#[derive(Clone)]
pub struct SpearFactory {
    weapon_factories: Arc<Vec<Arc<dyn WeaponFactory>>>,
}

impl SpearFactory {}

impl Default for SpearFactory {
    fn default() -> Self {
        Self {
            weapon_factories: Arc::new(vec![
                Arc::new(ShortSpearFactory::default()),
                Arc::new(MediumSpearFactory::default()),
                Arc::new(LongSpearFactory::default()),
            ]),
        }
    }
}

impl WeaponFactory for SpearFactory {
    fn create(&self, world: &mut World) -> Entity {
        let index = thread_rng().gen_range(0, self.weapon_factories.len());
        self.weapon_factories[index].create(world)
    }
}
