/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External dependencies

// Internal dependencies.
use crate::world::TiledArea;

pub trait DungeonGenerator {
    fn apply(&mut self, area: &mut dyn TiledArea);
}
