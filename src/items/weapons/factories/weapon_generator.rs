/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.

// Internal includes.
use crate::game::points::BuildLevel;
use crate::items::Item;

pub trait WeaponGenerator {
    fn create(&self, world: &mut World, level: BuildLevel) -> Entity;

    fn create_owned(&self, world: &mut World, level: BuildLevel) -> Entity {
        let item = self.create(world, level);
        world
            .write_storage::<Item>()
            .get_mut(item)
            .unwrap()
            .owned_mut(true);

        item
    }
}
