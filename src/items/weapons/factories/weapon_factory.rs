/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.

// Internal includes.
use crate::items::Item;

pub trait WeaponFactory {
    fn create(&self, world: &mut World) -> Entity;

    fn create_owned(&self, world: &mut World) -> Entity {
        let item = self.create(world);
        world
            .write_storage::<Item>()
            .get_mut(item)
            .unwrap()
            .owned_mut(true);
        item
    }
}
