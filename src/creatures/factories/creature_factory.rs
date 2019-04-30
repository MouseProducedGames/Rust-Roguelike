/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.

// Internal includes.
use crate::rrl_math::Position;

pub trait CreatureFactory {
    fn create(&self, world: &mut World) -> Entity;

    fn create_with_position(&self, world: &mut World, position: Position) -> Entity {
        let entity = self.create(world);
        let mut position_storage = world.write_storage::<Position>();
        if let Some(set_position) = position_storage.get_mut(entity) {
            *set_position = position;
        } else {
            let result = position_storage.insert(entity, position);
            if let Err(e) = result {
                panic!(e);
            }
        }

        entity
    }
}
