/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.

// Internal includes.

pub trait WeaponProcessor {
    fn process(&self, world: &mut World, item_entity: Entity) -> Entity;
}
