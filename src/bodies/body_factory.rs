/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.

// Internal includes.
use super::Body;

pub trait BodyFactory {
    fn create_body(&self, world: &mut World) -> Body;
}
